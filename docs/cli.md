# CLI

In general, the `py` command passes on its arguments to the selected Python interpreter. Below are the Launcher-specific arguments that are supported.

## Arguments

### `-[X]`

Specifies the major Python version desired, e.g. `-3`. Specifying such a restriction causes the equivalent [`PY_PYTHON[X]`](#py_pythonx) environment variable to be used if set.

See the [overview](index.md#on-the-command-line) for more details.

### `-[X.Y]`

Specifies the major and minor Python version desired, e.g. `-3.6` for Python 3.6.

See the [overview](index.md#on-the-command-line) for more details.

### `--list`

Lists all Python interpreters found on the `PATH` environment variable.

## Environment variables

### `PY_PYTHON`

Specifies a version restriction when none is specified on the command line, i.e. `py` is used. This is useful for setting the default Python version to always use.

See the [overview](index.md#environment-variables) for more details.

### `PY_PYTHON[X]`

Specifies the version restriction when the equivalent major Python version is specified on the command line, e.g. `-3` causes `PY_PYTHON3` to be used if set.

See the [overview](index.md#environment-variables) for more details.

### `PYLAUNCH_DEBUG`

When set, causes the Python Launcher to print out information about its interprter search to stderr.
