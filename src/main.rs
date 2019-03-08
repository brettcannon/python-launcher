// https://docs.python.org/3.8/using/windows.html#python-launcher-for-windows
// https://github.com/python/cpython/blob/master/PC/launcher.c

use std::{env, ffi, fs, os::unix::ffi::OsStrExt, path, str::FromStr};

use nix::unistd;

use python_launcher as py;

fn main() {
    match py::action_from_args(env::args().collect::<Vec<String>>()) {
        py::Action::Help(launcher_path) => help(&launcher_path),
        py::Action::Execute {
            launcher_path,
            version,
            args,
        } => execute(&launcher_path, version, &args),
    }
}

fn help(launcher_path: &path::Path) {
    let mut chosen_path: Option<path::PathBuf> = None;
    let mut requested_version = py::RequestedVersion::Any;
    if let venv_executable @ Some(..) = py::virtual_env() {
        chosen_path = venv_executable;
    } else if let Some(env_var) = requested_version.env_var() {
        if let Ok(env_var_value) = env::var(env_var) {
            if let Ok(env_requested_version) = py::RequestedVersion::from_str(&env_var_value) {
                requested_version = env_requested_version;
            }
        };
    }

    if chosen_path.is_none() {
        let found_versions = py::available_executables(requested_version);

        chosen_path = py::choose_executable(&found_versions);
    }

    if chosen_path.is_none() {
        println!("No suitable interpreter found for {:?}", requested_version);
        return;
    }

    let found_path = chosen_path.unwrap();

    println!(
        "Python Launcher for UNIX {}\n\
         \n\
         usage:\n\
         {} [launcher-args] [python-args] script [script-args]\n\
         \n\
         Launcher arguments:\n\
         \n\
         -h/--help : This output\n\
         -X        : Launch the latest Python X version (e.g. `-3` for the latest Python 3)\n\
         -X.Y      : Launch the specified Python version (e.g. `-3.6` for Python 3.6)\n\
         \n\
         The following help text is from {}:\n",
        env!("CARGO_PKG_VERSION"),
        launcher_path.to_string_lossy(),
        found_path.to_string_lossy()
    );

    if let Err(e) = run(&found_path, &["--help".to_string()]) {
        println!("{:?}", e);
    }
}

fn execute(_launcher: &path::PathBuf, version: py::RequestedVersion, original_args: &[String]) {
    let mut requested_version = version;
    let mut chosen_path: Option<path::PathBuf> = None;
    let mut args = original_args.to_owned();

    if requested_version == py::RequestedVersion::Any {
        if let venv_executable @ Some(..) = py::virtual_env() {
            chosen_path = venv_executable;
        } else if !args.is_empty() {
            // Using the first argument because it's the simplest and sanest.
            // We can't use the last argument because that could actually be an argument to the
            // Python module being executed. This is the same reason we can't go searching for
            // the first/last file path that we find. The only safe way to get the file path
            // regardless of its position is to replicate Python's arg parsing and that's a
            // **lot** of work for little gain.
            if let Ok(open_file) = fs::File::open(&args[0]) {
                if let Some(shebang) = py::find_shebang(open_file) {
                    if let Some((shebang_version, mut extra_args)) = py::split_shebang(&shebang) {
                        requested_version = shebang_version;
                        extra_args.append(&mut args.clone());
                        args = extra_args;
                    }
                }
            }
        }
    }

    if chosen_path.is_none() {
        if let Some(env_var) = requested_version.env_var() {
            if let Ok(env_var_value) = env::var(env_var) {
                if let Ok(env_requested_version) = py::RequestedVersion::from_str(&env_var_value) {
                    requested_version = env_requested_version;
                }
            };
        }

        let found_versions = py::available_executables(requested_version);

        chosen_path = py::choose_executable(&found_versions);
    }

    if chosen_path.is_none() {
        println!("No suitable interpreter found for {:?}", requested_version);
        return;
    }

    if let Err(e) = run(&chosen_path.unwrap(), &args) {
        println!("{:?}", e);
    }
}

fn run(executable: &path::Path, args: &[String]) -> nix::Result<()> {
    let executable_as_cstring = ffi::CString::new(executable.as_os_str().as_bytes()).unwrap();
    let mut argv = vec![executable_as_cstring.clone()];
    argv.extend(
        args.iter()
            .map(|arg| ffi::CString::new(arg.as_str()).unwrap()),
    );

    let result = unistd::execv(&executable_as_cstring, &argv);
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
