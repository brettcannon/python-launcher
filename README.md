# The Python Launcher for UNIX

An implementation of the `py` command for UNIX-based platforms.

The goal is to have `py` become the cross-platform command that all Python users
use when executing a Python interpreter. Not only is it short and to the
point, but it also provides a single command that documentation can use in
examples which will work regardless of what operating system a user is on.
Lastly, it side-steps the "what should the `python` command point to?" debate by
clearly specifying that upfront (i.e. the newest version of Python that is
installed).

# Search order

## `py -3.6`
1. Search `PATH` for `python3.6`

## `py -3`
1. Use the `PY_PYTHON3` environment variable if defined
   (e.g. `PY_PYTHON3=3.6`)
1. Search `PATH` for all instances of `python3.Y`
1. Find the executable with largest `Y`

## `py`
1. Use the `PY_PYTHON` environment variable if defined
   (e.g. `PY_PYTHON=3`)
   version requested and search accordingly
1. Search `PATH` for all instances of `pythonX.Y`
1. Find the executable with largest `X.Y` version

# TODO

**NOTE**: I am using this project to learn
[Rust](https://www.rust-lang.org/), so please don't be offended if I choose to
implement something myself instead of accepting a pull request that you submit.
(Pull requests to do something I have already implemented in a more idiomatic
fashion are very much appreciated, though.)

[PEP 397: Python launcher for Windows](https://www.python.org/dev/peps/pep-0397/) ([documentation](https://docs.python.org/3/using/windows.html#launcher))

- [Virtual environment takes precedence when no version specified](https://docs.python.org/3.8/using/windows.html#virtual-environments) (`VIRTUAL_ENV`)
- [Shebang line parsing](https://www.python.org/dev/peps/pep-0397/#shebang-line-parsing)
  - Only the [first argument if it's a file and doesn't start with `-`](https://www.python.org/dev/peps/pep-0397/#command-line-handling)
  - Not necessary, but nice to have
- [`PYLAUNCH_DEBUG`](https://docs.python.org/3.8/using/windows.html#diagnostics)
- `py -0`
  - Output well-formatted JSON to start in order for it to be consumable?
  - Output column format like `pip list`?
- `py -h` emits its own help before continuing on to call `python`
- [Configuration files](https://www.python.org/dev/peps/pep-0397/#configuration-file)
  - [Customized commands](https://www.python.org/dev/peps/pep-0397/#customized-commands)
  - Want a better format like TOML?
  - Want to use `Pipfile` and its `python_version` field?
  - Probably want a way to override things, e.g. wanting a framework build on macOS somehow
    - Aliasing? E.g. `2.7-framework=/System/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python`?
    - Just provide a way to specify a specific interpreter for a specific version? E.g. `2.7=/System/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python`
