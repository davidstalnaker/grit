use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;

use errors::GritError;

pub struct FileService {
    pub root_dir: PathBuf,
    pub grit_dir: PathBuf,
}

impl FileService {
    pub fn new() -> Result<FileService, GritError> {
        let root_dir = FileService::find_root_dir()?;
        let grit_dir = root_dir.join(".grit").to_path_buf();
        Ok(FileService {
            root_dir,
            grit_dir, 
        })
    }

    fn find_root_dir() -> Result<PathBuf, GritError> {
        let mut cur_dir = env::current_dir()?;
        loop {
            if FileService::is_grit_dir(&cur_dir) {
                return Ok(cur_dir);
            }
            if !cur_dir.pop() {
                return Err(GritError::NoGritDir);
            }
        }
    }

    fn is_grit_dir<P>(path: P) -> bool
    where
        P: Sized + AsRef<Path>,
    {
        path.as_ref().join(".grit").exists()
    }

    pub fn get_head_ref(&self) -> Result<PathBuf, GritError> {
        let mut head_file = File::open(self.root_dir.join(".grit/HEAD"))?;
        let mut ref_path = String::new();
        head_file.read_to_string(&mut ref_path)?;
        
        Ok(self.grit_dir.join(ref_path))
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
}