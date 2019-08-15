use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::path::PathBuf;

// XXX Want a global lock on environment variable mutation to prevent interleaving
// tests from stepping on each other.
// https://github.com/rust-lang/rust/issues/43155#issuecomment-315543432 should
// work, but I can't get access to the `lazy_static!` macro in this file to work.
pub struct TempEnvVar {
    key: OsString,
    value: Option<OsString>,
}

impl Drop for TempEnvVar {
    fn drop(&mut self) {
        println!(
            "Resetting {} to {:?}",
            self.key.to_string_lossy(),
            self.value
        );
        match &self.value {
            Some(original_value) => env::set_var(&self.key, original_value),
            None => env::remove_var(&self.key),
        }
    }
}

impl TempEnvVar {
    pub fn new(key: &OsStr, value: &OsStr) -> Self {
        let env_var = TempEnvVar {
            key: key.to_os_string(),
            value: env::var_os(key),
        };
        println!(
            "Setting {} to {}",
            key.to_string_lossy(),
            value.to_string_lossy()
        );
        env::set_var(key, value);
        env_var
    }
}

pub fn touch_file(path: PathBuf) -> PathBuf {
    let file = File::create(&path).unwrap();
    file.sync_all().unwrap();
    path
}
