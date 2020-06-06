# The Python Launcher for UNIX

An implementation of the `py` command for UNIX-based platforms
(with some potential experimentation for good measure 😉)

The goal is to have `py` become the cross-platform command that all Python users
use when executing a Python interpreter. By having a version-agnostic command
it side-steps the "what should the `python` command point to?" debate by
clearly specifying that upfront (i.e. the newest version of Python that can be
found). This also unifies the suggested command to document for launching
Python on both Windows as UNIX as `py` which has existed as the preferred
[command on Windows](https://docs.python.org/3/using/windows.html#launcher) for
some time.

See the top section of `py --help` for instructions.

# Search order

Please note that while searching, the search for a Python version can become
more specific. This leads to a switch in the search algorithm to the one most
appropriate to the specificity of the version.

## `py -3.6` (specific version)
1. Search `PATH` for `python3.6`

## `py -3` (loose/major version)
1. Use the version found in the `PY_PYTHON3` environment variable if defined
   and not the empty string (e.g. `PY_PYTHON3=3.6`)
1. Search `PATH` for all instances of `python3.Y`
1. Find the executable with largest `Y` that earliest on `PATH`

## `py` (any/unknown version)
1. Use `${VIRTUAL_ENV}/bin/python` immediately if available
1. If the first argument is a file path ...
   1. Check for a shebang
   1. If executable starts with `/usr/bin/python`, `/usr/local/bin/python`,
      `/usr/bin/env python` or `python`, proceed based on the version found
      (bare `python` is considered the equivalent of not specifying a
      Python version)
1. Use the version found in the `PY_PYTHON` environment variable if defined
   (e.g. `PY_PYTHON=3` or `PY_PYTHON=3.6`)
1. Search `PATH` for all instances of `pythonX.Y`
1. Find the executable with the largest `X.Y` earliest on `PATH`

# TODO

[![CI](https://github.com/brettcannon/python-launcher/workflows/CI/badge.svg)](https://github.com/brettcannon/python-launcher/actions?query=workflow%3ACI)

**NOTE**: I am using this project to learn
[Rust](https://www.rust-lang.org/), so please don't be offended if I choose to
implement something myself instead of accepting a pull request that you submit.
(Pull requests to do something I have already implemented in a more idiomatic
fashion are very much appreciated, though.)

[PEP 397: Python launcher for Windows](https://www.python.org/dev/peps/pep-0397/) / [PEP 486: Make the Python Launcher aware of virtual environments](https://www.python.org/dev/peps/pep-0486/) ([documentation](https://docs.python.org/3/using/windows.html#launcher); [source](https://github.com/python/cpython/blob/master/PC/launcher.c))

Everything in **bold** is required to hit MVP.

## Functionality
1. Provide a `python_launcher` extension module
   - It will make the pipenv developers happy
   - Might need a rename to `pylauncher` or `pyfinder` to follow Python practices (if it
     isn't too much trouble)
1. Windows support
   - `PATH`
   - Windows Store (should be covered by `PATH` search, but need to make sure)
   - Registry
   - Bitness
1. [Configuration files](https://www.python.org/dev/peps/pep-0397/#configuration-file)
   (key thing to remember is should not get to the point that you're using this to alias
   specific interpreters, just making it easier to specify constraints on what kind of
   interpreter you need and then letting the launcher pick for you)
   - [Customized commands](https://www.python.org/dev/peps/pep-0397/#customized-commands)?
   - Want a better format like TOML?
   - Want a way to override/specify things, e.g. wanting a framework build on macOS?
     - Aliasing? E.g. `2.7-framework` for
       `/System/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python`?
     - Just provide a way to specify a specific interpreter for a specific version?
       E.g. `2.7` for
       `/System/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python`
     - What about implementations that don't install to e.g. `python3.7` like `pypy3`?
       - Need more than just being able to alias `pypy3` to its Python version?
   - How should the config file search work?
     - Pre-defined locations?
     - Walk up from current directory?
     - [XDG base directory specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)?
1. Replacement for `.venv/bin/python` (while keeping the `python` name)?
   - `VENV_REDIRECT` variant in `launcher.c`
   - Might need switch off CLI additions -- i.e. `-h`, `--list`, and version specifier support -- in this situation to make this work
   - Read `../pyvenv.cfg` and its [`home` key](https://docs.python.org/3/library/venv.html#creating-virtual-environments) to determine where to look for the Python executable
     - What if `home` has multiple Python executables installed? Might need to add an `executable` key to give full path to the creating interpreter.
   - Acts as a heavyweight "symlink" to the Python executable for the virtual environment
   - Speeds up environment creation by not having to copy over entire Python installation on     Windows (e.g. `.pyd` files)
   - [See/edit the `site` module](https://github.com/python/cpython/blob/master/Lib/site.py#L456) to gain ability to specify virtual environment location (while maintaining the invariant on how to detect virtual environments as outlined in the [`venv` module docs](https://docs.python.org/3/library/venv.html#module-venv))
1. Automatically detect `.venv/pyvenv.cfg` and use that (basically an implicit setting of `$VIRTUAL_ENV`)?
1. Use `OsString`/`OsStr` everywhere (versus now which is wherever it's easy w/ `path::Path`)?
   - Widest compatibility for people where they have undecodable paths
     (which is hopefully a very small minority)
   - Massive pain to make work (e.g. cannot easily convert to a `CString`)
1. `--json` flag for JSONified `--list` output?
   - Would make consuming data from the CLI easier (e.g. for [Nushell](https://www.nushell.sh/))

## Polish
1. **Make sure all [potential `panic!` points](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#shortcuts-for-panic-on-error-unwrap-and-expect) are rare enough to be acceptable**
1. Have `--list` somehow denote an activated virtual environment?
   * Python Launcher doesn't denote or list anything
   * Seems useful since that would take precedence if the user didn't specify a version
1. Man page?
1. [`PYLAUNCH_DEBUG`](https://docs.python.org/3.8/using/windows.html#diagnostics)? ([Rust logging info](https://rust-lang-nursery.github.io/cli-wg/tutorial/output.html#logging))
1. [Distribute binaries](https://rust-lang-nursery.github.io/cli-wg/tutorial/packaging.html#distributing-binaries)

## Maintainability
1. **Flesh out documentation**
   1. **CLI documentation**
   1. Flowchart of how the interpreter is selected?
   1. API documentation
1. **Consider dropping [`nix`](https://crates.io/crates/nix)** for a straight
   [`libc`](https://crates.io/crates/libc) dependency (to potentially make Debian
   packaging easier)
   - Otherwise update `nix` to the latest version

Output from `py --list` on Windows:
```
Installed Pythons found by C:\WINDOWS\py.exe Launcher for Windows
 -3.8-64 *
 -3.7-64
 -3.6-64
 -2.7-64
 -2.7-64
 ```
