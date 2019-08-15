mod common;

use std::env;
use std::ffi::OsStr;

use serial_test_derive::serial;
use tempfile::TempDir;

use python_launcher;
use python_launcher::{ExactVersion, RequestedVersion};

use common::EnvVarState;

#[test]
#[serial]
fn all_executables() {
    println!("all_executables()");
    let dir1 = TempDir::new().unwrap();
    let dir2 = TempDir::new().unwrap();

    let python27_path = common::touch_file(dir1.path().join("python2.7"));
    let python36_dir1_path = common::touch_file(dir1.path().join("python3.6"));
    common::touch_file(dir2.path().join("python3.6"));
    let python37_path = common::touch_file(dir2.path().join("python3.7"));

    let new_path = env::join_paths([dir1.path(), dir2.path()].iter()).unwrap();
    let mut _temp_path = EnvVarState::new();
    _temp_path.change(OsStr::new("PATH"), Some(&new_path));

    let executables = python_launcher::all_executables();

    assert_eq!(executables.len(), 3);

    let python27_version = ExactVersion { major: 2, minor: 7 };
    assert!(executables.contains_key(&python27_version));
    assert_eq!(executables.get(&python27_version), Some(&python27_path));

    let python36_version = ExactVersion { major: 3, minor: 6 };
    assert!(executables.contains_key(&python27_version));
    assert_eq!(
        executables.get(&python36_version),
        Some(&python36_dir1_path)
    );

    let python37_version = ExactVersion { major: 3, minor: 7 };
    assert!(executables.contains_key(&python37_version));
    assert_eq!(executables.get(&python37_version), Some(&python37_path));
}

#[test]
#[serial]
fn find_executable() {
    println!("find_executable()");
    let dir1 = TempDir::new().unwrap();
    let dir2 = TempDir::new().unwrap();

    let python27_path = common::touch_file(dir1.path().join("python2.7"));
    let python36_path = common::touch_file(dir1.path().join("python3.6"));
    common::touch_file(dir2.path().join("python3.6"));
    let python37_path = common::touch_file(dir2.path().join("python3.7"));

    let new_path = env::join_paths([dir1.path(), dir2.path()].iter()).unwrap();
    let mut _temp_path = EnvVarState::new();
    _temp_path.change(OsStr::new("PATH"), Some(&new_path));

    assert_eq!(
        python_launcher::find_executable(RequestedVersion::Any),
        Some(python37_path.clone())
    );

    assert_eq!(
        python_launcher::find_executable(RequestedVersion::MajorOnly(2)),
        Some(python27_path)
    );

    assert_eq!(
        python_launcher::find_executable(RequestedVersion::Exact(3, 6)),
        Some(python36_path)
    );
}
