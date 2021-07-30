mod common;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use serial_test_derive::serial;

use python_launcher::cli;
use python_launcher::cli::Action;
use python_launcher::Error;
use python_launcher::RequestedVersion;

use common::{EnvState, EnvVarState};

#[test]
#[serial]
fn from_main_help() {
    let env_state = EnvState::new();
    for flag in ["-h", "--help"].iter() {
        let launcher_path = "/path/to/py";

        match Action::from_main(&[launcher_path.to_string(), (*flag).to_string()]) {
            Ok(Action::Help(message, python_path)) => {
                assert!(message.contains(launcher_path));
                assert_eq!(env_state.python37, python_path);
                assert!(message.contains(python_path.to_str().unwrap()));
            }
            _ => panic!("{:?} flag did not return Action::Help", flag),
        }
    }
}

#[test]
#[serial]
fn from_main_help_missing_interpreter() {
    let _state = EnvVarState::empty();
    for flag in ["-h", "--help"].iter() {
        let launcher_path = "/path/to/py";
        let help = Action::from_main(&[launcher_path.to_string(), (*flag).to_string()]);
        assert_eq!(
            help,
            Err(crate::Error::NoExecutableFound(RequestedVersion::Any))
        );
    }
}

#[test]
#[serial]
fn from_main_list() {
    let env_state = EnvState::new();

    match Action::from_main(&["/path/to/py".to_string(), "--list".to_string()]) {
        Ok(Action::List(output)) => {
            assert!(output.contains(env_state.python27.to_str().unwrap()));
            assert!(output.contains(env_state.python36.to_str().unwrap()));
            assert!(output.contains(env_state.python37.to_str().unwrap()));
        }
        _ => panic!("'--list' did not return Action::List"),
    }
}

#[test]
#[serial]
fn from_main_by_flag() {
    let _working_dir = common::CurrentDir::new();
    let env_state = common::EnvState::new();
    let launcher_location = "/path/to/py".to_string();
    let no_argv = Action::from_main(&[launcher_location.clone()]);

    match no_argv {
        Ok(Action::Execute {
            launcher_path,
            executable,
            args,
        }) => {
            assert_eq!(PathBuf::from(launcher_location.clone()), launcher_path);
            assert_eq!(executable, env_state.python37);
            assert_eq!(args.len(), 0);
        }
        Ok(Action::Help(_, _)) => panic!("Got back help"),
        Ok(Action::List(_)) => panic!("Got back a list of executables"),
        Err(error) => panic!("No executable found in default case: {:?}", error),
    }

    match Action::from_main(&[launcher_location.clone(), "-2".to_string()]) {
        Ok(Action::Execute {
            launcher_path,
            executable,
            args,
        }) => {
            assert_eq!(PathBuf::from(launcher_location.clone()), launcher_path);
            assert_eq!(executable, env_state.python27);
            assert_eq!(args.len(), 0);
        }
        _ => panic!("No executable found in `-3` case"),
    }

    match Action::from_main(&[launcher_location.clone(), "-3.6".to_string()]) {
        Ok(Action::Execute {
            launcher_path,
            executable,
            args,
        }) => {
            assert_eq!(PathBuf::from(launcher_location.clone()), launcher_path);
            assert_eq!(executable, env_state.python36);
            assert_eq!(args.len(), 0);
        }
        _ => panic!("No executable found in `-3.6` case"),
    }

    match Action::from_main(&[
        launcher_location.clone(),
        "-3.6".to_string(),
        "-I".to_string(),
    ]) {
        Ok(Action::Execute {
            launcher_path,
            executable,
            args,
        }) => {
            assert_eq!(PathBuf::from(launcher_location), launcher_path);
            assert_eq!(executable, env_state.python36);
            assert_eq!(args, ["-I".to_string()]);
        }
        _ => panic!("No executable found in `-3.6` case"),
    }
}

#[test]
#[serial]
fn from_main_activated_virtual_env() {
    let venv_path = "/path/to/venv";
    let mut env_state = common::EnvState::new();
    env_state.env_vars.change("VIRTUAL_ENV", Some(venv_path));

    match Action::from_main(&["/path/to/py".to_string()]) {
        Ok(Action::Execute { executable, .. }) => {
            let mut expected = PathBuf::from(venv_path);
            expected.push("bin");
            expected.push("python");
            assert_eq!(executable, expected);
        }
        _ => panic!("No executable found in `VIRTUAL_ENV` case"),
    }

    // VIRTUAL_ENV gets ignored if any specific version is requested.
    match Action::from_main(&["/path/to/py".to_string(), "-3".to_string()]) {
        Ok(Action::Execute { executable, .. }) => {
            assert_eq!(executable, env_state.python37);
        }
        _ => panic!("No executable found in `VIRTUAL_ENV` case"),
    }
}

#[test]
#[serial]
fn from_main_default_cwd_venv_path() {
    let _working_dir = common::CurrentDir::new();
    let env_state = common::EnvState::new();
    let mut expected = PathBuf::new();
    expected.push(cli::DEFAULT_VENV_DIR);
    expected.push("bin");
    fs::create_dir_all(&expected).unwrap();
    expected.push("python");
    common::touch_file(expected.clone());

    match Action::from_main(&["/path/to/py".to_string()]) {
        Ok(Action::Execute { executable, .. }) => {
            assert_eq!(executable, expected.canonicalize().unwrap());
        }
        _ => panic!("No executable found in default virtual environment case"),
    }

    // VIRTUAL_ENV gets ignored if any specific version is requested.
    match Action::from_main(&["/path/to/py".to_string(), "-3".to_string()]) {
        Ok(Action::Execute { executable, .. }) => {
            assert_eq!(executable, env_state.python37);
        }
        _ => panic!("No executable found in default virtual environment case"),
    }
}

#[test]
#[serial]
fn from_main_default_parent_venv_path() {
    let working_dir = common::CurrentDir::new();
    let temp_dir = working_dir.dir.path().to_path_buf();
    let env_state = common::EnvState::new();
    let mut expected = temp_dir.clone();
    expected.push(cli::DEFAULT_VENV_DIR);
    expected.push("bin");
    fs::create_dir_all(&expected).unwrap();
    expected.push("python");
    common::touch_file(expected.clone());

    let subdir = temp_dir.join("subdir");
    fs::create_dir(&subdir).unwrap();
    env::set_current_dir(&subdir).unwrap();

    match Action::from_main(&["/path/to/py".to_string()]) {
        Ok(Action::Execute { executable, .. }) => {
            assert_eq!(executable, expected.canonicalize().unwrap());
        }
        _ => panic!("No executable found in default virtual environment case"),
    }

    // VIRTUAL_ENV gets ignored if any specific version is requested.
    match Action::from_main(&["/path/to/py".to_string(), "-3".to_string()]) {
        Ok(Action::Execute { executable, .. }) => {
            assert_eq!(executable, env_state.python37);
        }
        _ => panic!("No executable found in default virtual environment case"),
    }
}

#[test]
#[serial]
fn from_main_shebang() {
    let _working_dir = common::CurrentDir::new();
    let env_state = common::EnvState::new();
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("shebang.py");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "#! /usr/bin/env python2.7").unwrap();

    match Action::from_main(&[
        "/path/to/py".to_string(),
        file_path.to_str().unwrap().to_string(),
    ]) {
        Ok(Action::Execute { executable, .. }) => {
            assert_eq!(executable, env_state.python27);
        }
        _ => panic!("No executable found in shebang case"),
    }

    // Shebang checking only works for the first argument to avoid accidentally
    // reading from arguments to Python code itself.
    match Action::from_main(&[
        "/path/to/py".to_string(),
        "-m".to_string(),
        "my_app".to_string(),
        file_path.to_str().unwrap().to_string(),
    ]) {
        Ok(Action::Execute { executable, .. }) => {
            assert_eq!(executable, env_state.python37);
        }
        _ => panic!("No executable found in shebang case"),
    }
}

#[test]
#[serial]
fn from_main_env_var() {
    let _working_dir = common::CurrentDir::new();
    let mut env_state = common::EnvState::new();
    env_state.env_vars.change("PY_PYTHON", Some("3.6"));
    let launcher_location = "/path/to/py".to_string();

    match Action::from_main(&[launcher_location.clone()]) {
        Ok(Action::Execute {
            launcher_path,
            executable,
            args,
        }) => {
            assert_eq!(PathBuf::from(launcher_location.clone()), launcher_path);
            assert_eq!(executable, env_state.python36);
            assert_eq!(args.len(), 0);
        }
        _ => panic!("No executable found in PY_PYTHON case"),
    }

    env_state.env_vars.change("PY_PYTHON3", Some("3.6"));

    match Action::from_main(&[launcher_location.clone(), "-3".to_string()]) {
        Ok(Action::Execute {
            launcher_path,
            executable,
            args,
        }) => {
            assert_eq!(PathBuf::from(launcher_location.clone()), launcher_path);
            assert_eq!(executable, env_state.python36);
            assert_eq!(args.len(), 0);
        }
        _ => panic!("No executable found in PY_PYTHON3 case"),
    }

    env_state.env_vars.change("PY_PYTHON3", Some("3.8.10"));

    if Action::from_main(&[launcher_location, "-3".to_string()]).is_ok() {
        panic!("Invalid PY_PYTHON3 did not error out");
    }
}

#[test]
#[serial]
fn from_main_no_executable_found() {
    let _env_state = common::EnvState::new();
    assert_eq!(
        Action::from_main(&["/path/to/py".to_string(), "-42.13".to_string()]),
        Err(Error::NoExecutableFound(RequestedVersion::Exact(42, 13)))
    );
}
