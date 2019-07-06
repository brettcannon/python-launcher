use std::{
    cmp,
    fmt::Write,
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
    pub fn from_main(argv: &[String]) -> Result<Self, String> {
        let mut args = argv.to_owned();
        let mut requested_version = RequestedVersion::Any;
        let launcher_path = PathBuf::from(args.remove(0)); // Strip the path to this executable.

        if !args.is_empty() {
            let flag = &args[0];

            if flag == "-h" || flag == "--help" {
                return match help(&launcher_path) {
                    Ok((message, executable_path)) => Ok(Action::Help(message, executable_path)),
                    Err(message) => Err(message),
                };
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

        let directories = crate::path_entries();

        match crate::find_executable(requested_version, directories.into_iter()) {
            Some(executable) => Ok(Action::Execute {
                launcher_path,
                executable,
                args,
            }),
            None => Err("no Python executable found".to_string()),
        }
    }
}

fn help(launcher_path: &Path) -> Result<(String, PathBuf), String> {
    let mut message = String::new();
    let directories = crate::path_entries();

    if let Some(found_path) = crate::find_executable(RequestedVersion::Any, directories.into_iter())
    {
        writeln!(
            message,
            include_str!("HELP.txt"),
            env!("CARGO_PKG_VERSION"),
            launcher_path.to_string_lossy(),
            found_path.to_string_lossy()
        )
        .unwrap();
        return Ok((message, found_path));
    } else {
        return Err("no Python executable found".to_string());
    }
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

fn list_executables() -> Result<String, String> {
    let paths = crate::path_entries();
    let executables = crate::all_executables(paths.into_iter());

    if executables.is_empty() {
        return Err("No Python executable found".to_string());
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
}
