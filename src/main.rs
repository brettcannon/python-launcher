// https://docs.python.org/3.8/using/windows.html#python-launcher-for-windows
// https://github.com/python/cpython/blob/master/PC/launcher.c

use std::{collections, env, ffi, os::unix::ffi::OsStrExt, path};

use nix::unistd;

use python_launcher as py;

fn main() {
    let action = py::action_from_args(env::args().collect::<Vec<String>>());
    let mut requested_version = py::RequestedVersion::Any;
    let mut chosen_path: Option<path::PathBuf> = None;
    if let py::Action::Execute {launcher: _, version, args: _} = action {
        requested_version = version;
    }

    if requested_version == py::RequestedVersion::Any {
        if let Some(venv_root) = env::var_os("VIRTUAL_ENV") {
            let mut path = path::PathBuf::new();
            path.push(venv_root);
            path.push("bin");
            path.push("python");
            // TODO: Do a is_file() check first?
            chosen_path = Some(path);
        }
    }

    // XXX Look for shebang; LKG: https://github.com/brettcannon/python-launcher/blob/d70069de26e1ce87eeaf44c1ebd8ea6bfe894053/src/main.rs#L26

    if chosen_path.is_none() {
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

        chosen_path = py::choose_executable(&found_versions);
    }

    // XXX Print an error message when no installed Python is found.

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
                env!("CARGO_PKG_VERSION"), launcher_path.to_string_lossy(), chosen_path.unwrap().to_string_lossy()
            );
        }
        py::Action::Execute {launcher: _, version: _, args} => {
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
