use std::{
    collections, env,
    error::Error,
    io::{self, BufRead},
    path,
    str::FromStr,
};

/// An integer part of a version specifier (e.g. the `X or `Y of `X.Y`).
type VersionComponent = u16;

/// Represents the version of Python a user requsted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RequestedVersion {
    Any,
    Loose(VersionComponent),
    Exact(VersionComponent, VersionComponent),
}

impl FromStr for RequestedVersion {
    type Err = String;

    fn from_str(version_string: &str) -> Result<Self, Self::Err> {
        if version_string.is_empty() {
            return Err("version string is empty".to_string());
        }

        let mut char_iter = version_string.chars();
        let mut major_ver = Vec::new();
        let mut dot = false;

        for c in char_iter.by_ref() {
            if c == '.' {
                dot = true;
                break;
            } else if c.is_ascii_digit() {
                major_ver.push(c);
            } else {
                return Err(format!(
                    "{:?} contains a non-numeric and non-period character",
                    version_string
                ));
            }
        }

        let mut minor_ver = Vec::new();
        if dot {
            for c in char_iter.by_ref() {
                if c.is_ascii_digit() {
                    minor_ver.push(c);
                } else {
                    return Err(format!(
                        "{:?} contains a non-numeric character after a period",
                        version_string
                    ));
                }
            }
        }

        let major = char_vec_to_int(&major_ver)?;
        if !dot {
            Ok(RequestedVersion::Loose(major))
        } else if minor_ver.is_empty() {
            Err(format!(
                "{:?} is missing a minor version number",
                version_string
            ))
        } else {
            let minor = char_vec_to_int(&minor_ver)?;
            Ok(RequestedVersion::Exact(major, minor))
        }
    }
}

/// The version of Python found.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Version {
    major: VersionComponent,
    minor: VersionComponent,
}

/// Represents how tight of a match a `Version` is to a `RequestedVersion`.
#[derive(Debug, PartialEq)]
pub enum VersionMatch {
    NotAtAll, // Not compatible.
    Loosely,  // Compatible, but potential for a better, newer match.
    Exactly,  // Matches a major.minor exactly.
}

impl Version {
    // XXX Make VersionMatch a part of Version.
    /// Sees how well of a match this Python version is for `requested`.
    pub fn matches(&self, requested: RequestedVersion) -> VersionMatch {
        match requested {
            RequestedVersion::Any => VersionMatch::Loosely,
            RequestedVersion::Loose(major) => {
                if self.major == major {
                    VersionMatch::Loosely
                } else {
                    VersionMatch::NotAtAll
                }
            }
            RequestedVersion::Exact(major, minor) => {
                if self.major == major && self.minor == minor {
                    VersionMatch::Exactly
                } else {
                    VersionMatch::NotAtAll
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Help(path::PathBuf),
    Execute {
        launcher: path::PathBuf,
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
/// let action = py::action_from_args(args);
/// ```
pub fn action_from_args(mut args: Vec<String>) -> Action {
    let launcher_path = path::PathBuf::from(args.remove(0)); // Strip the path to this executable.
    if !args.is_empty() {
        let flag = &args[0];

        if flag == "-h" || flag == "--help" {
            return Action::Help(launcher_path);
        } else if let Some(version) = version_from_flag(&flag) {
            args.remove(0);
            return Action::Execute {
                launcher: launcher_path,
                version,
                args,
            };
        }
    }
    Action::Execute {
        launcher: launcher_path,
        version: RequestedVersion::Any,
        args,
    }
}

/// Converts a `Vec<char>` to a `VersionComponent` integer.
fn char_vec_to_int(char_vec: &[char]) -> Result<VersionComponent, String> {
    let joined_string = char_vec.iter().collect::<String>();
    let parse_result = joined_string.parse();
    parse_result.or_else(|_| Err(format!("error converting {:?} to a number", joined_string)))
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

/// Returns the entries in `PATH`.
pub fn path_entries() -> Vec<path::PathBuf> {
    if let Some(path_val) = env::var_os("PATH") {
        env::split_paths(&path_val).collect()
    } else {
        Vec::new()
    }
}

/// Gets the contents of a directory.
///
/// Exists primarily to unwrap and ignore any unencodeable names.
pub fn directory_contents(path: &path::Path) -> collections::HashSet<path::PathBuf> {
    let mut paths = collections::HashSet::new();
    if let Ok(contents) = path.read_dir() {
        for content in contents {
            if let Ok(found_content) = content {
                paths.insert(found_content.path());
            }
        }
    }

    paths
}

/// Filters the paths down to `pythonX.Y` paths.
pub fn filter_python_executables(
    paths: collections::HashSet<path::PathBuf>,
) -> collections::HashMap<Version, path::PathBuf> {
    let mut executables = collections::HashMap::new();
    for path in paths {
        let unencoded_file_name = match path.file_name() {
            Some(x) => x,
            None => continue,
        };
        let file_name = match unencoded_file_name.to_str() {
            Some(x) => x,
            None => continue,
        };
        if file_name.len() < "python3.0".len() || !file_name.starts_with("python") {
            continue;
        }
        let version_part = &file_name["python".len()..];
        if let Ok(found_version) = RequestedVersion::from_str(&version_part) {
            match found_version {
                RequestedVersion::Exact(major, minor) => {
                    executables.insert(Version { major, minor }, path.clone())
                }
                _ => continue,
            };
        }
    }

    executables
}

/// Finds the executable representing the latest Python version.
pub fn choose_executable(
    version_paths: &collections::HashMap<Version, path::PathBuf>,
) -> Option<path::PathBuf> {
    let mut pairs: Vec<(&Version, &path::PathBuf)> = version_paths.iter().collect();
    pairs.sort_unstable_by_key(|p| p.0);
    pairs.last().map(|(_, path)| path.to_path_buf())
}

/// Checks the environment variable `env_var_name` for a Python version.
fn check_env_var(env_var_name: &str) -> Result<RequestedVersion, String> {
    let env_var = match env::var(env_var_name) {
        Ok(e) => e,
        Err(e) => return Err(e.description().to_string()),
    };

    RequestedVersion::from_str(&env_var)
}

/// Checks the `PY_PYTHON` environment variable.
pub fn check_default_env_var() -> Result<RequestedVersion, String> {
    check_env_var("PY_PYTHON")
}

/// Checks the `PY_PYTHON{major}` environment variable.
pub fn check_major_env_var(major: VersionComponent) -> Result<RequestedVersion, String> {
    check_env_var(&format!("PY_PYTHON{}", major))
}

// XXX Shebang:
//      In main():
//          Prepend extra arguments to `argv`
//          Continue search for an appropriate Python version

/// Finds the shebang line from `reader`.
///
/// If a shebang line is found, then the `#!` is removed and the line is stripped of leading and trailing whitespace.
pub fn find_shebang(reader: impl io::Read) -> Option<String> {
    let mut buffered_reader = io::BufReader::new(reader);

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
            Ok(RequestedVersion::Loose(2))
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
    #[allow(non_snake_case)]
    fn test_RequestedVersion_from_string() {
        assert!(RequestedVersion::from_str(&".3".to_string()).is_err());
        assert!(RequestedVersion::from_str(&"3.".to_string()).is_err());
        assert!(RequestedVersion::from_str(&"h".to_string()).is_err());
        assert!(RequestedVersion::from_str(&"3.b".to_string()).is_err());
        assert!(RequestedVersion::from_str(&"a.7".to_string()).is_err());
        assert_eq!(
            RequestedVersion::from_str(&"3".to_string()),
            Ok(RequestedVersion::Loose(3))
        );
        assert_eq!(
            RequestedVersion::from_str(&"3.8".to_string()),
            Ok(RequestedVersion::Exact(3, 8))
        );
        assert_eq!(
            RequestedVersion::from_str(&"42.13".to_string()),
            Ok(RequestedVersion::Exact(42, 13))
        );
        assert!(RequestedVersion::from_str(&"3.6.5".to_string()).is_err());
    }

    #[test]
    fn test_version_from_flag() {
        assert!(version_from_flag(&"-S".to_string()).is_none());
        assert!(version_from_flag(&"--something".to_string()).is_none());
        assert_eq!(
            version_from_flag(&"-3".to_string()),
            Some(RequestedVersion::Loose(3))
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
        let launcher_path = path::PathBuf::from(&launcher_string);

        assert_eq!(
            action_from_args(vec![launcher_string.clone()]),
            Action::Execute {
                launcher: launcher_path.clone(),
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
            action_from_args(vec![launcher_string.clone(), String::from("-V")]),
            Action::Execute {
                launcher: launcher_path.clone(),
                version: RequestedVersion::Any,
                args: vec![String::from("-V")],
            }
        );

        assert_eq!(
            action_from_args(vec![launcher_string.clone(), String::from("-3")]),
            Action::Execute {
                launcher: launcher_path.clone(),
                version: RequestedVersion::Loose(3),
                args: Vec::new(),
            }
        );

        assert_eq!(
            action_from_args(vec![launcher_string.clone(), String::from("-3.6")]),
            Action::Execute {
                launcher: launcher_path.clone(),
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
                launcher: launcher_path.clone(),
                version: RequestedVersion::Exact(3, 6),
                args: vec![String::from("script.py")],
            }
        );

        assert_eq!(
            action_from_args(vec![launcher_string.clone(), String::from("script.py")]),
            Action::Execute {
                launcher: launcher_path.clone(),
                version: RequestedVersion::Any,
                args: vec![String::from("script.py")],
            }
        );
    }

    #[test]
    fn unit_test_path_entries() {
        let paths = vec!["/a", "/b", "/c"];
        if let Ok(joined_paths) = env::join_paths(&paths) {
            let original_paths = env::var_os("PATH");
            env::set_var("PATH", joined_paths);
            assert_eq!(
                path_entries(),
                paths
                    .iter()
                    .map(|p| path::PathBuf::from(p))
                    .collect::<Vec<path::PathBuf>>()
            );
            match original_paths {
                Some(paths) => env::set_var("PATH", paths),
                None => env::set_var("PATH", ""),
            }
        }
    }

    #[test]
    fn system_test_path_entries() {
        if let Some(paths) = env::var_os("PATH") {
            let found_paths = path_entries();
            assert_eq!(found_paths.len(), env::split_paths(&paths).count());
            for (index, path) in env::split_paths(&paths).enumerate() {
                assert_eq!(found_paths[index], path);
            }
        }
    }

    #[test]
    fn test_filter_python_executables() {
        let paths = vec![
            "/bad/path/python",    // Under-specified.
            "/bad/path/python3",   // Under-specified.
            "/bad/path/hello",     // Not Python.
            "/bad/path/pytho3.6",  // Typo.
            "/bad/path/rython3.6", // Typo.
            "/good/path/python3.6",
            "/good/python42.13",
        ];
        let all_paths = paths
            .iter()
            .map(path::PathBuf::from)
            .collect::<collections::HashSet<path::PathBuf>>();
        let results = filter_python_executables(all_paths);
        let good_version1 = Version { major: 3, minor: 6 };
        let good_version2 = Version {
            major: 42,
            minor: 13,
        };
        let mut expected = paths[5];
        match results.get(&good_version1) {
            Some(path) => assert_eq!(*path, path::PathBuf::from(expected)),
            None => panic!("{:?} not found", good_version1),
        };
        expected = paths[6];
        match results.get(&good_version2) {
            Some(path) => assert_eq!(*path, path::PathBuf::from(expected)),
            None => panic!("{:?} not found", good_version2),
        }
    }

    #[test]
    fn test_version_matches() {
        let any = RequestedVersion::Any;
        let loose_42 = RequestedVersion::Loose(42);
        let exact_42_13 = RequestedVersion::Exact(42, 13);

        let version_3_6 = Version { major: 3, minor: 6 };
        let version_42_0 = Version {
            major: 42,
            minor: 0,
        };
        let version_42_13 = Version {
            major: 42,
            minor: 13,
        };

        assert_eq!(version_3_6.matches(any), VersionMatch::Loosely);
        assert_eq!(version_3_6.matches(loose_42), VersionMatch::NotAtAll);
        assert_eq!(version_3_6.matches(exact_42_13), VersionMatch::NotAtAll);

        assert_eq!(version_42_0.matches(any), VersionMatch::Loosely);
        assert_eq!(version_42_0.matches(loose_42), VersionMatch::Loosely);
        assert_eq!(version_42_0.matches(exact_42_13), VersionMatch::NotAtAll);

        assert_eq!(version_42_13.matches(any), VersionMatch::Loosely);
        assert_eq!(version_42_13.matches(loose_42), VersionMatch::Loosely);
        assert_eq!(version_42_13.matches(exact_42_13), VersionMatch::Exactly);
    }

    #[test]
    fn test_choose_executable() {
        let version_3_6 = Version { major: 3, minor: 6 };
        let version_42_0 = Version {
            major: 42,
            minor: 0,
        };
        let version_42_13 = Version {
            major: 42,
            minor: 13,
        };
        let path_3_6 = path::PathBuf::from("/python3.6");
        let path_42_0 = path::PathBuf::from("/python42.0");
        let path_42_13 = path::PathBuf::from("/python42.13");
        let mut mapping = collections::HashMap::new();

        if choose_executable(&mapping).is_some() {
            panic!("found a non-existent path");
        };

        mapping.insert(version_3_6, path_3_6.clone());
        match choose_executable(&mapping) {
            Some(path) => assert_eq!(path, path_3_6),
            None => panic!("no path found"),
        }

        mapping.insert(version_42_0, path_42_0);
        mapping.insert(version_42_13, path_42_13.clone());
        match choose_executable(&mapping) {
            Some(path) => assert_eq!(path, path_42_13),
            None => panic!("no path found"),
        }
    }

    #[test]
    fn test_check_default_env_var() {
        env::remove_var("PY_PYTHON");
        match check_default_env_var() {
            Err(_) => (),
            Ok(_) => panic!("supposedly a value was found when PY_PYTHON isn't set"),
        };

        env::set_var("PY_PYTHON", "42.13");
        assert_eq!(check_default_env_var(), Ok(RequestedVersion::Exact(42, 13)));

        env::set_var("PY_PYTHON", "some bunk");
        match check_default_env_var() {
            Err(_) => (),
            Ok(_) => panic!("supposedly found a value when PY_PYTHON set to 'some bunk'"),
        }
    }

    #[test]
    fn test_check_major_env_var() {
        env::remove_var("PY_PYTHON42");
        match check_major_env_var(42) {
            Err(_) => (),
            Ok(_) => panic!("supposedly a value was found when PY_PYTHON42 isn't set"),
        };

        env::set_var("PY_PYTHON42", "42.13");
        assert_eq!(check_major_env_var(42), Ok(RequestedVersion::Exact(42, 13)));

        env::set_var("PY_PYTHON42", "some bunk");
        match check_major_env_var(42) {
            Err(_) => (),
            Ok(_) => panic!("supposedly found a value when PY_PYTHON42 set to 'some bunk'"),
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
            Some((RequestedVersion::Loose(2), Vec::new()))
        );
        assert_eq!(
            split_shebang(&"/usr/bin/python42.13".to_string()),
            Some((RequestedVersion::Exact(42, 13), Vec::new()))
        );
        assert_eq!(
            split_shebang(&"python -S -v".to_string()),
            Some((
                RequestedVersion::Loose(2),
                vec!["-S".to_string(), "-v".to_string()]
            ))
        );
        assert_eq!(
            split_shebang(&"/usr/local/bin/python3.7 -S".to_string()),
            Some((RequestedVersion::Exact(3, 7), vec!["-S".to_string()]))
        );
    }
}
