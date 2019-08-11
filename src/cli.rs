use std::{
    cmp, env,
    fmt::Write,
    fs::File,
    io::{BufRead, BufReader, Read},
    iter::FromIterator,
    path::{Path, PathBuf},
    str::FromStr,
    string::ToString,
};

use crate::RequestedVersion;

pub enum Action {
    Help(String, PathBuf),
    List(String),
    Execute {
        launcher_path: PathBuf,
        executable: PathBuf,
        args: Vec<String>,
    },
}

impl Action {
    pub fn from_main(argv: &[String]) -> crate::Result<Self> {
        let mut args = argv.to_owned();
        let mut requested_version = RequestedVersion::Any;
        let launcher_path = PathBuf::from(args.remove(0)); // Strip the path to this executable.

        if !args.is_empty() {
            let flag = &args[0];

            if flag == "-h" || flag == "--help" {
                if let Some(executable_path) = crate::find_executable(RequestedVersion::Any) {
                    return Ok(Action::Help(
                        help_message(&launcher_path, &executable_path),
                        executable_path,
                    ));
                } else {
                    return Err(crate::Error::NoExecutableFound(RequestedVersion::Any));
                }
            } else if flag == "--list" {
                return match list_executables() {
                    Ok(list) => Ok(Action::List(list)),
                    Err(message) => Err(message),
                };
            } else if let Some(version) = version_from_flag(&flag) {
                args.remove(0);
                requested_version = version;
            }
        }

        match find_executable(requested_version, &args) {
            Ok(executable) => Ok(Action::Execute {
                launcher_path,
                executable,
                args,
            }),
            Err(message) => Err(message),
        }
    }
}

fn help_message(launcher_path: &Path, executable_path: &Path) -> String {
    let mut message = String::new();
    writeln!(
        message,
        include_str!("HELP.txt"),
        env!("CARGO_PKG_VERSION"),
        launcher_path.to_string_lossy(),
        executable_path.to_string_lossy()
    )
    .unwrap();
    message
}

/// Attempts to find a version specifier from a CLI argument.
///
/// It is assumed that the flag from the command-line is passed as-is
/// (i.e. the flag starts with `-`).
fn version_from_flag(arg: &str) -> Option<RequestedVersion> {
    if !arg.starts_with('-') {
        None
    } else {
        RequestedVersion::from_str(&arg[1..]).ok()
    }
}

// XXX Factor out `all_executables()` to ease testing.
fn list_executables() -> crate::Result<String> {
    let executables = crate::all_executables();

    if executables.is_empty() {
        return Err(crate::Error::NoExecutableFound(RequestedVersion::Any));
    }

    let mut executable_pairs = Vec::from_iter(executables);
    executable_pairs.sort_unstable();

    let max_version_length = executable_pairs.iter().fold(0, |max_so_far, pair| {
        cmp::max(max_so_far, pair.0.to_string().len())
    });

    let left_column_width = cmp::max(max_version_length, "Version".len());
    let mut help_string = String::new();
    // Including two spaces between columns for readability.
    writeln!(help_string, "{:<1$}  Path", "Version", left_column_width).unwrap();
    writeln!(help_string, "{:<1$}  ====", "=======", left_column_width).unwrap();

    for (version, path) in executable_pairs {
        writeln!(
            help_string,
            "{:<2$}  {}",
            version.to_string(),
            path.to_string_lossy(),
            left_column_width
        )
        .unwrap();
    }

    Ok(help_string)
}

// XXX Expose publicly?
// XXX Factor out `VIRTUAL_ENV` call.
/// Returns the path to the activated virtual environment's executable.
///
/// A virtual environment is determined to be activated based on the existence of the `VIRTUAL_ENV`
/// environment variable.
fn venv_executable() -> Option<PathBuf> {
    match env::var_os("VIRTUAL_ENV") {
        None => None,
        Some(venv_root) => {
            let mut path = PathBuf::new();
            path.push(venv_root);
            path.push("bin");
            path.push("python");
            // TODO: Do a is_file() check first?
            Some(path)
        }
    }
}

// XXX Expose publicly?
// https://en.m.wikipedia.org/wiki/Shebang_(Unix)
fn parse_python_shebang(reader: &mut impl Read) -> Option<RequestedVersion> {
    let mut shebang_buffer = [0; 2];
    if reader.read(&mut shebang_buffer).is_err() || shebang_buffer != [0x23, 0x21] {
        // Doesn't start w/ `#!` in ASCII/UTF-8.
        return None;
    }

    let mut buffered_reader = BufReader::new(reader);
    let mut first_line = String::new();

    if buffered_reader.read_line(&mut first_line).is_err() {
        return None;
    };

    // Whitespace between `#!` and the path is allowed.
    let line = first_line.trim();

    let accepted_paths = [
        "python",
        "/usr/bin/python",
        "/usr/local/bin/python",
        "/usr/bin/env python",
    ];

    for acceptable_path in &accepted_paths {
        if !line.starts_with(acceptable_path) {
            continue;
        }

        return match RequestedVersion::from_str(&acceptable_path[acceptable_path.len()..]) {
            Ok(version) => Some(version),
            Err(_) => None,
        };
    }

    None
}

// XXX Expose publicly?
fn find_executable(version: RequestedVersion, args: &[String]) -> crate::Result<PathBuf> {
    let mut requested_version = version;
    let mut chosen_path: Option<PathBuf> = None;

    if requested_version == RequestedVersion::Any {
        if let venv_executable @ Some(..) = venv_executable() {
            chosen_path = venv_executable;
        } else if !args.is_empty() {
            // Using the first argument because it's the simplest and sanest.
            // We can't use the last argument because that could actually be an argument to the
            // Python module being executed. This is the same reason we can't go searching for
            // the first/last file path that we find. The only safe way to get the file path
            // regardless of its position is to replicate Python's arg parsing and that's a
            // **lot** of work for little gain. Hence we only care about the first argument.
            if let Ok(mut open_file) = File::open(&args[0]) {
                if let Some(shebang_version) = parse_python_shebang(&mut open_file) {
                    requested_version = shebang_version;
                }
            }
        }
    }

    if chosen_path.is_none() {
        if let Some(env_var) = requested_version.env_var() {
            if let Ok(env_var_value) = env::var(env_var) {
                if !env_var_value.is_empty() {
                    if let Ok(env_requested_version) = RequestedVersion::from_str(&env_var_value) {
                        requested_version = env_requested_version;
                    }
                }
            };
        }

        if let Some(executable_path) = crate::find_executable(requested_version) {
            chosen_path = Some(executable_path);
        }
    }

    match chosen_path {
        Some(path) => Ok(path),
        None => Err(crate::Error::NoExecutableFound(requested_version)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_from_flag() {
        assert!(version_from_flag(&"-S".to_string()).is_none());
        assert!(version_from_flag(&"--something".to_string()).is_none());
        assert_eq!(
            version_from_flag(&"-3".to_string()),
            Some(RequestedVersion::MajorOnly(3))
        );
        assert_eq!(
            version_from_flag(&"-3.6".to_string()),
            Some(RequestedVersion::Exact(3, 6))
        );
        assert_eq!(
            version_from_flag(&"-42.13".to_string()),
            Some(RequestedVersion::Exact(42, 13))
        );
        assert!(version_from_flag(&"-3.6.4".to_string()).is_none());
    }

    #[test]
    fn test_help_message() {
        let launcher_path = "/some/path/to/launcher";
        let python_path = "/a/path/to/python";

        let help = help_message(&PathBuf::from(launcher_path), &PathBuf::from(python_path));
        assert!(help.contains(env!("CARGO_PKG_VERSION")));
        assert!(help.contains(launcher_path));
        assert!(help.contains(python_path));
    }

    // XXX Test list_executables()

    // XXX Test venv_executable()

    // XXX Test parse_python_shebang()

    // XXX Test find_executable()

    // XXX Test Action::from_main()
}
