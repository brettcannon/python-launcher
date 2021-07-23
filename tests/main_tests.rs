use python_launcher::RequestedVersion;

use assert_cmd::Command;
use predicates::str;
use test_case::test_case;

fn py_executable() -> Command {
    Command::cargo_bin("py").unwrap()
}

#[test_case("-h"; "short")]
#[test_case("--help"; "long")]
fn help_flags(help_flag: &str) {
    let python = python_launcher::find_executable(RequestedVersion::Any)
        .expect("no Python executable found");
    let result = py_executable().arg(help_flag).assert();

    result
        .success()
        .stdout(str::contains(python.to_string_lossy()));
}

#[test]
fn list_output() {
    let python = python_launcher::find_executable(RequestedVersion::Any)
        .expect("no Python executable found");
    let result = py_executable().arg("--list").assert();

    result
        .success()
        .stdout(str::contains(python.to_string_lossy()));
}

#[test_case("3."; "invalid version specifier")]
#[test_case("0.1"; "non-existent version")]
fn version_failure(version: &str) {
    let flag = format!("-{}", version);
    let result = py_executable().arg(flag).assert();

    result.failure();
}

#[test]
fn invalid_activated_virtual_env() {
    let result = py_executable()
        .env("VIRTUAL_ENV", "this does not exist")
        .assert();

    result.failure();
}

#[test]
fn logging_output() {
    let result = py_executable()
        .args(&["-c", "pass"])
        .env("PYLAUNCH_DEBUG", "1")
        .assert();

    result.success().stderr(str::contains("Executing"));
}
