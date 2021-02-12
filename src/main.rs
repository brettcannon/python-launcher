// https://docs.python.org/3.8/using/windows.html#python-launcher-for-windows
// https://github.com/python/cpython/blob/master/PC/launcher.c

use std::{env, ffi::CString, os::unix::ffi::OsStrExt, path::Path};

use nix::unistd;

use python_launcher::cli;

#[cfg(not(tarpaulin_include))]
fn main() {
    human_panic::setup_panic!(Metadata {
        name: env!("CARGO_PKG_DESCRIPTION").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        authors: env!("CARGO_PKG_AUTHORS").into(),
        homepage: env!("CARGO_PKG_REPOSITORY").into(),
    });

    let log_level = if env::var_os("PYLAUNCH_DEBUG").is_some() {
        3
    } else {
        0
    };
    /*
    - `error!` is for errors
    - `info!` is to communicate what the launcher is doing/checking
    - `debug!` is communicating about specific values
    */
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
                print!("{}", message);
                run(&executable, &["--help".to_string()])
                    .map_err(|message| log_exit(nix::errno::errno(), message))
                    .unwrap()
            }
            cli::Action::List(output) => print!("{}", output),
            cli::Action::Execute {
                executable, args, ..
            } => run(&executable, &args)
                .map_err(|message| log_exit(nix::errno::errno(), message))
                .unwrap(),
        },
        Err(message) => log_exit(message.exit_code(), message),
    }
}

#[cfg(not(tarpaulin_include))]
fn log_exit(return_code: i32, message: impl std::error::Error) {
    log::error!("{}", message);
    std::process::exit(return_code);
}

#[cfg(not(tarpaulin_include))]
fn run(executable: &Path, args: &[String]) -> nix::Result<()> {
    if executable.is_file() {
        log::info!("Executing {} with {:?}", executable.display(), args);
    } else {
        log::error!("{}: No such file", executable.display());
        std::process::exit(1);
    }
    let executable_as_cstring = CString::new(executable.as_os_str().as_bytes()).unwrap();
    let mut argv = vec![executable_as_cstring.clone()];
    argv.extend(args.iter().map(|arg| CString::new(arg.as_str()).unwrap()));

    unistd::execv(&executable_as_cstring, &argv).map(|_| ())
}
