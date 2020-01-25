use std::collections::HashMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::path::PathBuf;

use tempfile::TempDir;

// XXX Want a global lock on environment variable mutation to prevent interleaving
// tests from stepping on each other.
// https://github.com/rust-lang/rust/issues/43155#issuecomment-315543432 should
// work, but I can't get access to the `lazy_static!` macro in this file to work.
pub struct EnvVarState {
    changed: HashMap<OsString, Option<OsString>>,
}

impl Drop for EnvVarState {
    fn drop(&mut self) {
        self.changed.iter().for_each(|(k, v)| match &v {
            Some(original_v) => env::set_var(&k, original_v),
            None => env::remove_var(&k),
        });
    }
}

impl EnvVarState {
    pub fn new() -> Self {
        Self {
            changed: HashMap::new(),
        }
    }

    pub fn change(&mut self, k: &str, v: Option<&str>) {
        let os_k = OsStr::new(k);
        if !self.changed.contains_key(os_k) {
            let original_v = env::var_os(k);
            self.changed.insert(os_k.to_os_string(), original_v);
        }
        match v {
            Some(new_v) => env::set_var(k, new_v),
            None => env::remove_var(k),
        }
    }
}

fn touch_file(path: PathBuf) -> PathBuf {
    let file = File::create(&path).unwrap();
    file.sync_all().unwrap();
    path
}

pub struct EnvState {
    _dir1: TempDir,
    _dir2: TempDir,
    pub env_vars: EnvVarState,
    pub python27: PathBuf,
    pub python36: PathBuf,
    pub python37: PathBuf,
}

impl EnvState {
    /// Create a testing environement within the OS.
    /// - Create two temp directories (referred to as `dir1` and `dir2` from now on)
    /// - `dir1/python2.7`
    /// - `dir1/python3.6`
    /// - `dir2/python3.6`
    /// - `dir2/python3.7`
    /// - `PATH` environment variable is set to `dir1` and `dir2`
    /// - `VIRTUAL_ENV` is unset
    /// - `PY_PYTHON` is unset
    /// - `PY_PYTHON3` is unset
    /// - `PY_PYTHON2` is unset
    pub fn new() -> Self {
        let dir1 = TempDir::new().unwrap();
        let dir2 = TempDir::new().unwrap();

        let python27 = touch_file(dir1.path().join("python2.7"));
        let python36 = touch_file(dir1.path().join("python3.6"));
        touch_file(dir2.path().join("python3.6"));
        let python37 = touch_file(dir2.path().join("python3.7"));

        let new_path = env::join_paths([dir1.path(), dir2.path()].iter()).unwrap();
        let mut env_changes = EnvVarState::new();
        env_changes.change("PATH", Some(&new_path.to_str().unwrap()));
        for env_var in ["VIRTUAL_ENV", "PY_PYTHON", "PY_PYTHON3", "PY_PYTHON2"].iter() {
            env_changes.change(env_var, None);
        }

        Self {
            _dir1: dir1,
            _dir2: dir2,
            env_vars: env_changes,
            python27,
            python36,
            python37,
        }
    }
}
