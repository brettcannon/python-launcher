use std::collections;
use std::env;
use std::path;

/// An integer part of a version specifier (e.g. the `X or `Y of `X.Y`).
type VersionComponent = u16;

/// A mapping of executable version and paths.
type Executables = collections::HashMap<Version, path::PathBuf>;

/// Represents the version of Python a user requsted.
#[derive(Debug, PartialEq)]
pub enum RequestedVersion {
    Any,
    Loose(VersionComponent),
    Exact(VersionComponent, VersionComponent),
}

impl RequestedVersion {
    /// Creates a new `RequestedVersion` from a version specifier string.
    fn from_string(ver: &String) -> Result<Self, String> {
        let mut char_iter = ver.chars();
        let mut major_ver: Vec<char> = Vec::new();
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
                    ver
                ));
            }
        }

        let mut minor_ver: Vec<char> = Vec::new();
        if dot {
            for c in char_iter.by_ref() {
                if c.is_ascii_digit() {
                    minor_ver.push(c);
                } else {
                    return Err(format!(
                        "{:?} contains a non-numeric character after a period",
                        ver
                    ));
                }
            }
        }

        if major_ver.len() == 0 {
            Err(format!("version string is empty"))
        } else {
            let major = char_vec_to_int(&major_ver)?;
            if !dot {
                Ok(RequestedVersion::Loose(major))
            } else if minor_ver.len() == 0 {
                Err(format!("{:?} is missing a minor version number", ver))
            } else {
                let minor = char_vec_to_int(&minor_ver)?;
                Ok(RequestedVersion::Exact(major, minor))
            }
        }
    }
}

/// The version of Python found.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Version {
    major: VersionComponent,
    minor: VersionComponent,
}

/// Represents how tight of a match a `Version` is to a `RequestedVersion`.
#[derive(Debug, PartialEq)]
enum VersionMatch {
    NotAtAll, // Not compatible.
    Loosely,  // Compatible, but potential for a better, newer match.
    Exactly,  // Matches a major.minor exactly.
}

impl Version {
    fn matches(&self, requested: &RequestedVersion) -> VersionMatch {
        match requested {
            RequestedVersion::Any => VersionMatch::Loosely,
            RequestedVersion::Loose(major) => if self.major == *major {
                VersionMatch::Loosely
            } else {
                VersionMatch::NotAtAll
            },
            RequestedVersion::Exact(major, minor) => {
                if self.major == *major && self.minor == *minor {
                    VersionMatch::Exactly
                } else {
                    VersionMatch::NotAtAll
                }
            }
        }
    }
}

/// Collect Python executables matching requested version from PATH.
fn find_executables(requested_version: &RequestedVersion) -> Executables {
    let mut executables = Executables::new();
    for path in path_entries().into_iter() {
        let exes_here = filter_python_executables(directory_contents(&path));
        for (version, executable) in exes_here.into_iter() {
            if executables.contains_key(&version) || !executable.is_file() {
                continue
            }
            match version.matches(requested_version) {
                VersionMatch::Exactly => {
                    executables.insert(version, executable);
                    return executables;
                },
                VersionMatch::Loosely => {
                    executables.insert(version, executable);
                },
                VersionMatch::NotAtAll => {},
            }
        }
    }
    executables
}

/// Choose the best (highest-versioned) Python executable from given mapping.
fn choose_executable(executables: Executables) -> Option<path::PathBuf> {
    executables.into_iter().max_by(|(k1, _), (k2, _)| k1.cmp(k2)).map(|(_, v)| v)
}

/// Converts a `Vec<char>` to a `VersionComponent` integer.
fn char_vec_to_int(char_vec: &Vec<char>) -> Result<VersionComponent, String> {
    let joined_string = char_vec.into_iter().collect::<String>();
    let parse_result = joined_string.parse::<VersionComponent>();
    parse_result.or(Err(format!(
        "error converting {:?} to a number",
        joined_string
    )))
}

/// Attempts to parse a version specifier from a CLI argument.
///
/// Any failure to parse leads to `RequestedVersion::Any` being returned.
fn parse_version_from_cli(arg: &String) -> RequestedVersion {
    if arg.starts_with("-") {
        let mut version = arg.clone();
        version.remove(0);
        match RequestedVersion::from_string(&version) {
            Ok(v) => v,
            Err(_) => RequestedVersion::Any,
        }
    } else {
        RequestedVersion::Any
    }
}

/// Checks if the string contains a version specifier.
///
/// If not version specifier is found, `RequestedVersion::Any` is returned.
//
// https://docs.python.org/3.8/using/windows.html#from-the-command-line
pub fn check_cli_arg(arg: &String) -> RequestedVersion {
    let version_from_cli = parse_version_from_cli(arg);
    if version_from_cli != RequestedVersion::Any {
        version_from_cli
    } else {
        // XXX shebang from file
        println!("No version found in the first CLI arg");
        RequestedVersion::Any
    }
}

/// Returns the entries in `PATH`.
fn path_entries() -> Vec<path::PathBuf> {
    let path_val = match env::var_os("PATH") {
        Some(val) => val,
        None => return Vec::new(),
    };
    env::split_paths(&path_val).collect()
}

/// Gets the contents of a directory.
///
/// Exists primarily to unwrap and ignore any unencodeable names.
fn directory_contents(path: &path::PathBuf) -> collections::HashSet<path::PathBuf> {
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
fn filter_python_executables(
    paths: collections::HashSet<path::PathBuf>,
) -> Executables {
    let mut executables = Executables::new();
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
        if let Ok(found_version) = RequestedVersion::from_string(&version_part.to_string()) {
            match found_version {
                RequestedVersion::Exact(major, minor) => executables.insert(
                    Version {
                        major: major,
                        minor: minor,
                    },
                    path.clone(),
                ),
                _ => continue,
            };
        }
    }

    return executables;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_RequestedVersion_from_string() {
        assert!(RequestedVersion::from_string(&".3".to_string()).is_err());
        assert!(RequestedVersion::from_string(&"3.".to_string()).is_err());
        assert!(RequestedVersion::from_string(&"h".to_string()).is_err());
        assert!(RequestedVersion::from_string(&"3.b".to_string()).is_err());
        assert!(RequestedVersion::from_string(&"a.7".to_string()).is_err());
        assert_eq!(
            RequestedVersion::from_string(&"3".to_string()),
            Ok(RequestedVersion::Loose(3))
        );
        assert_eq!(
            RequestedVersion::from_string(&"3.8".to_string()),
            Ok(RequestedVersion::Exact(3, 8))
        );
        assert_eq!(
            RequestedVersion::from_string(&"42.13".to_string()),
            Ok(RequestedVersion::Exact(42, 13))
        );
        assert!(RequestedVersion::from_string(&"3.6.5".to_string()).is_err());
    }

    #[test]
    fn test_parse_version_from_cli() {
        assert_eq!(
            parse_version_from_cli(&"path/to/file".to_string()),
            RequestedVersion::Any
        );
        assert_eq!(
            parse_version_from_cli(&"3".to_string()),
            RequestedVersion::Any
        );
        assert_eq!(
            parse_version_from_cli(&"-S".to_string()),
            RequestedVersion::Any
        );
        assert_eq!(
            parse_version_from_cli(&"--something".to_string()),
            RequestedVersion::Any
        );
        assert_eq!(
            parse_version_from_cli(&"-3".to_string()),
            RequestedVersion::Loose(3)
        );
        assert_eq!(
            parse_version_from_cli(&"-3.6".to_string()),
            RequestedVersion::Exact(3, 6)
        );
        assert_eq!(
            parse_version_from_cli(&"-42.13".to_string()),
            RequestedVersion::Exact(42, 13)
        );
        assert_eq!(
            parse_version_from_cli(&"-3.6.4".to_string()),
            RequestedVersion::Any
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
            .map(|p| path::PathBuf::from(p))
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

        assert_eq!(version_3_6.matches(&any), VersionMatch::Loosely);
        assert_eq!(version_3_6.matches(&loose_42), VersionMatch::NotAtAll);
        assert_eq!(version_3_6.matches(&exact_42_13), VersionMatch::NotAtAll);

        assert_eq!(version_42_0.matches(&any), VersionMatch::Loosely);
        assert_eq!(version_42_0.matches(&loose_42), VersionMatch::Loosely);
        assert_eq!(version_42_0.matches(&exact_42_13), VersionMatch::NotAtAll);

        assert_eq!(version_42_13.matches(&any), VersionMatch::Loosely);
        assert_eq!(version_42_13.matches(&loose_42), VersionMatch::Loosely);
        assert_eq!(version_42_13.matches(&exact_42_13), VersionMatch::Exactly);
    }

    #[test]
    fn test_choose_executable() {
        let zero_exes = Executables::new();
        let one_exe = vec![
            (Version { major: 3, minor: 8 }, path::PathBuf::from("3.8"))
        ].into_iter().collect();
        let many_exes = vec![
            (Version { major: 2, minor: 5 }, path::PathBuf::from("2.5")),
            (Version { major: 3, minor: 8 }, path::PathBuf::from("3.8")),
            (Version { major: 3, minor: 5 }, path::PathBuf::from("3.5")),
        ].into_iter().collect();

        assert_eq!(choose_executable(zero_exes), None);
        assert_eq!(choose_executable(one_exe), Some(path::PathBuf::from("3.8")));
        assert_eq!(choose_executable(many_exes), Some(path::PathBuf::from("3.8")));
    }
}
