pub mod cli;

use std::{
    collections::HashMap,
    convert::From,
    env,
    path::{Path, PathBuf},
    str::FromStr,
};

/// An integer part of a version specifier (e.g. the `X or `Y of `X.Y`).
type ComponentSize = u16;
/// Failure to parse a string containing a specified Python version.
type ParseVersionError = String;

/// Represents the version of Python a user requsted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RequestedVersion {
    Any,
    MajorOnly(ComponentSize),
    Exact(ComponentSize, ComponentSize),
}

impl FromStr for RequestedVersion {
    type Err = ParseVersionError;

    // XXX Require `python` as a prefix?
    fn from_str(version_string: &str) -> Result<Self, Self::Err> {
        if version_string.is_empty() {
            Ok(RequestedVersion::Any)
        } else if version_string.contains('.') {
            ExactVersion::from_str(version_string)
                .and_then(|v| Ok(RequestedVersion::Exact(v.major, v.minor)))
        } else {
            match version_string.parse::<ComponentSize>() {
                Ok(number) => Ok(RequestedVersion::MajorOnly(number)),
                Err(parse_error) => Err(parse_error.to_string()),
            }
        }
    }
}

impl RequestedVersion {
    /// Returns the string representing the environment variable for the requested version.
    pub fn env_var(self) -> Option<String> {
        match self {
            RequestedVersion::Any => Some("PY_PYTHON".to_string()),
            RequestedVersion::MajorOnly(major) => Some(format!("PY_PYTHON{}", major)),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExactVersion {
    pub major: ComponentSize,
    pub minor: ComponentSize,
}

impl From<ExactVersion> for RequestedVersion {
    fn from(version: ExactVersion) -> Self {
        RequestedVersion::Exact(version.major, version.minor)
    }
}

impl ToString for ExactVersion {
    fn to_string(&self) -> String {
        format!("{}.{}", self.major, self.minor)
    }
}

impl FromStr for ExactVersion {
    type Err = ParseVersionError;

    fn from_str(version_string: &str) -> Result<Self, Self::Err> {
        if let Some(dot_index) = version_string.find('.') {
            if let Some(major_str) = version_string.get(..dot_index) {
                let major = match major_str.parse::<ComponentSize>() {
                    Ok(number) => number,
                    Err(parse_error) => return Err(parse_error.to_string()),
                };

                if let Some(minor_str) = version_string.get(dot_index + 1..) {
                    return match minor_str.parse::<ComponentSize>() {
                        Ok(minor) => Ok(ExactVersion { major, minor }),
                        Err(parse_error) => Err(parse_error.to_string()),
                    };
                } else {
                    return Err("no minor version after the '.' found".to_string());
                }
            } else {
                return Err("no major version before the '.' found".to_string());
            }
        } else {
            return Err("no '.' found".to_string());
        }
    }
}

impl ExactVersion {
    pub fn supports(&self, requested: RequestedVersion) -> bool {
        match requested {
            RequestedVersion::Any => true,
            RequestedVersion::MajorOnly(major_version) => self.major == major_version,
            RequestedVersion::Exact(major_version, minor_version) => {
                self.major == major_version && self.minor == minor_version
            }
        }
    }

    // XXX Return Result? Would align more with e.g. from_str(), but also requires returning an error message.
    pub fn from_path(path: &Path) -> Option<Self> {
        if let Some(file_name) = path.file_name().and_then(|p| p.to_str()) {
            if file_name.len() >= "python3.0".len() && file_name.starts_with("python") {
                let version_part = &file_name["python".len()..];
                if let Ok(found_version) = RequestedVersion::from_str(&version_part) {
                    return match found_version {
                        RequestedVersion::Exact(major, minor) => {
                            Some(ExactVersion { major, minor })
                        }
                        _ => None,
                    };
                }
            }
        }
        None
    }
}

// XXX Drop `directories` parameter.
pub fn find_executable(
    requested: RequestedVersion,
    directories: impl Iterator<Item = PathBuf>,
) -> Option<PathBuf> {
    // It would seem to make sense to call `.iter()` here, but the borrow checker says "no".
    let found_executables = all_executables(directories);
    match requested {
        RequestedVersion::Any => found_executables.iter().max(),
        RequestedVersion::MajorOnly(_) => found_executables
            .iter()
            .filter(|pair| pair.0.supports(requested))
            .max(),
        RequestedVersion::Exact(_, _) => found_executables
            .iter()
            .find(|pair| pair.0.supports(requested)),
    }
    .map(|pair| pair.1.clone())
}

// XXX Drop `directories` parameter.
pub fn all_executables(
    directories: impl Iterator<Item = PathBuf>,
) -> HashMap<ExactVersion, PathBuf> {
    let mut executables = HashMap::new();
    for path in flatten_directories(directories) {
        if let Some(version) = ExactVersion::from_path(&path) {
            executables.entry(version).or_insert(path);
        }
    }
    executables
}

fn flatten_directories(
    directories: impl Iterator<Item = PathBuf>,
) -> impl Iterator<Item = PathBuf> {
    directories
        .filter_map(|p| p.read_dir().ok()) // Filter to Ok(ReadDir).
        .flatten() // Flatten out `for DirEntry in ReadDir`.
        .filter_map(|e| e.ok()) // Filter to Ok(DirEntry).
        .map(|e| e.path()) // Get the PathBuf from the DirEntry.
}

/// Convert `PATH` into a `Vec<PathBuf>`.
fn path_entries() -> Vec<PathBuf> {
    if let Some(path_val) = env::var_os("PATH") {
        env::split_paths(&path_val).collect()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_requestedversion_from_str() {
        assert!(RequestedVersion::from_str(".3").is_err());
        assert!(RequestedVersion::from_str("3.").is_err());
        assert!(RequestedVersion::from_str("h").is_err());
        assert!(RequestedVersion::from_str("3.b").is_err());
        assert!(RequestedVersion::from_str("a.7").is_err());
        assert_eq!(RequestedVersion::from_str(""), Ok(RequestedVersion::Any));
        assert_eq!(
            RequestedVersion::from_str("3"),
            Ok(RequestedVersion::MajorOnly(3))
        );
        assert_eq!(
            RequestedVersion::from_str("3.8"),
            Ok(RequestedVersion::Exact(3, 8))
        );
        assert_eq!(
            RequestedVersion::from_str("42.13"),
            Ok(RequestedVersion::Exact(42, 13))
        );
        assert!(RequestedVersion::from_str("3.6.5").is_err());
    }

    #[test]
    fn test_requestedversion_from_exactversion() {
        assert_eq!(
            RequestedVersion::from(ExactVersion {
                major: 42,
                minor: 13
            }),
            RequestedVersion::Exact(42, 13)
        );
    }

    #[test]
    fn test_env_var() {
        assert_eq!(
            RequestedVersion::Any.env_var(),
            Some("PY_PYTHON".to_string())
        );
        assert_eq!(
            RequestedVersion::MajorOnly(3).env_var(),
            Some("PY_PYTHON3".to_string())
        );
        assert_eq!(
            RequestedVersion::MajorOnly(42).env_var(),
            Some("PY_PYTHON42".to_string())
        );
        assert!(RequestedVersion::Exact(42, 13).env_var().is_none());
    }

    #[test]
    fn test_exactversion_to_string() {
        assert_eq!(
            ExactVersion { major: 3, minor: 8 }.to_string(),
            "3.8".to_string()
        );
        assert_eq!(
            ExactVersion {
                major: 42,
                minor: 13
            }
            .to_string(),
            "42.13".to_string()
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
                    .map(|p| PathBuf::from(p))
                    .collect::<Vec<PathBuf>>()
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
}
