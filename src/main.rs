// https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html
// https://docs.python.org/3.8/using/windows.html#python-launcher-for-windows
// https://github.com/python/cpython/blob/master/PC/launcher.c

// `py -3.6`
// 1. Search `PATH` for `python3.6`

// `py -3`
// 1. Check `PY_PYTHON3` (no checks for sanity, e.g. `3` and `2.7` are acceptable)
// 1. Search `PATH` for `python3.X`
// 1. Use executable with largest `X`

// `py`
// 1. Check shebang
// 1. Check for virtual environment (and then `python`)
// 1. Check `PY_PYTHON`; if set to e.g. `3`, run as `py -3`; if set to e.g. `3.6`, run as `py -3.6`
// 1. Search `PATH` for `pythonX.Y`
// 1. Use executable with largest `X`, then largest `Y`

extern crate python_launcher;

use std::env;

use python_launcher as py;

fn main() {
    println!("Args: {:?}", env::args());
    let mut version = py::RequestedVersion::Any;
    // https://docs.python.org/3.8/using/windows.html#from-the-command-line
    // XXX shebang?
    // https://docs.python.org/3.8/using/windows.html#shebang-lines
    if env::args().len() > 1 {
        version = match env::args().nth(1) {
            // XXX `-0`
            // XXX `-h`/`--help`
            Some(arg) => py::check_cli_arg(arg),
            None => py::RequestedVersion::Any,
        };
    }
    println!("CLI version: {:?}", version);

    // XXX shebang
    // https://docs.python.org/3.8/using/windows.html#customizing-default-python-versions
    // XXX Environment variable (if appropriate)? `PY_PYTHON`, `PY_PYTHON{major}`
    // https://docs.python.org/3.8/using/windows.html#virtual-environments
    // XXX Virtual environment takes precedence when no version specified; `VIRTUAL_ENV`
    // XXX Config file?
    // https://docs.python.org/3.8/using/windows.html#diagnostics
    // XXX PYLAUNCH_DEBUG
}
