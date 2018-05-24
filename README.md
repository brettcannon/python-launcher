# The Python Launcher for UNIX

An implementation of the `py` command for UNIX-based platforms.

The goal is to have `py` become the cross-platform command that all Python users
use when executing a Python interpreter. Not only is it is short and to the
point, but it also provides a single command that documentation and such can use
which will work regardless of what operating system a user is on. Lastly, it
side-steps the "what should `python` point to" debate by clearly specifying that
upfront (i.e. the newest version of Python that happens to be installed).

# [PEP 397: Python launcher for Windows](https://www.python.org/dev/peps/pep-0397/) ([documentation](https://docs.python.org/3/using/windows.html#launcher))
- [Shebang line parsing](https://www.python.org/dev/peps/pep-0397/#shebang-line-parsing)
  - Only the [first argument if it's a file and doesn't start with `-`](https://www.python.org/dev/peps/pep-0397/#command-line-handling)
  - Not necessary, but nice to have
- [Configuration files](https://www.python.org/dev/peps/pep-0397/#configuration-file)
  - [Customized commands](https://www.python.org/dev/peps/pep-0397/#customized-commands)
  - Want a better format like TOML?
  - Want to use `Pipfile` and its `python_version` field?
  - Probably want a way to override things, e.g. wanting a framework build on macOS somehow
    - Aliasing? E.g. `2.7-framework=/System/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python`?
    - Just provide a way to specify a specific interpreter for a specific version? E.g. `2.7=/System/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python`
- [Specifying the version](https://www.python.org/dev/peps/pep-0397/#python-version-qualifiers)
  - Unspecified defaults to `2` is no longer true?
  - [On the commmand-line](https://www.python.org/dev/peps/pep-0397/#command-line-handling)
  - Environment variables
- `py -0`
  - Output well-formatted JSON to start in order for it to be consumable?
  - Output column format like `pip list`?

# Search order

## `py -3.6`

1. Search `PATH` for `python3.6`

## `py -3`
1. Check `PY_PYTHON3`; if set to e.g. `3.6`, run as `py -3.6` (error to only set as `3`?)
1. Search `PATH` for `python3.X`
1. Use executable with largest `X`

## `py`

1. Check `PY_PYTHON`; if set to e.g. `3`, run as `py -3`; if set to e.g. `3.6`, run as `py -3.6`
1. Search `PATH` for `pythonX.Y`
1. Use executable with largest `X`, then largest `Y`


# Implementation notes

[std::fs::read_dir](https://doc.rust-lang.org/std/fs/fn.read_dir.html)

[std::vec::Vec::sort_by_key](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by_key)

[execv](https://linux.die.net/man/3/execv):
1. [nix::unistd::exec](https://docs.rs/nix/0.9.0/nix/unistd/fn.execv.html)
1. [libc::execv](https://docs.rs/libc/0.2.40/libc/fn.execv.html)
