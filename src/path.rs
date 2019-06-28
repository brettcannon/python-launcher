use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    env,
    hash::BuildHasher,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::version::{ExactVersion, Match, RequestedVersion};

// XXX Convert all PATH traversal code into a lazy iterable
/// Convert `PATH` into a `Vec<PathBuf>`.
pub fn path_entries() -> Vec<PathBuf> {
    if let Some(path_val) = env::var_os("PATH") {
        env::split_paths(&path_val).collect()
    } else {
        Vec::new()
    }
}

/// Gets the contents of a directory.
///
/// Exists primarily to unwrap and ignore any unencodeable names.
pub fn directory_contents(path: &Path) -> HashSet<PathBuf> {
    let mut paths = HashSet::new();
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
pub fn filter_python_executables<S: BuildHasher>(
    paths: HashSet<PathBuf, S>,
) -> HashMap<ExactVersion, PathBuf> {
    let mut executables = HashMap::new();
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
                    executables.insert(ExactVersion { major, minor }, path.clone())
                }
                _ => continue,
            };
        }
    }

    executables
}

// XXX Write tests
/// Find all available executables that are acceptable for the requested version as found on `PATH`.
pub fn available_executables(
    requested_version: RequestedVersion,
) -> HashMap<ExactVersion, PathBuf> {
    let mut found_versions = HashMap::new();
    for path in path_entries() {
        let all_contents = directory_contents(&path);
        for (version, path) in filter_python_executables(all_contents) {
            if let Entry::Vacant(entry) = found_versions.entry(version) {
                match entry.key().matches(requested_version) {
                    Match::No => continue,
                    Match::Somewhat => {
                        if path.is_file() {
                            entry.insert(path);
                        }
                    }
                    Match::Yes => {
                        if path.is_file() {
                            entry.insert(path);
                            return found_versions;
                        }
                    }
                }
            }
        }
    }
    found_versions
}

/// Finds the executable representing the latest Python version.
pub fn choose_executable<S: BuildHasher>(
    version_paths: &HashMap<ExactVersion, PathBuf, S>,
) -> Option<PathBuf> {
    let mut pairs: Vec<(&ExactVersion, &PathBuf)> = version_paths.iter().collect();
    pairs.sort_unstable_by_key(|p| p.0);
    pairs.last().map(|(_, path)| path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

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
            .map(PathBuf::from)
            .collect::<HashSet<PathBuf>>();
        let results = filter_python_executables(all_paths);
        let good_version1 = ExactVersion { major: 3, minor: 6 };
        let good_version2 = ExactVersion {
            major: 42,
            minor: 13,
        };
        let mut expected = paths[5];
        match results.get(&good_version1) {
            Some(path) => assert_eq!(*path, PathBuf::from(expected)),
            None => panic!("{:?} not found", good_version1),
        };
        expected = paths[6];
        match results.get(&good_version2) {
            Some(path) => assert_eq!(*path, PathBuf::from(expected)),
            None => panic!("{:?} not found", good_version2),
        }
    }

    #[test]
    fn test_choose_executable() {
        let version_3_6 = ExactVersion { major: 3, minor: 6 };
        let version_42_0 = ExactVersion {
            major: 42,
            minor: 0,
        };
        let version_42_13 = ExactVersion {
            major: 42,
            minor: 13,
        };
        let path_3_6 = PathBuf::from("/python3.6");
        let path_42_0 = PathBuf::from("/python42.0");
        let path_42_13 = PathBuf::from("/python42.13");
        let mut mapping = HashMap::new();

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
}
