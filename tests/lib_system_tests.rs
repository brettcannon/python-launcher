mod common;

use serial_test_derive::serial;

use python_launcher;
use python_launcher::{ExactVersion, RequestedVersion};

use common::EnvState;

#[test]
#[serial]
fn all_executables() {
    let env_state = EnvState::new();

    let executables = python_launcher::all_executables();

    assert_eq!(executables.len(), 3);

    let python27_version = ExactVersion { major: 2, minor: 7 };
    assert!(executables.contains_key(&python27_version));
    assert_eq!(
        executables.get(&python27_version),
        Some(&env_state.python27)
    );

    let python36_version = ExactVersion { major: 3, minor: 6 };
    assert!(executables.contains_key(&python27_version));
    assert_eq!(
        executables.get(&python36_version),
        Some(&env_state.python36)
    );

    let python37_version = ExactVersion { major: 3, minor: 7 };
    assert!(executables.contains_key(&python37_version));
    assert_eq!(
        executables.get(&python37_version),
        Some(&env_state.python37)
    );
}

#[test]
#[serial]
fn find_executable() {
    let env_state = EnvState::new();

    assert_eq!(
        python_launcher::find_executable(RequestedVersion::Any),
        Some(env_state.python37)
    );

    assert_eq!(
        python_launcher::find_executable(RequestedVersion::MajorOnly(2)),
        Some(env_state.python27)
    );

    assert_eq!(
        python_launcher::find_executable(RequestedVersion::Exact(3, 6)),
        Some(env_state.python36)
    );
}
