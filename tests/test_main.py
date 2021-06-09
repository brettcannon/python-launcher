"""Perform integration/main tests on the Python launcher.

Due to the fact that Rust's testing infrastructure doesn't allow for direct
testing of main.rs, this file executes a debug build of the Python launcher to
exercise main.rs. All code contained elsewhere in this project is expected to
be tested using Rust code.
"""

import os
import pathlib
import subprocess
import sys

import pytest


@pytest.fixture
def py(monkeypatch, tmp_path):
    """Provide a convenience function for calling the Python launcher.

    The function has a 'path' attribute for a pathlib.Path object pointing
    at where the Python launcher is located.

    The function has a 'python_executable' attribute for a pathlib.Path
    object pointing to a temporary symlink to the virtual environments'
    python executable.

    The critical environment variables which can influence the execution of
    the Python launcher are set to a known good state. This includes setting
    PATH to a single directory in which we create a symlink named with major
    and minor versions in the name.
    """
    symlink_name = f"python{sys.version_info.major}.{sys.version_info.minor}"
    python_executable = tmp_path / symlink_name
    os.symlink(sys.executable, python_executable)

    monkeypatch.delenv("PYLAUNCH_DEBUG", raising=False)
    monkeypatch.setenv("PATH", os.fspath(tmp_path))
    monkeypatch.delenv("VIRTUAL_ENV", raising=False)
    py_path = pathlib.Path(__file__).parent.parent / "target" / "debug" / "py"

    def call_py(*args, debug=False):
        call = [py_path]
        call.extend(args)
        env = os.environ.copy()
        if debug:
            env["PYLAUNCH_DEBUG"] = "1"
        print(call, env)
        return subprocess.run(call, capture_output=True, text=True, env=env)

    call_py.path = py_path
    call_py.python_executable = python_executable
    yield call_py


@pytest.mark.parametrize("flag", ["--help", "-h"])
def test_help(py, flag):
    call = py(flag)
    assert not call.returncode
    assert os.fspath(py.path) in call.stdout
    assert os.fspath(py.python_executable) in call.stdout
    assert not call.stderr


def test_list(py):
    call = py("--list")
    assert not call.returncode
    assert os.fspath(py.python_executable) in call.stdout
    assert ".".join(map(str, sys.version_info[:2])) in call.stdout
    assert not call.stderr


@pytest.mark.parametrize(
    "python_version",
    [None, f"-{sys.version_info[0]}", f"-{sys.version_info[0]}.{sys.version_info[1]}"],
)
def test_execute(py, python_version):
    # Don't use sys.executable as symlinks and such make it hard to get an
    # easy comparison.
    args = ["-c" "import sys; print(sys.version)"]
    if python_version:
        args.insert(0, python_version)
    call = py(*args)
    assert not call.returncode
    assert call.stdout.strip() == sys.version
    assert not call.stderr


class TestExitCode:
    def call_failed(self, call):
        assert call.returncode
        assert call.stderr

    def test_malformed_version(self, py):
        self.call_failed(py("-3."))

    def test_nonexistent_version(self, py):
        self.call_failed(py("-0.9"))

    def test_unexecutable_file(self, py, tmp_path, monkeypatch):
        version = "0.1"
        not_executable = tmp_path / f"python{version}"
        not_executable.touch()
        monkeypatch.setenv("PATH", os.fspath(tmp_path), prepend=os.pathsep)

        self.call_failed(py(f"-{version}"))

    def test_file_does_not_exist(self, py, monkeypatch):
        bad_venv_path = "this_path_does_not_exist"
        assert not os.path.exists(bad_venv_path)
        monkeypatch.setenv("VIRTUAL_ENV", bad_venv_path)

        self.call_failed(py())

    def test_directory(self, py, tmp_path, monkeypatch):
        dir_path = tmp_path / "bin" / "python"
        monkeypatch.setenv("VIRTUAL_ENV", os.fspath(tmp_path))
        self.call_failed(py())


def test_PYLAUNCH_DEBUG(py):
    call = py("-c", "pass", debug=True)
    assert not call.returncode
    assert call.stderr


if __name__ == "__main__":
    pytest.main()
