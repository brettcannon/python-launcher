use std::{convert::From, path::Path, str::FromStr};

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

/// XXX
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

/// Represents how tight of a match an `ExactVersion` is to a `RequestedVersion`.
#[derive(Debug, PartialEq)]
pub enum Match {
    No,       // Not compatible.
    Somewhat, // Compatible, but potential for a better, newer match.
    Yes,      // Matches a major.minor exactly.
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
    /// Sees how well of a match this Python version is for `requested`.
    pub fn matches(&self, requested: RequestedVersion) -> Match {
        match requested {
            RequestedVersion::Any => Match::Somewhat,
            RequestedVersion::MajorOnly(major) => {
                if self.major == major {
                    Match::Somewhat
                } else {
                    Match::No
                }
            }
            RequestedVersion::Exact(major, minor) => {
                if self.major == major && self.minor == minor {
                    Match::Yes
                } else {
                    Match::No
                }
            }
        }
    }

    // XXX Tests
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
    fn test_version_matches() {
        let any = RequestedVersion::Any;
        let loose_42 = RequestedVersion::MajorOnly(42);
        let exact_42_13 = RequestedVersion::Exact(42, 13);

        let version_3_6 = ExactVersion { major: 3, minor: 6 };
        let version_42_0 = ExactVersion {
            major: 42,
            minor: 0,
        };
        let version_42_13 = ExactVersion {
            major: 42,
            minor: 13,
        };

        assert_eq!(version_3_6.matches(any), Match::Somewhat);
        assert_eq!(version_3_6.matches(loose_42), Match::No);
        assert_eq!(version_3_6.matches(exact_42_13), Match::No);

        assert_eq!(version_42_0.matches(any), Match::Somewhat);
        assert_eq!(version_42_0.matches(loose_42), Match::Somewhat);
        assert_eq!(version_42_0.matches(exact_42_13), Match::No);

        assert_eq!(version_42_13.matches(any), Match::Somewhat);
        assert_eq!(version_42_13.matches(loose_42), Match::Somewhat);
        assert_eq!(version_42_13.matches(exact_42_13), Match::Yes);
    }
}
