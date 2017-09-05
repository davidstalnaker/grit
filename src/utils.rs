use std::io::Read;
use std::fs::File;
use std::env;
use std::path::{Path, PathBuf};

use errors::GritError;

pub fn find_root_dir() -> Result<PathBuf, GritError> {
    let mut cur_dir = env::current_dir()?;
    loop {
        if is_grit_dir(&cur_dir) {
            return Ok(cur_dir);
        }
        if !cur_dir.pop() {
            return Err(GritError::NoGritDir);
        }
    }
}

pub fn get_head_ref() -> Result<PathBuf, GritError> {
    let root_dir = find_root_dir()?;

    let mut head_file = File::open(&root_dir.join(".grit/HEAD"))?;
    let mut ref_path = String::new();
    head_file.read_to_string(&mut ref_path)?;
    
    Ok(root_dir.join(".grit").join(ref_path))
}

pub fn get_hash_from_ref(ref_path: &PathBuf) -> Option<String> {
    match File::open(ref_path) {
        Ok(ref mut f) => {
            let mut hash = String::new();
            f.read_to_string(&mut hash).unwrap();
            Some(hash)
        },
        Err(_) => None
    }
}

pub fn is_grit_dir<P>(path: P) -> bool
where
    P: Sized + AsRef<Path>,
{
    path.as_ref().join(".grit").exists()
}
