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

use std::collections;
use std::env;

use python_launcher as py;

fn main() {
    println!("Args: {:?}", env::args());
    let mut requested_version = py::RequestedVersion::Any;
    // https://docs.python.org/3.8/using/windows.html#from-the-command-line
    // XXX shebang?
    // https://docs.python.org/3.8/using/windows.html#shebang-lines
    if env::args().len() > 1 {
        requested_version = match env::args().nth(1) {
            // XXX `-0`
            // XXX `-h`/`--help`
            Some(arg) => py::check_cli_arg(&arg),
            None => py::RequestedVersion::Any,
        };
    }
    println!("CLI version: {:?}", requested_version);

    let mut found_versions = collections::HashMap::new();
    for path in py::path_entries() {
        let all_contents = py::directory_contents(&path);
        for (version, path) in py::filter_python_executables(all_contents) {
            match version.matches(&requested_version) {
                py::VersionMatch::NotAtAll => continue,
                py::VersionMatch::Loosely => {
                    if !found_versions.contains_key(&version) && path.is_file() {
                        found_versions.insert(version, path);
                    }
                }
                py::VersionMatch::Exactly => {
                    if path.is_file() {
                        found_versions.insert(version, path);
                        break;
                    }
                }
            };
        }
    }

    println!("Found {:?}", found_versions);
    let chosen_path = py::choose_executable(&found_versions);
    println!("Chose {:?}", chosen_path);

    // XXX shebang
    // https://docs.python.org/3.8/using/windows.html#customizing-default-python-versions
    // XXX Environment variable (if appropriate)? `PY_PYTHON`, `PY_PYTHON{major}`
    // https://docs.python.org/3.8/using/windows.html#virtual-environments
    // XXX Virtual environment takes precedence when no version specified; `VIRTUAL_ENV`
    // XXX Config file?
    // https://docs.python.org/3.8/using/windows.html#diagnostics
    // XXX PYLAUNCH_DEBUG
}
