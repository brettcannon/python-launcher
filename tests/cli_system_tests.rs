mod common;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use serial_test_derive::serial;
use tempfile;

use python_launcher::cli::Action;
use python_launcher::Error;
use python_launcher::RequestedVersion;

use common::EnvState;

#[test]
#[serial]
fn from_main_help() {
    let env_state = EnvState::new();
    for flag in ["-h", "--help"].iter() {
        let launcher_path = "/path/to/py";
        let help = Action::from_main(&[launcher_path.to_string(), (*flag).to_string()]);
        if let Ok(Action::Help(message, python_path)) = help {
            assert!(message.contains(launcher_path));
            assert_eq!(env_state.python37, python_path);
            assert!(message.contains(python_path.to_str().unwrap()));
        } else {
            panic!("{:?} flag did not return Action::Help", flag);
        }
    }
}

#[test]
#[serial]
fn from_main_list() {
    let env_state = EnvState::new();
    let list = Action::from_main(&["/path/to/py".to_string(), "--list".to_string()]);
    if let Ok(Action::List(output)) = list {
        assert!(output.contains(env_state.python27.to_str().unwrap()));
        assert!(output.contains(env_state.python36.to_str().unwrap()));
        assert!(output.contains(env_state.python37.to_str().unwrap()));
    } else {
        panic!("'--list' did not return Action::List");
    }
}

#[test]
#[serial]
fn from_main_by_flag() {
    let env_state = common::EnvState::new();
    let launcher_location = "/path/to/py".to_string();
    let no_argv = Action::from_main(&[launcher_location.clone()]);
    if let Ok(Action::Execute {
        launcher_path,
        executable,
        args,
    }) = no_argv
    {
        assert_eq!(PathBuf::from(launcher_location.clone()), launcher_path);
        assert_eq!(executable, env_state.python37);
        assert_eq!(args.len(), 0);
    } else {
        panic!("No executable found in default case");
    }

    let argv_2 = Action::from_main(&[launcher_location.clone(), "-2".to_string()]);
    if let Ok(Action::Execute {
        launcher_path,
        executable,
        args,
    }) = argv_2
    {
        assert_eq!(PathBuf::from(launcher_location.clone()), launcher_path);
        assert_eq!(executable, env_state.python27);
        assert_eq!(args.len(), 0);
    } else {
        panic!("No executable found in `-3` case");
    }

    let argv_36 = Action::from_main(&[launcher_location.clone(), "-3.6".to_string()]);
    if let Ok(Action::Execute {
        launcher_path,
        executable,
        args,
    }) = argv_36
    {
        assert_eq!(PathBuf::from(launcher_location.clone()), launcher_path);
        assert_eq!(executable, env_state.python36);
        assert_eq!(args.len(), 0);
    } else {
        panic!("No executable found in `-3.6` case");
    }

    let argv_36_args = Action::from_main(&[
        launcher_location.clone(),
        "-3.6".to_string(),
        "-I".to_string(),
    ]);
    if let Ok(Action::Execute {
        launcher_path,
        executable,
        args,
    }) = argv_36_args
    {
        assert_eq!(PathBuf::from(launcher_location), launcher_path);
        assert_eq!(executable, env_state.python36);
        assert_eq!(args, ["-I".to_string()]);
    } else {
        panic!("No executable found in `-3.6` case");
    }
}

#[test]
#[serial]
fn from_main_activated_virtual_env() {
    let venv_path = "/path/to/venv";
    let mut env_state = common::EnvState::new();
    env_state.env_vars.change("VIRTUAL_ENV", Some(&venv_path));
    let venv_executable = Action::from_main(&["/path/to/py".to_string()]);
    if let Ok(Action::Execute { executable, .. }) = venv_executable {
        let mut expected = PathBuf::from(venv_path);
        expected.push("bin");
        expected.push("python");
        assert_eq!(executable, expected);
    } else {
        panic!("No executable found in `VIRTUAL_ENV` case");
    }

    // VIRTUAL_ENV gets ignored if any specific version is requested.
    let ignore_virtual_env = Action::from_main(&["/path/to/py".to_string(), "-3".to_string()]);
    if let Ok(Action::Execute { executable, .. }) = ignore_virtual_env {
        assert_eq!(executable, env_state.python37);
    } else {
        panic!("No executable found in `VIRTUAL_ENV` case");
    }
}

#[test]
#[serial]
fn from_main_shebang() {
    let env_state = common::EnvState::new();
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("shebang.py");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "#! /usr/bin/env python2.7").unwrap();
    let shebang_choice = Action::from_main(&[
        "/path/to/py".to_string(),
        file_path.to_str().unwrap().to_string(),
    ]);
    if let Ok(Action::Execute { executable, .. }) = shebang_choice {
        assert_eq!(executable, env_state.python27);
    } else {
        panic!("No executable found in shebang case");
    }

    // Shebang checking only works for the first argument to avoid accidentally
    // reading from arguments to Python code itself.
    let skip_shebang = Action::from_main(&[
        "/path/to/py".to_string(),
        "-m".to_string(),
        "my_app".to_string(),
        file_path.to_str().unwrap().to_string(),
    ]);
    if let Ok(Action::Execute { executable, .. }) = skip_shebang {
        assert_eq!(executable, env_state.python37);
    } else {
        panic!("No executable found in shebang case");
    }
}

#[test]
#[serial]
fn from_main_env_var() {
    let mut env_state = common::EnvState::new();
    env_state.env_vars.change("PY_PYTHON", Some("3.6"));
    let launcher_location = "/path/to/py".to_string();
    let py_python = Action::from_main(&[launcher_location.clone()]);
    if let Ok(Action::Execute {
        launcher_path,
        executable,
        args,
    }) = py_python
    {
        assert_eq!(PathBuf::from(launcher_location.clone()), launcher_path);
        assert_eq!(executable, env_state.python36);
        assert_eq!(args.len(), 0);
    } else {
        panic!("No executable found in PY_PYTHON case");
    }

    env_state.env_vars.change("PY_PYTHON3", Some("3.6"));
    let py_python3 = Action::from_main(&[launcher_location.clone(), "-3".to_string()]);
    if let Ok(Action::Execute {
        launcher_path,
        executable,
        args,
    }) = py_python3
    {
        assert_eq!(PathBuf::from(launcher_location), launcher_path);
        assert_eq!(executable, env_state.python36);
        assert_eq!(args.len(), 0);
    } else {
        panic!("No executable found in PY_PYTHON3 case");
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
