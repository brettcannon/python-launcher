# Contribution Guide

[![CI](https://github.com/brettcannon/python-launcher/actions/workflows/main.yml/badge.svg?event=push)](https://github.com/brettcannon/python-launcher/actions/workflows/main.yml)
[![Security check](https://github.com/brettcannon/python-launcher/actions/workflows/security-check.yml/badge.svg)](https://github.com/brettcannon/python-launcher/actions/workflows/security-check.yml)
[![codecov](https://codecov.io/gh/brettcannon/python-launcher/branch/master/graph/badge.svg?token=s2ZuXJQPPd)](https://codecov.io/gh/brettcannon/python-launcher)

The Python Launcher is _mostly_ run as a typical Rust project. The only
potential differences is the automation tool used (for convenience) and using
Python to run tests for `src/main.rs`.

## Using doit for automation

We use [doit](https://pydoit.org/) as an automation tool. It's
[available on PyPI](https://pypi.org/project/doit/), so you can choose to
install it however you prefer to install Python tools.

If you choose to use doit (it's purely a nicety to avoid having to make sure to
run everything appropriately), do note it will create a virtual environment for you
in a `.venv` directory if one doesn't exist. So if you choose to install doit
into a virtual environment for just this project, having it created into `.venv`
will simplify things for you.

### Running the Python tests

Since projects with a Rust binary component
[can't write integration tests for `src/main.rs`](https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html#integration-tests-for-binary-crates),
this project relies on [pytest](https://pypi.org/project/pytest/) to test the
code in `src/main.rs` (which is kept to a reasonable minimum).

If you choose not to use doit to run the tests, you can do it manually with:

```shell
python -m pytest tests/
```

This assumes that `python` points to an environment with pytest installed.

If you created a virtual environment in `.venv` and installed the requirements
listed in `dev-requirements.txt` into it, you can actually use the Python
Launcher itself to run the tests on itself:

```shell
./target/debug/py -m pytest tests/
```

Or:

```shell
cargo run -- -m pytest tests/
```
