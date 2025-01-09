//! Providing a convenient way to launch `python`
//!
//! The binary is meant to make the `py` command your go-to command for
//! launching a Python interpreter while writing code. It does this by trying
//! to find the Python interpreter that you most likely want to use.
//!
//! # Examples
//!
//! Launch the newest version of Python installed.
//! ```text
//! > py
//! ```
//! It will also launch any activated virtual environments (as set by the
//! `$VIRTUAL_ENV` environment variable) or any virtual environment in a
//! `.venv` subdirectory in any of the current or parent directories.
//!
//! You can also launch a specific version of Python.
//! ```text
//! > py -3.6
//! ```
//!
//! If you only care about the major version of Python, you can specify only
//! that.
//! ```text
//! > py -3
//! ```
//!
//! # Important environment variables
//!
//! - `VIRTUAL_ENV`: an activated virtual environment.
//! - `PYLAUNCH_DEBUG`: turn on logging.
//! - `PY_PYTHON`: specify what Python version to use by default.
//! - `PY_PYTHON*`: specify the Python version to use by default for a major
//!   version (e.g. `PY_PYTHON3` for `-3`).

use std::{env, ffi::CString, os::unix::ffi::OsStrExt, path::Path};

use human_panic::Metadata;

use nix::errno::Errno;
use nix::unistd;

use python_launcher::cli;

fn main() {
    human_panic::setup_panic!(Metadata::new(
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION")
    )
    .authors(env!("CARGO_PKG_AUTHORS"))
    .homepage(env!("CARGO_PKG_REPOSITORY")));

    let log_level = if env::var_os("PYLAUNCH_DEBUG").is_some() {
        3
    } else {
        0
    };
    // - `error!` is for errors
    // - `info!` is to communicate what the launcher is doing/checking
    // - `debug!` is communicating about specific values
    stderrlog::new()
        .module(module_path!())
        .module("python_launcher")
        .show_level(false)
        .verbosity(log_level) // [error, warn, info, debug, trace]
        .init()
        .unwrap();

    match cli::Action::from_main(&env::args().collect::<Vec<String>>()) {
        Ok(action) => match action {
            cli::Action::Help(message, executable) => {
                print!("{message}");
                run(&executable, &["--help".to_string()])
                    .map_err(|message| log_exit(Errno::last_raw(), message))
                    .unwrap()
            }
            cli::Action::List(output) => print!("{output}"),
            cli::Action::Execute {
                executable, args, ..
            } => run(&executable, &args)
                .map_err(|message| log_exit(Errno::last_raw(), message))
                .unwrap(),
        },
        Err(message) => log_exit(message.exit_code(), message),
    }
}

fn log_exit(return_code: i32, message: impl std::error::Error) {
    log::error!("{message}");
    std::process::exit(return_code);
}

fn run(executable: &Path, args: &[String]) -> nix::Result<()> {
    let printable_executable = executable.display();
    if executable.is_file() {
        log::info!("Executing {printable_executable} with {args:?}");
    } else {
        log::error!("{printable_executable}: No such file");
        std::process::exit(1);
    }
    let executable_as_cstring = CString::new(executable.as_os_str().as_bytes()).unwrap();
    let mut argv = vec![executable_as_cstring.clone()];
    argv.extend(args.iter().map(|arg| CString::new(arg.as_str()).unwrap()));

    unistd::execv(&executable_as_cstring, &argv).map(|_| ())
}
