// https://docs.python.org/3.8/using/windows.html#python-launcher-for-windows
// https://github.com/python/cpython/blob/master/PC/launcher.c

use std::{
    cmp::max, env, ffi, fs, iter::FromIterator, os::unix::ffi::OsStrExt, path, str::FromStr,
};

use nix::unistd;

use python_launcher as py;

fn main() {
    match py::action_from_args(env::args().collect::<Vec<String>>()) {
        py::Action::Help(launcher_path) => help(&launcher_path),
        py::Action::List => list_available_executables(),
        py::Action::Execute {
            launcher_path,
            version,
            args,
        } => execute(&launcher_path, version, &args),
    }
}

/// Find a Python executable on `PATH`.
///
/// Environment variables are checked to see if they specify a specific Python version.
fn find_executable(version: py::RequestedVersion) -> Option<path::PathBuf> {
    let mut requested_version = version;

    if let Some(env_var) = requested_version.env_var() {
        if let Ok(env_var_value) = env::var(env_var) {
            if let Ok(env_requested_version) = py::RequestedVersion::from_str(&env_var_value) {
                requested_version = env_requested_version;
            }
        };
    }

    let found_versions = py::available_executables(requested_version);

    py::choose_executable(&found_versions)
}

fn help(launcher_path: &path::Path) {
    let mut chosen_path: Option<path::PathBuf>;

    if let venv_executable @ Some(..) = py::virtual_env() {
        chosen_path = venv_executable;
    } else {
        chosen_path = find_executable(py::RequestedVersion::Any);
        if chosen_path.is_none() {
            println!("No Python interpreter found");
            return;
        }
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

fn list_available_executables() {
    let executables = py::available_executables(py::RequestedVersion::Any);
    if executables.is_empty() {
        println!("No Python executables found");
        return;
    }
    let mut executable_pairs = Vec::from_iter(executables);
    executable_pairs.sort_unstable();

    let max_version_length = executable_pairs.iter().fold(0, |max_so_far, pair| {
        max(max_so_far, pair.0.to_string().len())
    });

    // Including two spaces for readability padding.
    let left_column_width = max(max_version_length, "Version".len());

    println!("{:<1$}  Path", "Version", left_column_width);
    println!("{:<1$}  ====", "=======", left_column_width);

    for (version, path) in executable_pairs {
        println!(
            "{:<2$}  {}",
            version.to_string(),
            path.to_string_lossy(),
            left_column_width
        );
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
        chosen_path = find_executable(requested_version);

        if chosen_path.is_none() {
            println!("No suitable interpreter found for {:?}", requested_version);
            return;
        }
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
