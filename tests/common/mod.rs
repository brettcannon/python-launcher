use std::collections::HashMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::path::PathBuf;

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

    pub fn change(&mut self, k: &OsStr, v: Option<&OsStr>) {
        if !self.changed.contains_key(k) {
            let original_v = env::var_os(k);
            self.changed.insert(k.to_os_string(), original_v);
        }
        match v {
            Some(new_v) => env::set_var(k, new_v),
            None => env::remove_var(k),
        }
    }
}

pub fn touch_file(path: PathBuf) -> PathBuf {
    let file = File::create(&path).unwrap();
    file.sync_all().unwrap();
    path
}
