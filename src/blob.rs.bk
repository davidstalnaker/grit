extern crate crypto;

use std::{io, fs};
use std::io::{Read, Write};
use std::fs::File;
use std::path::PathBuf;
use self::crypto::sha1::Sha1;
use self::crypto::digest::Digest;

pub struct Blob {
    pub hash: String,
    pub data: Vec<u8>
}

impl Blob {
    pub fn from_path(path: &PathBuf) -> io::Result<Blob> {
        let mut f = File::open(path)?;
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;

        let mut sha = Sha1::new();
        sha.input(&bytes);
        Ok(Blob {
            hash: sha.result_str(),
            data: bytes
        })
    }

    pub fn write(&self, root_dir: &PathBuf) -> io::Result<()> {
        let objects = root_dir.join(".grit").join("objects");
        let blob_dir = objects.join(&self.hash[..2]);
        if !blob_dir.exists() {
            fs::create_dir(&blob_dir)?;
        }
        let blob = blob_dir.join(&self.hash[2..]);
        if !blob.exists() {
            let mut blob_f = File::create(&blob)?;
            blob_f.write_all(&self.data)?;
        }

        Ok(())
    }
}

