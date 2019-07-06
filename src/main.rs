// https://docs.python.org/3.8/using/windows.html#python-launcher-for-windows
// https://github.com/python/cpython/blob/master/PC/launcher.c

use std::{env, ffi::CString, os::unix::ffi::OsStrExt, path::Path};

use nix::unistd;

use python_launcher::cli;

// XXX Proper exit codes.
// XXX Write errors out to stderr.
fn main() {
    match cli::Action::from_main(&env::args().collect::<Vec<String>>()) {
        Ok(action) => match action {
            cli::Action::Help(message, executable) => {
                print!("{}", message);
                if let Err(message) = run(&executable, &["--help".to_string()]) {
                    println!("{}", message);
                }
            }
            cli::Action::List(output) => print!("{}", output),
            cli::Action::Execute {
                executable, args, ..
            } => {
                if let Err(message) = run(&executable, &args) {
                    println!("{}", message)
                }
            }
        },
        Err(message) => println!("{}", message),
    }
}

fn run(executable: &Path, args: &[String]) -> nix::Result<()> {
    let executable_as_cstring = CString::new(executable.as_os_str().as_bytes()).unwrap();
    let mut argv = vec![executable_as_cstring.clone()];
    argv.extend(args.iter().map(|arg| CString::new(arg.as_str()).unwrap()));

    unistd::execv(&executable_as_cstring, &argv).map(|_| ())
}
