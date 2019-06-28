use std::{
    env,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
    str::FromStr,
    string::ToString,
};

use crate::version::RequestedVersion;

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Help(PathBuf),
    List,
    Execute {
        launcher_path: PathBuf,
        version: RequestedVersion,
        args: Vec<String>,
    },
}

/// Figure out what action is being requested based on the arguments to the executable.
///
/// # Examples
/// Typically you will construct a `Vec<String>` from `env::args()`.
/// ```
/// use std::env;
/// use python_launcher as py;
/// let args = env::args().collect::<Vec<String>>();
/// let action = py::cli::action_from_args(args);
/// ```
pub fn action_from_args(mut args: Vec<String>) -> Action {
    let launcher_path = PathBuf::from(args.remove(0)); // Strip the path to this executable.
    if !args.is_empty() {
        let flag = &args[0];

        if flag == "-h" || flag == "--help" {
            return Action::Help(launcher_path);
        } else if flag == "--list" {
            return Action::List;
        } else if let Some(version) = version_from_flag(&flag) {
            args.remove(0);
            return Action::Execute {
                launcher_path,
                version,
                args,
            };
        }
    }
    Action::Execute {
        launcher_path,
        version: RequestedVersion::Any,
        args,
    }
}

/// Attempts to find a version specifier from a CLI argument.
///
/// It is assumed that the flag from the command-line is passed as-is
/// (i.e. the flag starts with `-`).
pub fn version_from_flag(arg: &str) -> Option<RequestedVersion> {
    if !arg.starts_with('-') {
        None
    } else {
        RequestedVersion::from_str(&arg[1..]).ok()
    }
}

/// Returns the path to the activated virtual environment.
///
/// A virtual environment is determined to be activated based on the existence of the `VIRTUAL_ENV`
/// environment variable.
pub fn virtual_env() -> Option<PathBuf> {
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

// XXX Shebang:
//      In main():
//          Prepend extra arguments to `argv`
//          Continue search for an appropriate Python version

/// Finds the shebang line from `reader`.
///
/// If a shebang line is found, then the `#!` is removed and the line is stripped of leading and trailing whitespace.
pub fn find_shebang(reader: impl Read) -> Option<String> {
    let mut buffered_reader = BufReader::new(reader);

    let mut line = String::new();
    if buffered_reader.read_line(&mut line).is_err() {
        return None;
    };

    if !line.starts_with("#!") {
        None
    } else {
        Some(line[2..].trim().to_string())
    }
}

/// Split the shebang into the Python version specified and the arguments to pass to the executable.
///
/// `Some` is only returned if the specified executable is one of:
/// - `/usr/bin/python`
/// - `/usr/local/bin/python`
/// - `/usr/bin/env python`
/// - `python`
pub fn split_shebang(shebang_line: &str) -> Option<(RequestedVersion, Vec<String>)> {
    let accepted_paths = [
        "/usr/bin/python",
        "/usr/local/bin/python",
        "/usr/bin/env python",
        "python",
    ];

    for exec_path in &accepted_paths {
        if !shebang_line.starts_with(exec_path) {
            continue;
        }

        let trimmed_shebang = shebang_line[exec_path.len()..].to_string();
        let version_string: String = trimmed_shebang
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        let specified_version = if version_string.is_empty() {
            Ok(RequestedVersion::MajorOnly(2))
        } else {
            RequestedVersion::from_str(&version_string)
        };

        return specified_version
            .map(|version| {
                let args = trimmed_shebang[version_string.len()..].trim();
                (
                    version,
                    args.split_whitespace().map(|s| s.to_string()).collect(),
                )
            })
            .ok();
    }

    None
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
    fn test_action_from_args() {
        let launcher_string = String::from("/py");
        let launcher_path = PathBuf::from(&launcher_string);

        assert_eq!(
            action_from_args(vec![launcher_string.clone()]),
            Action::Execute {
                launcher_path: launcher_path.clone(),
                version: RequestedVersion::Any,
                args: Vec::new(),
            }
        );

        assert_eq!(
            action_from_args(vec![launcher_string.clone(), String::from("-h")]),
            Action::Help(launcher_path.clone())
        );

        assert_eq!(
            action_from_args(vec![launcher_string.clone(), String::from("--help")]),
            Action::Help(launcher_path.clone())
        );

        assert_eq!(
            action_from_args(vec![launcher_string.clone(), String::from("--list")]),
            Action::List
        );

        assert_eq!(
            action_from_args(vec![launcher_string.clone(), String::from("-V")]),
            Action::Execute {
                launcher_path: launcher_path.clone(),
                version: RequestedVersion::Any,
                args: vec![String::from("-V")],
            }
        );

        assert_eq!(
            action_from_args(vec![launcher_string.clone(), String::from("-3")]),
            Action::Execute {
                launcher_path: launcher_path.clone(),
                version: RequestedVersion::MajorOnly(3),
                args: Vec::new(),
            }
        );

        assert_eq!(
            action_from_args(vec![launcher_string.clone(), String::from("-3.6")]),
            Action::Execute {
                launcher_path: launcher_path.clone(),
                version: RequestedVersion::Exact(3, 6),
                args: Vec::new(),
            }
        );

        assert_eq!(
            action_from_args(vec![
                launcher_string.clone(),
                String::from("-3.6"),
                String::from("script.py"),
            ]),
            Action::Execute {
                launcher_path: launcher_path.clone(),
                version: RequestedVersion::Exact(3, 6),
                args: vec![String::from("script.py")],
            }
        );

        assert_eq!(
            action_from_args(vec![launcher_string.clone(), String::from("script.py")]),
            Action::Execute {
                launcher_path: launcher_path.clone(),
                version: RequestedVersion::Any,
                args: vec![String::from("script.py")],
            }
        );
    }

    #[test]
    fn test_virtual_env() {
        let original_venv = env::var_os("VIRTUAL_ENV");

        env::remove_var("VIRTUAL_ENV");
        assert_eq!(virtual_env(), None);

        env::set_var("VIRTUAL_ENV", "/some/path");
        assert_eq!(virtual_env(), Some(PathBuf::from("/some/path/bin/python")));

        match original_venv {
            None => env::remove_var("VIRTUAL_ENV"),
            Some(venv_value) => env::set_var("VIRTUAL_ENV", venv_value),
        }
    }

    #[test]
    fn test_find_shebang() {
        // Common case.
        assert_eq!(
            find_shebang("#! /usr/bin/cat\nprint('Hello!')\n".as_bytes()),
            Some("/usr/bin/cat".to_string())
        );

        // No shebang.
        assert_eq!(find_shebang("print('Hello!')".as_bytes()), None);

        // No whitespace between `#!` and command.
        assert_eq!(
            find_shebang("#!/usr/bin/cat\nHello".as_bytes()),
            Some("/usr/bin/cat".to_string())
        );

        // Command wtih arguments.
        assert_eq!(
            find_shebang("#! /usr/bin/env python -S".as_bytes()),
            Some("/usr/bin/env python -S".to_string())
        );

        // Strip trailing whitespace.
        assert_eq!(
            find_shebang("#! /usr/bin/python \n# Hello".as_bytes()),
            Some("/usr/bin/python".to_string())
        );

        // Nothing but a shebang.
        assert_eq!(
            find_shebang("#! /usr/bin/python".as_bytes()),
            Some("/usr/bin/python".to_string())
        );
    }

    #[test]
    fn test_split_shebang() {
        assert_eq!(split_shebang(&"/usr/bin/rustup".to_string()), None);
        assert_eq!(
            split_shebang(&"/usr/bin/rustup self update".to_string()),
            None
        );
        assert_eq!(
            split_shebang(&"/usr/bin/env python".to_string()),
            Some((RequestedVersion::MajorOnly(2), Vec::new()))
        );
        assert_eq!(
            split_shebang(&"/usr/bin/python42.13".to_string()),
            Some((RequestedVersion::Exact(42, 13), Vec::new()))
        );
        assert_eq!(
            split_shebang(&"python -S -v".to_string()),
            Some((
                RequestedVersion::MajorOnly(2),
                vec!["-S".to_string(), "-v".to_string()]
            ))
        );
        assert_eq!(
            split_shebang(&"/usr/local/bin/python3.7 -S".to_string()),
            Some((RequestedVersion::Exact(3, 7), vec!["-S".to_string()]))
        );
    }
}
