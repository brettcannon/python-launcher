use std::collections::HashMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::path::PathBuf;

use tempfile::TempDir;

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

    #[allow(dead_code)]
    pub fn empty() -> Self {
        let mut state = Self::new();
        state.change("PATH", None);
        for env_var in ["VIRTUAL_ENV", "PY_PYTHON", "PY_PYTHON3", "PY_PYTHON2"].iter() {
            state.change(env_var, None);
        }

        state
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

pub struct CurrentDir {
    _original_dir: PathBuf,
    pub dir: TempDir,
}

impl Drop for CurrentDir {
    fn drop(&mut self) {
        env::set_current_dir(&self._original_dir).unwrap();
    }
}

impl CurrentDir {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let _original_dir = env::current_dir().unwrap();
        let dir = TempDir::new().unwrap();
        env::set_current_dir(dir.path()).unwrap();
        Self { _original_dir, dir }
    }
}

pub fn touch_file(path: PathBuf) -> PathBuf {
    let file = File::create(&path).unwrap();
    file.sync_all().unwrap();
    path
}

#[allow(dead_code)]
pub struct EnvState {
    _dir1: TempDir,
    _dir2: TempDir,
    pub env_vars: EnvVarState,
    pub python27: PathBuf,
    pub python36: PathBuf,
    pub python37: PathBuf,
}

impl EnvState {
    /// Create a testing environment within the OS.
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
    #[allow(dead_code)]
    pub fn new() -> Self {
        let dir1 = TempDir::new().unwrap();
        let dir2 = TempDir::new().unwrap();

        let python27 = touch_file(dir1.path().join("python2.7"));
        let python36 = touch_file(dir1.path().join("python3.6"));
        touch_file(dir2.path().join("python3.6"));
        let python37 = touch_file(dir2.path().join("python3.7"));

        let new_path = env::join_paths([dir1.path(), dir2.path()].iter()).unwrap();
        let mut env_changes = EnvVarState::new();
        env_changes.change("PATH", Some(new_path.to_str().unwrap()));
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
