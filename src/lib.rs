use std::env;
use std::path;

type VersionComponent = u16;

#[derive(Debug, PartialEq)]
pub enum RequestedVersion {
    Any,
    Loose(VersionComponent),
    Exact(VersionComponent, VersionComponent),
}

impl RequestedVersion {
    fn from_string(ver: String) -> Result<Self, String> {
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

/*
struct Version {
    major: VersionComponent,
    minor: VersionComponent,
}

enum VersionMatch {
    NotAtAll,
    Loosely,
    Exactly,
}
*/
// fmt::Display
// comparable/sortable

/* XXX Iterator for fully-qualified `pythonX.Y` executables:
fn directory_contents()
    Get the contents of those directories - I/O
    -> Return a collection
fn filter_python_executables()
    Look for `pythonX.Y` entries
    Exact or loose match? Exact then done, else save and keep looking
    -> Map of Version: Path
*/

fn char_vec_to_int(char_vec: &Vec<char>) -> Result<VersionComponent, String> {
    let joined_string = char_vec.into_iter().collect::<String>();
    let parse_result = joined_string.parse::<VersionComponent>();
    parse_result.or(Err(format!(
        "error converting {:?} to a number",
        joined_string
    )))
}

fn parse_version_from_cli(arg: String) -> RequestedVersion {
    if arg.starts_with("-") {
        let mut version = arg;
        version.remove(0);
        match RequestedVersion::from_string(version) {
            Ok(v) => v,
            Err(_) => RequestedVersion::Any,
        }
    } else {
        RequestedVersion::Any
    }
}

// https://docs.python.org/3.8/using/windows.html#from-the-command-line
pub fn check_cli_arg(arg: String) -> RequestedVersion {
    let version_from_cli = parse_version_from_cli(arg);
    if version_from_cli != RequestedVersion::Any {
        version_from_cli
    } else {
        // XXX shebang from file
        println!("No version found in the first CLI arg");
        RequestedVersion::Any
    }
}

fn path_directories() -> Vec<path::PathBuf> {
    let path_val = match env::var_os("PATH") {
        Some(val) => val,
        None => return Vec::new(),
    };
    env::split_paths(&path_val).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_RequestedVersion_from_string() {
        assert!(RequestedVersion::from_string(".3".to_string()).is_err());
        assert!(RequestedVersion::from_string("3.".to_string()).is_err());
        assert!(RequestedVersion::from_string("h".to_string()).is_err());
        assert!(RequestedVersion::from_string("3.b".to_string()).is_err());
        assert!(RequestedVersion::from_string("a.7".to_string()).is_err());
        assert_eq!(
            RequestedVersion::from_string("3".to_string()),
            Ok(RequestedVersion::Loose(3))
        );
        assert_eq!(
            RequestedVersion::from_string("3.8".to_string()),
            Ok(RequestedVersion::Exact(3, 8))
        );
        assert!(RequestedVersion::from_string("3.6.5".to_string()).is_err());
    }

    #[test]
    fn test_parse_version_from_cli() {
        assert_eq!(
            parse_version_from_cli("path/to/file".to_string()),
            RequestedVersion::Any
        );
        assert_eq!(
            parse_version_from_cli("3".to_string()),
            RequestedVersion::Any
        );
        assert_eq!(
            parse_version_from_cli("-S".to_string()),
            RequestedVersion::Any
        );
        assert_eq!(
            parse_version_from_cli("--something".to_string()),
            RequestedVersion::Any
        );
        assert_eq!(
            parse_version_from_cli("-3".to_string()),
            RequestedVersion::Loose(3)
        );
        assert_eq!(
            parse_version_from_cli("-3.6".to_string()),
            RequestedVersion::Exact(3, 6)
        );
        assert_eq!(
            parse_version_from_cli("-3.6.4".to_string()),
            RequestedVersion::Any
        );
    }

    #[test]
    fn test_path_directories() {
        if let Some(paths) = env::var_os("PATH") {
            let found_paths = path_directories();
            assert_eq!(found_paths.len(), env::split_paths(&paths).count());
            for (index, path) in env::split_paths(&paths).enumerate() {
                assert_eq!(found_paths[index], path);
            }
        }
    }
}
