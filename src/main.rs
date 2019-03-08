// https://docs.python.org/3.8/using/windows.html#python-launcher-for-windows
// https://github.com/python/cpython/blob/master/PC/launcher.c

use std::{env, ffi, fs, os::unix::ffi::OsStrExt, path, str::FromStr};

use nix::unistd;

use python_launcher as py;

fn main() {
    let mut action = py::action_from_args(env::args().collect::<Vec<String>>());
    let mut requested_version = py::RequestedVersion::Any;
    let mut chosen_path: Option<path::PathBuf> = None;

    if let py::Action::Execute { version, .. } = action {
        requested_version = version;
    }

    if requested_version == py::RequestedVersion::Any {
        if let venv_executable @ Some(..) = py::virtual_env() {
            chosen_path = venv_executable;
        } else if let py::Action::Execute { launcher, args, .. } = action.clone() {
            if !args.is_empty() {
                // Using the first argument because it's the simplest and sanest.
                // We can't use the last argument because that could actually be an argument to the
                // Python module being executed. This is the same reason we can't go searching for
                // the first/last file path that we find. The only safe way to get the file path
                // regardless of its position is to replicate Python's arg parsing and that's a
                // **lot** of work for little gain.
                if let Ok(open_file) = fs::File::open(&args[0]) {
                    if let Some(shebang) = py::find_shebang(open_file) {
                        if let Some((shebang_version, mut extra_args)) = py::split_shebang(&shebang)
                        {
                            extra_args.append(&mut args.clone());
                            action = py::Action::Execute {
                                launcher,
                                version: shebang_version,
                                args: extra_args,
                            };
                        }
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

    match &action {
        py::Action::Help(launcher_path) => {
            println!(
                "Python Launcher for UNIX {}
                \n\
                usage:\n\
                {} [launcher-args] [python-args] script [script-args]\n\
                \n\
                Launcher arguments:\n\
                \n\
                -X     : Launch the latest Python X version (e.g. `-3` for the latest Python 3)\n\
                -X.Y   : Launch the specified Python version (e.g. `-3.6` for Python 3.6)\n\
                \n\
                The following help text is from {}:\n\
                \n",
                env!("CARGO_PKG_VERSION"),
                launcher_path.to_string_lossy(),
                chosen_path.unwrap().to_string_lossy()
            );
        }
        py::Action::Execute { args, .. } => {
            if let Err(e) = run(&chosen_path.unwrap(), &args) {
                println!("{:?}", e);
            }
        }
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
