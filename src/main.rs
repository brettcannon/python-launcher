// https://docs.python.org/3.8/using/windows.html#python-launcher-for-windows
// https://github.com/python/cpython/blob/master/PC/launcher.c

extern crate nix;
extern crate python_launcher;

use std::collections;
use std::env;
use std::ffi;
use std::os::unix::ffi::OsStrExt;
use std::path;

use nix::unistd;

use python_launcher as py;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0); // Strip the path to this executable.
    let mut requested_version = py::RequestedVersion::Any;

    if args.len() >= 1 {
        if args[0].starts_with("-") {
            if let Some(version) = py::version_from_flag(&args[0]) {
                requested_version = version;
                args.remove(0);
            }
        } else {
            // XXX Try for a shebang.
        }
    }

    if requested_version == py::RequestedVersion::Any {
        if let Some(venv_root) = env::var_os("VIRTUAL_ENV") {
            let mut path = path::PathBuf::new();
            path.push(venv_root);
            path.push("bin");
            path.push("python");
            match run(&path, &args) {
                Err(e) => println!("{:?}", e),
                Ok(_) => (),
            };
            return;
        }
    }

    match requested_version {
        py::RequestedVersion::Any => match py::check_default_env_var() {
            Ok(found_version) => requested_version = found_version,
            _ => (),
        },
        py::RequestedVersion::Loose(major) => match py::check_major_env_var(major) {
            Ok(found_version) => requested_version = found_version,
            _ => (),
        },
        py::RequestedVersion::Exact(_, _) => (),
    };

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

    let chosen_path = py::choose_executable(&found_versions).unwrap();
    match run(&chosen_path, &args) {
        Err(e) => println!("{:?}", e),
        Ok(_) => (),
    };
}

fn run(executable: &path::PathBuf, args: &Vec<String>) -> nix::Result<()> {
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
