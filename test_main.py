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
def py(monkeypatch):
    """Provide a convenience function for calling the Python launcher.

    The function has a 'path' attribute for a pathlib.Path object pointing
    at where the Python launcher is located.

    The critical environment variables which can influence the execution of
    the Python launcher are set to a known good state. This includes setting
    PATH to a single directory of where the Python interpreter executing this
    file is located.
    """
    python_executable = pathlib.Path(sys.executable)
    monkeypatch.setenv("PATH", os.fspath(python_executable.parent))
    try:
        monkeypatch.delenv("VIRTUAL_ENV")
    except KeyError:
        pass
    py_path = pathlib.Path(__file__).parent / "target" / "debug" / "py"

    def call_py(*args):
        call = [py_path]
        call.extend(args)
        return subprocess.run(call, capture_output=True, text=True)

    call_py.path = py_path
    yield call_py


@pytest.mark.parametrize("flag", ["--help", "-h"])
def test_help(py, flag):
    call = py(flag)
    assert not call.returncode
    assert os.fspath(py.path) in call.stdout
    assert sys.executable in call.stdout


def test_list(py):
    call = py("--list")
    assert not call.returncode
    assert sys.executable in call.stdout
    assert ".".join(map(str, sys.version_info[:2])) in call.stdout


def test_execute(py):
    # Don't use sys.executable as symlinks and such make it hard to get an
    # easy comparison.
    call = py("-c" "import sys; print(sys.version)")
    assert not call.returncode
    assert call.stdout.strip() == sys.version


if __name__ == "__main__":
    pytest.main()
