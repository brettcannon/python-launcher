# The Python Launcher for UNIX

An implementation of the `py` command for UNIX-based platforms.

The goal is to have `py` become the cross-platform command that all Python users
use when executing a Python interpreter. By having a version-agnostic command
it side-steps the "what should the `python` command point to?" debate by
clearly specifying that upfront (i.e. the newest version of Python that is
installed). This also unifies the suggested command to document for launching
Python on both Windows as UNIX as `py` which has existed as the preferred
[command on Windows](https://docs.python.org/3/using/windows.html#launcher) for
some time.

See the top of `py --help` for instructions.

# Search order

Please note that while searching, the search for a Python version can become
more specific. This leads to a switch in the search algorithm to the one most
appropriate to the specificity of the version.

## `py -3.6` (specific version)
1. Search `PATH` for `python3.6`

## `py -3` (loose/major version)
1. Use the version found in the `PY_PYTHON3` environment variable if defined
   (e.g. `PY_PYTHON3=3.6`)
1. Search `PATH` for all instances of `python3.Y`
1. Find the executable with largest `Y` that earliest on `PATH`

## `py` (any/unknown version)
1. Use `${VIRTUAL_ENV}/bin/python` immediately if available
1. If the first argument is a file path ...
   1. Check for a shebang
   1. If executable starts with `/usr/bin/python`, `/usr/local/bin/python`,
      `/usr/bin/env python` or `python`, proceed based on the version found
      (bare `python` is considered `python2` for backwards-compatibility)
1. Use the version found in the `PY_PYTHON` environment variable if defined
   (e.g. `PY_PYTHON=3` or `PY_PYTHON=3.6`)
1. Search `PATH` for all instances of `pythonX.Y`
1. Find the executable with the largest `X.Y` earliest on `PATH`

# TODO

**NOTE**: I am using this project to learn
[Rust](https://www.rust-lang.org/), so please don't be offended if I choose to
implement something myself instead of accepting a pull request that you submit.
(Pull requests to do something I have already implemented in a more idiomatic
fashion are very much appreciated, though.)

[PEP 397: Python launcher for Windows](https://www.python.org/dev/peps/pep-0397/) ([documentation](https://docs.python.org/3/using/windows.html#launcher))

## Functionality
1. Keep environment variable naming?
  - No other Python env vars are prefixed with `PY_` (it's always `PYTHON`)
  - The `PY_PYTHON` aspect feels redundant
1. Consider dropping assumption that `python` in shebangs represent `python2`?
  - Linux distros are already starting to redefine `python` as `python3` in some spots
  - It's looking more and more likely that PEP 394 will support letting distros choose what `python` represents
1. [Configuration files](https://www.python.org/dev/peps/pep-0397/#configuration-file)
   (key thing to remember is should not get to the point that you're using this to alias
   specific interpreters, just making it easier to specify constraints on what kind of
   interpreter you need and then letting launcher pick for you)
   - [Customized commands](https://www.python.org/dev/peps/pep-0397/#customized-commands)?
   - Want a better format like TOML?
   - Want a way to override/specify things, e.g. wanting a framework build on macOS?
     - Aliasing? E.g. `2.7-framework` for `/System/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python`?
     - Just provide a way to specify a specific interpreter for a specific version? E.g. `2.7` for `/System/Library/Frameworks/Python.framework/Versions/2.7/Resources/Python.app/Contents/MacOS/Python`
     - What about implementations that don't install to e.g. `python3.7` like `pypy3`?
       - Need more than just being able to alias name to Python version?
   - How should config file search work?
     - Pre-defined locations?
     - Walk up from current directory?
     - [XDG base directory specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)?
1. Windows support
   - Registry
   - `PATH`
   - Windows Store (should be covered by `PATH` search, but need to make sure)
1. Read `../pyvenv.cfg` to resolve for `Any` version
   - Acts as a heavyweight "symlink" to the Python executable for the virtual environment
   - Speeds up environment creation by not having to copy over entire Python installation on Windows (e.g. `.pyd` files)
1. Provide a `python_launcher` package (it will make the pipenv developers happy 😃; might need a rename to `pylauncher` or `pyfinder` to follow Python practices)
1. Use `OsString`/`OsStr` everywhere (versus now which is wherever it's easy w/ `path::Path`)?
   - Widest compatibility for people where they have undecodable paths
     (which is hopefully a very small minority)
   - Massive pain to make work (e.g. cannot easily convert to a `CString`)

## Polish
1. Have `--list` somehow denote activated virtual environment?
   * What does the Windows launcher do in this case?
   * Slight pain as there's no way to no the version of Python w/o executing it to query
     its version as virtual environments has no `major.minor`-named executable
1. Provide a helpful error message based on requested version when no interpreter found
1. Start using [`human-panic`](https://github.com/rust-clique/human-panic)
1. Man page?
1. [`PYLAUNCH_DEBUG`](https://docs.python.org/3.8/using/windows.html#diagnostics)?

## Maintainability
1. Pare down public exposure of functions
1. Consider having functions take arguments instead of querying environment
   (i.e. don't directly query `PATH`, `VIRTUAL_ENV` to ease testability)
   - Can provide functions or constants to minimize typos in querying environment
1. Go through functions to adjust for returning `Option` versus `Result`
     (e.g. `split_shebang(),`version_from_flag()`, `choose_executable()`)
1. Make sure everything is tested
1. Flesh out documentation (and include examples as appropriate for even more testing)
1. Consider dropping [`nix`](https://crates.io/crates/nix) dependency for a straight
   [`libc`](https://crates.io/crates/libc) dependency (to potentially make Debian
   packaging easier)

