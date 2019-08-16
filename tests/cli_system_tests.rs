mod common;

use serial_test_derive::serial;

use python_launcher::cli::Action;

use common::EnvState;

#[test]
#[serial]
fn from_main_help() {
    let env_state = EnvState::new();
    for flag in ["-h", "--help"].iter() {
        let launcher_path = "/path/to/py";
        let help = Action::from_main(&[launcher_path.to_string(), flag.to_string()]);
        if let Ok(Action::Help(message, python_path)) = help {
            assert!(message.contains(launcher_path));
            assert_eq!(env_state.python37, python_path);
        } else {
            panic!("{:?} flag did not return Action::Help", flag);
        }
    }
}

#[test]
#[serial]
fn from_main_list() {
    // --list
}

#[test]
#[serial]
fn from_main_activated_virtual_env() {
    // VIRTUAL_ENV
}

#[test]
#[serial]
fn from_main_shebang() {
    // #! /usr/bin/python3
}

#[test]
#[serial]
fn from_main_env_var() {
    // PY_PYTHON
}

#[test]
#[serial]
fn from_main_no_executable_found() {
    // Err(crate::Error::NoExecutableFound(requested_version))
}

#[test]
#[serial]
fn from_main_execute() {
    // no argv
    // -3 argv
    // -3.7 argv
}
// XXX Test Action::from_main()
