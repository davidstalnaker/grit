extern crate crypto;

use std::io;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use self::crypto::sha1::Sha1;
use self::crypto::digest::Digest;

pub struct Blob {
    pub hash: String,
    pub data: Vec<u8>,
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
            data: bytes,
        })
    }
}
