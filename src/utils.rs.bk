use std::env;
use std::path::{Path, PathBuf};

use errors::GritError;

pub fn find_root_dir() -> Result<PathBuf, GritError> {
    let mut cur_dir = env::current_dir().unwrap();
    loop {
        if is_grit_dir(&cur_dir) {
            return Ok(cur_dir)
        }
        if !cur_dir.pop() {
            return Err(GritError::NoGritDir);
        }
    }
}

pub fn is_grit_dir<P>(path: P) -> bool where P: Sized + AsRef<Path> {
    path.as_ref().join(".grit").exists()
}

