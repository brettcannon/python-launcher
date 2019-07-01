// https://docs.python.org/3.8/using/windows.html#python-launcher-for-windows
// https://github.com/python/cpython/blob/master/PC/launcher.c

use std::{
    env,
    ffi::CString,
    fs::File,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    str::FromStr,
};

use nix::unistd;

use python_launcher as py;
use python_launcher::cli;
use python_launcher::cli::Action;
use python_launcher::path;
use python_launcher::version::RequestedVersion;

fn main() {
    match py::cli::action_from_args(env::args().collect::<Vec<String>>()) {
        Action::Help(launcher_path) => help(&launcher_path),
        // XXX Error out if failed: https://rust-lang-nursery.github.io/cli-wg/in-depth/exit-code.html.
        Action::List => println!("{}", cli::list_executables().unwrap()),
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

fn execute(_launcher: &PathBuf, version: RequestedVersion, args: &[String]) {
    let mut requested_version = version;
    let mut chosen_path: Option<PathBuf> = None;

    if requested_version == RequestedVersion::Any {
        if let venv_executable @ Some(..) = py::cli::activated_venv_executable() {
            chosen_path = venv_executable;
        } else if !args.is_empty() {
            // Using the first argument because it's the simplest and sanest.
            // We can't use the last argument because that could actually be an argument to the
            // Python module being executed. This is the same reason we can't go searching for
            // the first/last file path that we find. The only safe way to get the file path
            // regardless of its position is to replicate Python's arg parsing and that's a
            // **lot** of work for little gain. Hence we only care about the first argument.
            if let Ok(mut open_file) = File::open(&args[0]) {
                if let Some(shebang_version) = py::cli::parse_python_shebang(&mut open_file) {
                    requested_version = shebang_version;
                }
            }
        }
    }

    // XXX Check `PY_PYTHON`
    if chosen_path.is_none() {
        let directories = path::path_entries();
        match path::find_executable(requested_version, directories.into_iter()) {
            executable_path @ Some(..) => chosen_path = executable_path,
            None => {
                println!("No suitable interpreter found for {:?}", requested_version);
                // XXX Exit code
                return;
            }
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
