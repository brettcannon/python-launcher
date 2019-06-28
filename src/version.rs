use std::str::FromStr;

/// An integer part of a version specifier (e.g. the `X or `Y of `X.Y`).
type ComponentSize = u16;

/// Represents the version of Python a user requsted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RequestedVersion {
    Any,
    MajorOnly(ComponentSize),
    Exact(ComponentSize, ComponentSize),
}

impl FromStr for RequestedVersion {
    type Err = String;

    // XXX Custom Result
    fn from_str(version_string: &str) -> Result<Self, Self::Err> {
        if version_string.is_empty() {
            return Err("version string is empty".to_string());
        }

        // XXX Crate to help parse?
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
            Ok(RequestedVersion::MajorOnly(major))
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

impl RequestedVersion {
    // XXX Tests
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

/// Represents how tight of a match an `ExactVersion` is to a `RequestedVersion`.
#[derive(Debug, PartialEq)]
pub enum Match {
    No,       // Not compatible.
    Somewhat, // Compatible, but potential for a better, newer match.
    Yes,      // Matches a major.minor exactly.
}

impl ToString for ExactVersion {
    // XXX test
    fn to_string(&self) -> String {
        format!("{}.{}", self.major, self.minor)
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
}

// XXX Must be something better than this
/// Converts a `Vec<char>` to a `ComponentSize` integer.
fn char_vec_to_int(char_vec: &[char]) -> Result<ComponentSize, String> {
    let joined_string = char_vec.iter().collect::<String>();
    let parse_result = joined_string.parse();
    parse_result.or_else(|_| Err(format!("error converting {:?} to a number", joined_string)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_RequestedVersion_from_str() {
        assert!(RequestedVersion::from_str(&".3".to_string()).is_err());
        assert!(RequestedVersion::from_str(&"3.".to_string()).is_err());
        assert!(RequestedVersion::from_str(&"h".to_string()).is_err());
        assert!(RequestedVersion::from_str(&"3.b".to_string()).is_err());
        assert!(RequestedVersion::from_str(&"a.7".to_string()).is_err());
        assert_eq!(
            RequestedVersion::from_str(&"3".to_string()),
            Ok(RequestedVersion::MajorOnly(3))
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
