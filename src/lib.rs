pub mod cli;

use std::{
    collections::HashMap,
    convert::From,
    env, fmt,
    num::ParseIntError,
    path::{Path, PathBuf},
    str::FromStr,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    // {RequestedVersion, ExactVersion}::from_str
    ParseVersionComponentError(ParseIntError),
    // RequestedVersion::from_str
    DotMissing,
    // ExactVersion::from_path
    FileNameMissing,
    FileNameToStrError,
    PathFileNameError,
    // cli::{list_executables, find_executable, help}
    NoExecutableFound(RequestedVersion),
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseVersionComponentError(int_error) => {
                write!(f, "Error parsing a version component: {}", int_error)
            }
            Self::DotMissing => write!(f, "'.' missing from the version"),
            Self::FileNameMissing => write!(f, "Path object lacks a file name"),
            Self::FileNameToStrError => write!(f, "Failed to convert file name to `str`"),
            Self::PathFileNameError => write!(f, "File name not of the format `pythonX.Y`"),
            Self::NoExecutableFound(requested_version) => write!(
                f,
                "No executable found for {}",
                requested_version.to_string()
            ),
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ParseVersionComponentError(int_error) => Some(int_error),
            Self::DotMissing => None,
            Self::FileNameMissing => None,
            Self::FileNameToStrError => None,
            Self::PathFileNameError => None,
            Self::NoExecutableFound(_) => None,
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl Error {
    pub fn exit_code(&self) -> exitcode::ExitCode {
        match self {
            Self::ParseVersionComponentError(_) => exitcode::USAGE,
            Self::DotMissing => exitcode::USAGE,
            Self::FileNameMissing => exitcode::USAGE,
            Self::FileNameToStrError => exitcode::SOFTWARE,
            Self::PathFileNameError => exitcode::SOFTWARE,
            Self::NoExecutableFound(_) => exitcode::USAGE,
        }
    }
}

/// An integral part of a version specifier (e.g. the `X` or `Y` of `X.Y`).
type ComponentSize = u16;

/// Represents the version of Python a user requsted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RequestedVersion {
    Any,
    MajorOnly(ComponentSize),
    Exact(ComponentSize, ComponentSize),
}

impl ToString for RequestedVersion {
    fn to_string(&self) -> String {
        match self {
            Self::Any => "Python".to_string(),
            Self::MajorOnly(major) => format!("Python {}", major),
            Self::Exact(major, minor) => format!("Python {}.{}", major, minor),
        }
    }
}

impl FromStr for RequestedVersion {
    type Err = Error;

    // XXX Require `python` as a prefix?
    fn from_str(version_string: &str) -> Result<Self> {
        if version_string.is_empty() {
            Ok(Self::Any)
        } else if version_string.contains('.') {
            let exact_version = ExactVersion::from_str(version_string)?;
            Ok(Self::Exact(exact_version.major, exact_version.minor))
        } else {
            match version_string.parse::<ComponentSize>() {
                Ok(number) => Ok(Self::MajorOnly(number)),
                Err(parse_error) => Err(Error::ParseVersionComponentError(parse_error)),
            }
        }
    }
}

impl RequestedVersion {
    /// Returns the string representing the environment variable for the requested version.
    pub fn env_var(self) -> Option<String> {
        match self {
            Self::Any => Some("PY_PYTHON".to_string()),
            Self::MajorOnly(major) => Some(format!("PY_PYTHON{}", major)),
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
        Self::Exact(version.major, version.minor)
    }
}

impl ToString for ExactVersion {
    fn to_string(&self) -> String {
        format!("{}.{}", self.major, self.minor)
    }
}

impl FromStr for ExactVersion {
    type Err = Error;

    fn from_str(version_string: &str) -> Result<Self> {
        match version_string.find('.') {
            Some(dot_index) => {
                let major_str = &version_string[..dot_index];
                let major = match major_str.parse::<ComponentSize>() {
                    Ok(number) => number,
                    Err(parse_error) => return Err(Error::ParseVersionComponentError(parse_error)),
                };
                let minor_str = &version_string[dot_index + 1..];

                match minor_str.parse::<ComponentSize>() {
                    Ok(minor) => Ok(Self { major, minor }),
                    Err(parse_error) => Err(Error::ParseVersionComponentError(parse_error)),
                }
            }
            None => Err(Error::DotMissing),
        }
    }
}

fn acceptable_file_name(file_name: &str) -> bool {
    file_name.len() >= "python3.0".len() && file_name.starts_with("python")
}

impl ExactVersion {
    pub fn from_path(path: &Path) -> Result<Self> {
        path.file_name()
            .ok_or(Error::FileNameMissing)
            .and_then(|raw_file_name| match raw_file_name.to_str() {
                Some(file_name) if acceptable_file_name(file_name) => {
                    Self::from_str(&file_name["python".len()..])
                }
                Some(_) => Err(Error::PathFileNameError),
                None => Err(Error::FileNameToStrError),
            })
    }

    // XXX from_shebang()?

    pub fn supports(&self, requested: RequestedVersion) -> bool {
        match requested {
            RequestedVersion::Any => true,
            RequestedVersion::MajorOnly(major_version) => self.major == major_version,
            RequestedVersion::Exact(major_version, minor_version) => {
                self.major == major_version && self.minor == minor_version
            }
        }
    }
}

fn env_path() -> Vec<PathBuf> {
    // Would love to have a return type of `impl Iterator<Item = PathBuf>
    // and return just SplitPaths and iter::empty(), but Rust
    // complains about differing return types.
    match env::var_os("PATH") {
        Some(path_val) => env::split_paths(&path_val).collect(),
        None => Vec::new(),
    }
}

fn flatten_directories(
    directories: impl IntoIterator<Item = PathBuf>,
) -> impl Iterator<Item = PathBuf> {
    directories
        .into_iter()
        .filter_map(|p| p.read_dir().ok()) // Filter to Ok(ReadDir).
        .flatten() // Flatten out `for DirEntry in ReadDir`.
        .filter_map(|e| e.ok()) // Filter to Ok(DirEntry).
        .map(|e| e.path()) // Get the PathBuf from the DirEntry.
}

fn all_executables_in_paths(
    paths: impl IntoIterator<Item = PathBuf>,
) -> HashMap<ExactVersion, PathBuf> {
    let mut executables = HashMap::new();
    paths.into_iter().for_each(|path| {
        ExactVersion::from_path(&path).map_or((), |version| {
            executables.entry(version).or_insert(path);
        })
    });

    log::debug!("Found executables: {:?}", executables.values());
    executables
}

pub fn all_executables() -> HashMap<ExactVersion, PathBuf> {
    log::info!("Checking PATH environment variable");
    let path_entries = env_path();
    log::debug!("PATH: {:?}", path_entries);
    let paths = flatten_directories(path_entries);
    all_executables_in_paths(paths)
}

fn find_executable_in_hashmap(
    requested: RequestedVersion,
    found_executables: &HashMap<ExactVersion, PathBuf>,
) -> Option<PathBuf> {
    let mut iter = found_executables.iter();
    match requested {
        RequestedVersion::Any => iter.max(),
        RequestedVersion::MajorOnly(_) => iter.filter(|pair| pair.0.supports(requested)).max(),
        RequestedVersion::Exact(_, _) => iter.find(|pair| pair.0.supports(requested)),
    }
    .map(|pair| pair.1.clone())
}

pub fn find_executable(requested: RequestedVersion) -> Option<PathBuf> {
    let found_executables = all_executables();
    find_executable_in_hashmap(requested, &found_executables)
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case(RequestedVersion::Any => "Python" ; "Any")]
    #[test_case(RequestedVersion::MajorOnly(3) => "Python 3" ; "Major")]
    #[test_case(RequestedVersion::Exact(3, 8) => "Python 3.8" ; "Exact/major.minor")]
    fn requestedversion_to_string_tests(requested_version: RequestedVersion) -> String {
        requested_version.to_string()
    }

    #[test_case(".3" => matches Err(Error::ParseVersionComponentError(_)) ; "missing major version is an error")]
    #[test_case("3." => matches Err(Error::ParseVersionComponentError(_)) ; "missing minor version is an error")]
    #[test_case("h" => matches Err(Error::ParseVersionComponentError(_)) ; "non-number, non-emptry string is an error")]
    #[test_case("3.b" => matches Err(Error::ParseVersionComponentError(_)) ; "major.minor where minor is a non-number is an error")]
    #[test_case("a.7" => matches Err(Error::ParseVersionComponentError(_)) ; "major.minor where major is a non-number is an error")]
    #[test_case("" => Ok(RequestedVersion::Any) ; "empty string is Any")]
    #[test_case("3" => Ok(RequestedVersion::MajorOnly(3)) ; "major-only version")]
    #[test_case("3.8" => Ok(RequestedVersion::Exact(3, 8)) ; "major.minor")]
    #[test_case("42.13" => Ok(RequestedVersion::Exact(42, 13)) ; "double digit version components")]
    #[test_case("3.6.5" => matches Err(Error::ParseVersionComponentError(_)) ; "specifying a micro version is an error")]
    fn requestedversion_from_str_tests(version_str: &str) -> Result<RequestedVersion> {
        RequestedVersion::from_str(version_str)
    }

    #[test_case(RequestedVersion::Any => Some("PY_PYTHON".to_string()) ; "Any is PY_PYTHON")]
    #[test_case(RequestedVersion::MajorOnly(3) => Some("PY_PYTHON3".to_string()) ; "major-only is PY_PYTHON{major}")]
    #[test_case(RequestedVersion::MajorOnly(42) => Some("PY_PYTHON42".to_string()) ; "double-digit major component")]
    #[test_case(RequestedVersion::Exact(42, 13) => None ; "exact/major.minor has no environment variable")]
    fn requstedversion_env_var_tests(requested_version: RequestedVersion) -> Option<String> {
        requested_version.env_var()
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

    #[test_case(3, 8 => "3.8" ; "single digits")]
    #[test_case(42, 13 => "42.13" ; "double digits")]
    fn exactversion_to_string_tests(major: ComponentSize, minor: ComponentSize) -> String {
        ExactVersion { major, minor }.to_string()
    }

    #[test_case("" => Err(Error::DotMissing) ; "empty string is an error")]
    #[test_case("3" => Err(Error::DotMissing) ; "major-only version is an error")]
    #[test_case(".7" => matches Err(Error::ParseVersionComponentError(_)) ; "missing major version is an error")]
    #[test_case("3." => matches Err(Error::ParseVersionComponentError(_)) ; "missing minor version is an error")]
    #[test_case("3.Y" => matches Err(Error::ParseVersionComponentError(_)) ; "non-digit minor version is an error")]
    #[test_case("X.7" => matches Err(Error::ParseVersionComponentError(_)) ; "non-digit major version is an error")]
    #[test_case("42.13" => Ok(ExactVersion {major: 42, minor: 13 }) ; "double digit version components")]
    fn exactversion_from_str_tests(version_str: &str) -> Result<ExactVersion> {
        ExactVersion::from_str(version_str)
    }

    #[test_case("/" => Err(Error::FileNameMissing) ; "path missing a file name is an error")]
    #[test_case("/notpython" => Err(Error::PathFileNameError) ; "path not ending with 'python' is an error")]
    #[test_case("/python3" => Err(Error::PathFileNameError) ; "filename lacking a minor component is an error")]
    #[test_case("/pythonX.Y" => matches Err(Error::ParseVersionComponentError(_)) ; "filename with non-digit version is an error")]
    #[test_case("/python42.13" => Ok(ExactVersion { major: 42, minor: 13 }) ; "double digit version components")]
    fn exactversion_from_path_tests(path: &str) -> Result<ExactVersion> {
        ExactVersion::from_path(&PathBuf::from(path))
    }

    #[test]
    fn exactversion_from_path_invalid_utf8() {
        // From https://doc.rust-lang.org/std/ffi/struct.OsStr.html#examples-2.
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        let source = [0x66, 0x6f, 0x80, 0x6f];
        let os_str = OsStr::from_bytes(&source[..]);
        let path = PathBuf::from(os_str);
        assert_eq!(
            ExactVersion::from_path(&path),
            Err(Error::FileNameToStrError)
        );
    }

    #[test_case(RequestedVersion::Any => true ; "Any supports all versions")]
    #[test_case(RequestedVersion::MajorOnly(2) => false ; "major-only mismatch")]
    #[test_case(RequestedVersion::MajorOnly(3) => true ; "major-only match")]
    #[test_case(RequestedVersion::Exact(2, 7) => false ; "older major version")]
    #[test_case(RequestedVersion::Exact(3, 5) => false ; "older minor version")]
    #[test_case(RequestedVersion::Exact(4, 0) => false ; "newer major version")]
    #[test_case(RequestedVersion::Exact(3, 7) => false ; "newer minor version")]
    #[test_case(RequestedVersion::Exact(3, 6) => true ; "same version")]
    fn exactversion_supports_tests(requested_version: RequestedVersion) -> bool {
        let example = ExactVersion { major: 3, minor: 6 };
        example.supports(requested_version)
    }

    #[test_case(2, 7, "/dir1/python2.7" ; "first directory")]
    #[test_case(3, 6, "/dir1/python3.6" ; "matches in multiple directories")]
    #[test_case(3, 7, "/dir2/python3.7" ; "last directory")]
    fn all_executables_in_paths_tests(major: ComponentSize, minor: ComponentSize, path: &str) {
        let python27_path = PathBuf::from("/dir1/python2.7");
        let python36_dir1_path = PathBuf::from("/dir1/python3.6");
        let python36_dir2_path = PathBuf::from("/dir2/python3.6");
        let python37_path = PathBuf::from("/dir2/python3.7");
        let files = vec![
            python27_path,
            python36_dir1_path,
            python36_dir2_path,
            python37_path,
        ];

        let executables = all_executables_in_paths(files.into_iter());
        assert_eq!(executables.len(), 3);

        let version = ExactVersion { major, minor };
        assert!(executables.contains_key(&version));
        assert_eq!(executables.get(&version), Some(&PathBuf::from(path)));
    }

    #[test_case(RequestedVersion::Any => Some(PathBuf::from("/python3.7")) ; "Any version chooses newest version")]
    #[test_case(RequestedVersion::MajorOnly(42) => None ; "major-only version newer than any options")]
    #[test_case(RequestedVersion::MajorOnly(3) => Some(PathBuf::from("/python3.7")) ; "matching major version chooses newest minor version")]
    #[test_case(RequestedVersion::Exact(3, 8) => None ; "version not available")]
    #[test_case(RequestedVersion::Exact(3, 6) => Some(PathBuf::from("/python3.6")) ; "exact version match")]
    fn find_executable_in_hashmap_tests(requested_version: RequestedVersion) -> Option<PathBuf> {
        let mut executables = HashMap::new();
        assert_eq!(
            find_executable_in_hashmap(RequestedVersion::Any, &executables),
            None
        );

        let python36_path = PathBuf::from("/python3.6");
        executables.insert(ExactVersion { major: 3, minor: 6 }, python36_path);

        let python37_path = PathBuf::from("/python3.7");
        executables.insert(ExactVersion { major: 3, minor: 7 }, python37_path);

        find_executable_in_hashmap(requested_version, &executables)
    }
}
