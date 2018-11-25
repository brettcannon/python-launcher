// https://docs.python.org/3.8/using/windows.html#python-launcher-for-windows
// https://github.com/python/cpython/blob/master/PC/launcher.c

extern crate nix;
extern crate python_launcher;

use std::{collections, env, ffi, fs, os::unix::ffi::OsStrExt, path};

use nix::unistd;

use python_launcher as py;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    let launcher_path = args.remove(0); // Strip the path to this executable.
    let mut requested_version = py::RequestedVersion::Any;
    // TODO: Refactor version detection code so this can happen as part of argument parsing
    //       instead of as this early-on check and acts as state.
    let help = args.len() == 1 && (args[0] == "-h" || args[0] == "--help");

    if !help && !args.is_empty() {
        if args[0].starts_with('-') {
            if let Some(version) = py::version_from_flag(&args[0]) {
                requested_version = version;
                args.remove(0);
            }
        } else if let Ok(open_file) = fs::File::open(&args[0]) {
            if let Some(shebang) = py::find_shebang(open_file) {
                if let Some((shebang_version, mut extra_args)) = py::split_shebang(&shebang) {
                    requested_version = shebang_version;
                    extra_args.append(&mut args);
                    args = extra_args;
                }
            }
        }
    }

    if requested_version == py::RequestedVersion::Any {
        if let Some(venv_root) = env::var_os("VIRTUAL_ENV") {
            let mut path = path::PathBuf::new();
            path.push(venv_root);
            path.push("bin");
            path.push("python");
            // TODO: Do a is_file() check first?
            if let Err(e) = run(&path, &args) {
                println!("{:?}", e);
            };
            return;
        }
    }

    requested_version = match requested_version {
        py::RequestedVersion::Any => py::check_default_env_var().unwrap_or(requested_version),
        py::RequestedVersion::Loose(major) => {
            py::check_major_env_var(major).unwrap_or(requested_version)
        }
        py::RequestedVersion::Exact(_, _) => requested_version,
    };

    let mut found_versions = collections::HashMap::new();
    for path in py::path_entries() {
        let all_contents = py::directory_contents(&path);
        for (version, path) in py::filter_python_executables(all_contents) {
            match version.matches(&requested_version) {
                py::VersionMatch::NotAtAll => continue,
                py::VersionMatch::Loosely => {
                    // The order of this guard is on purpose to potentially skip a stat call.
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

    // XXX Print an error message when no installed Python is found.
    let chosen_path = py::choose_executable(&found_versions).unwrap();
    if help {
        let version = env!("CARGO_PKG_VERSION");
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
            The following help text is from Python:\n\
            \n",
            version, launcher_path
        );
    }
    if let Err(e) = run(&chosen_path, &args) {
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
