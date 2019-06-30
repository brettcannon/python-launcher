// https://docs.python.org/3.8/using/windows.html#python-launcher-for-windows
// https://github.com/python/cpython/blob/master/PC/launcher.c

use std::{
    cmp, env,
    ffi::CString,
    fs::File,
    iter::FromIterator,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    str::FromStr,
};

use nix::unistd;

use python_launcher as py;
use python_launcher::cli::Action;
use python_launcher::version::RequestedVersion;

fn main() {
    match py::cli::action_from_args(env::args().collect::<Vec<String>>()) {
        Action::Help(launcher_path) => help(&launcher_path),
        Action::List => list_executables(),
        Action::Execute {
            launcher_path,
            version,
            args,
        } => execute(&launcher_path, version, &args),
    }
}

/// Find a Python executable on `PATH`.
///
/// Environment variables are checked to see if they specify a specific Python version.
fn find_executable(version: RequestedVersion) -> Option<PathBuf> {
    let mut requested_version = version;

    if let Some(env_var) = requested_version.env_var() {
        if let Ok(env_var_value) = env::var(env_var) {
            if let Ok(env_requested_version) = RequestedVersion::from_str(&env_var_value) {
                requested_version = env_requested_version;
            }
        };
    }

    let found_versions = py::path::available_executables(requested_version);

    py::path::choose_executable(&found_versions)
}

fn help(launcher_path: &Path) {
    let mut chosen_path: Option<PathBuf>;

    if let venv_executable @ Some(..) = py::cli::activated_venv_executable() {
        chosen_path = venv_executable;
    } else {
        chosen_path = find_executable(RequestedVersion::Any);
        if chosen_path.is_none() {
            println!("No Python interpreter found");
            return;
        }
    }

    let found_path = chosen_path.unwrap();

    println!(
        include_str!("HELP.txt"),
        env!("CARGO_PKG_VERSION"),
        launcher_path.to_string_lossy(),
        found_path.to_string_lossy()
    );

    if let Err(e) = run(&found_path, &["--help".to_string()]) {
        println!("{:?}", e);
    }
}

fn list_executables() {
    let executables = py::path::available_executables(py::version::RequestedVersion::Any);
    if executables.is_empty() {
        println!("No Python executables found");
        return;
    }
    let mut executable_pairs = Vec::from_iter(executables);
    executable_pairs.sort_unstable();

    let max_version_length = executable_pairs.iter().fold(0, |max_so_far, pair| {
        cmp::max(max_so_far, pair.0.to_string().len())
    });

    // Including two spaces for readability padding.
    let left_column_width = cmp::max(max_version_length, "Version".len());

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

fn execute(_launcher: &PathBuf, version: RequestedVersion, original_args: &[String]) {
    let mut requested_version = version;
    let mut chosen_path: Option<PathBuf> = None;
    let mut args = original_args.to_owned();

    if requested_version == RequestedVersion::Any {
        if let venv_executable @ Some(..) = py::cli::activated_venv_executable() {
            chosen_path = venv_executable;
        } else if !args.is_empty() {
            // Using the first argument because it's the simplest and sanest.
            // We can't use the last argument because that could actually be an argument to the
            // Python module being executed. This is the same reason we can't go searching for
            // the first/last file path that we find. The only safe way to get the file path
            // regardless of its position is to replicate Python's arg parsing and that's a
            // **lot** of work for little gain.
            if let Ok(open_file) = File::open(&args[0]) {
                if let Some(shebang) = py::cli::find_shebang(open_file) {
                    // XXX Drop extra args as these won't be parsed appropriately by simply splitting on whitespace
                    if let Some((shebang_version, mut extra_args)) =
                        py::cli::split_shebang(&shebang)
                    {
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

fn run(executable: &Path, args: &[String]) -> nix::Result<()> {
    let executable_as_cstring = CString::new(executable.as_os_str().as_bytes()).unwrap();
    let mut argv = vec![executable_as_cstring.clone()];
    argv.extend(args.iter().map(|arg| CString::new(arg.as_str()).unwrap()));

    let result = unistd::execv(&executable_as_cstring, &argv);
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
