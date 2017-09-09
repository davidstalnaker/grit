use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Read, Write};
use std::io;
use std::fs;

use errors::GritError;
use blob::Blob;
use commit_object::Commit;

pub struct FileService {
    pub root_dir: PathBuf,
    pub grit_dir: PathBuf,
    pub object_dir: PathBuf,
}

impl FileService {
    pub fn new() -> Result<FileService, GritError> {
        let root_dir = FileService::find_root_dir()?;
        let grit_dir = root_dir.join(".grit").to_path_buf();
        let object_dir = grit_dir.join("objects").to_path_buf();
        Ok(FileService {
            root_dir,
            grit_dir,
            object_dir,
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

    pub fn get_head_ref(&self) -> io::Result<PathBuf> {
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
            }
            Err(_) => None,
        }
    }

    pub fn write_blob(&self, blob: &Blob) -> io::Result<()> {
        self.write_object(&blob.hash, &blob.data)
    }

    pub fn read_commit(&self, hash: &str) -> Result<Commit, GritError> {
        Commit::from_string(hash, &self.read_object(hash)?)
    }

    pub fn write_commit(&self, commit: &mut Commit) -> io::Result<()> {
        commit.update();

        match commit {
            &mut Commit {
                hash: Some(ref hash),
                data: Some(ref data),
                ..
            } => {
                self.write_object(hash, data)?;
                let head = self.get_head_ref()?;
                let mut head_f = File::create(&head)?;
                head_f.write_all(hash.as_bytes())?;
            }
            _ => panic!("Commit.update() should have set hash and data."),
        }

        Ok(())
    }

    pub fn read_object(&self, hash: &str) -> io::Result<String> {
        let mut data = String::new();
        let object_filename = self.object_dir.join(&hash[..2]).join(&hash[2..]);
        let mut object_f = File::open(&object_filename)?;
        object_f.read_to_string(&mut data)?;
        Ok(data)
    }

    pub fn write_object(&self, hash: &str, data: &Vec<u8>) -> io::Result<()> {
        let blob_dir = self.object_dir.join(&hash[..2]);
        if !blob_dir.exists() {
            fs::create_dir(&blob_dir)?;
        }
        let blob_filename = blob_dir.join(&hash[2..]);
        let mut blob_f = File::create(&blob_filename)?;
        blob_f.write_all(data)?;

        Ok(())
    }
}
