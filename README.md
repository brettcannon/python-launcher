# The Python Launcher for Unix

An implementation of the `py` command for Unix-based platforms
(with some potential experimentation for good measure 😉)

The goal is to have `py` become the cross-platform command that all Python users
use when executing a Python interpreter. By having a version-agnostic command
it side-steps the "what should the `python` command point to?" debate by
clearly specifying that upfront (i.e. the newest version of Python that can be
found). This also unifies the suggested command to document for launching
Python on both Windows as Unix as `py` which has existed as the preferred
[command on Windows](https://docs.python.org/3/using/windows.html#launcher) for
some time.

See the top section of `py --help` for instructions.

## Search order

Please note that while searching, the search for a Python version can become
more specific. This leads to a switch in the search algorithm to the one most
appropriate to the specificity of the version.

You can always run the Python Launcher with `PYVENV_DEBUG` set to some value
to have it output logging details of how it is performing its search.

### `py -3.6` (specific version)
1. Search `PATH` for `python3.6`

### `py -3` (loose/major version)
1. Check for the `PY_PYTHON3` environment variable, and if defined
   and not the empty string then use it as the specific version
   (e.g. `PY_PYTHON3=3.6`)
1. Search `PATH` for all instances of `python3.*`
1. Find the executable with the newest version number that comes earliest on
   `PATH`

### `py` (any version)
1. Use `${VIRTUAL_ENV}/bin/python` immediately if available
1. If the first argument is a file path ...
   1. Check for a shebang
   1. If shebang path starts with `/usr/bin/python`, `/usr/local/bin/python`,
      `/usr/bin/env python` or `python`, proceed based on the version found
      on that path
      (bare `python` is considered the equivalent of not specifying a
      Python version)
1. Check for the `PY_PYTHON` environment variable, and if defined then use it
   as the loose or specific version (e.g. `PY_PYTHON=3` or `PY_PYTHON=3.6`)
1. Search `PATH` for all instances of `python*.*`
1. Find the executable with the newest version that is earliest on `PATH`

## TODO

[Issues to finish to reach MVP](https://github.com/brettcannon/python-launcher/issues?q=is%3Aissue+is%3Aopen+label%3AMVP)

## Appendix

- [PEP 397: Python launcher for Windows](https://www.python.org/dev/peps/pep-0397/)
- [PEP 486: Make the Python Launcher aware of virtual environments](https://www.python.org/dev/peps/pep-0486/)
- Windows Launcher
  - [Documentation](https://docs.python.org/3/using/windows.html#launcher)
  - [Source](https://github.com/python/cpython/blob/master/PC/launcher.c)
