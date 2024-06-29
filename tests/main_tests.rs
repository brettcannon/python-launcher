#![allow(clippy::items_after_test_module)]

mod common;

use std::path::PathBuf;

use common::CurrentDir;

use python_launcher::{ExactVersion, RequestedVersion};

use assert_cmd::Command;
use predicates::str;
use test_case::test_case;

fn py_executable() -> Command {
    Command::cargo_bin("py").expect("binary 'py' not found")
}

#[test_case("-h"; "short")]
#[test_case("--help"; "long")]
fn help_flags(help_flag: &str) {
    let python = python_launcher::find_executable(RequestedVersion::Any)
        .expect("no Python executable found");
    let result = py_executable().arg(help_flag).assert();

    result
        .success()
        .stdout(str::contains(python.to_string_lossy()))
        .stderr(str::is_empty());
}

#[test]
fn list_output() {
    let pythons = python_launcher::all_executables();
    let mut result = py_executable().arg("--list").assert();

    result = result.success();

    for (version, path) in pythons.iter() {
        result = result
            .stdout(str::contains(version.to_string()))
            .stdout(str::contains(path.to_string_lossy()));
    }
}

#[test]
fn any_version() {
    let python = python_launcher::find_executable(RequestedVersion::Any)
        .expect("no Python executable found");
    let version = ExactVersion::from_path(&python).unwrap();
    let result = py_executable()
        .args(["-c", "import sys; print(sys.version)"])
        .assert();

    result
        .success()
        .stdout(str::starts_with(version.to_string()))
        .stderr(str::is_empty());
}

#[test]
fn major_version() {
    let python = python_launcher::find_executable(RequestedVersion::Any)
        .expect("no Python executable found");
    let version = ExactVersion::from_path(&python).unwrap();
    let version_flag = format!("-{}", version.major);
    let result = py_executable()
        .args([
            version_flag.as_str(),
            "-c",
            "import sys; print(sys.version)",
        ])
        .assert();

    result
        .success()
        .stdout(str::starts_with(version.to_string()))
        .stderr(str::is_empty());
}

#[test]
fn exact_version() {
    let python = python_launcher::find_executable(RequestedVersion::Any)
        .expect("no Python executable found");
    let version = ExactVersion::from_path(&python).unwrap();
    let version_flag = format!("-{version}");
    let result = py_executable()
        .args([
            version_flag.as_str(),
            "-c",
            "import sys; print(sys.version)",
        ])
        .assert();

    result
        .success()
        .stdout(str::starts_with(version.to_string()))
        .stderr(str::is_empty());
}

#[test]
fn logging_output() {
    let result = py_executable()
        .args(["-c", "pass"])
        .env("PYLAUNCH_DEBUG", "1")
        .assert();

    result
        .success()
        .stdout(str::is_empty())
        .stderr(str::contains("Executing"));
}

#[test_case("3."; "invalid version specifier")]
#[test_case("0.1"; "non-existent version")]
fn version_failure(version: &str) {
    let flag = format!("-{version}");
    let result = py_executable().arg(flag).assert();

    result.failure().stdout(str::is_empty());
}

#[test]
fn nonexistent_activated_virtual_env_dir() {
    let result = py_executable()
        .env("VIRTUAL_ENV", "this does not exist")
        .assert();

    result.failure().stdout(str::is_empty());
}

#[test]
fn empty_activated_virtual_env() {
    let cwd = CurrentDir::new();
    let result = py_executable()
        .env("VIRTUAL_ENV", cwd.dir.path().as_os_str())
        .assert();

    result.failure();
}

#[test]
fn no_executable() {
    let cwd = CurrentDir::new();
    let cwd_name = cwd.dir.path().as_os_str();
    let fake_python = PathBuf::from("python0.1");
    common::touch_file(fake_python);
    let result = py_executable().env("PATH", cwd_name).assert();

    result.failure();
}
