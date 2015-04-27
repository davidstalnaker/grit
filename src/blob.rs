extern crate sha1;

use std::{io, fs};
use std::io::{Read, Write};
use std::fs::File;
use std::path::PathBuf;

use utils::path_exists;

pub enum LazyData {
    Filename(PathBuf),
    Data(Vec<u8>)
}


pub struct Blob {
    pub hash: String,
    pub data: LazyData
}

impl Blob {
    pub fn from_path(path: &PathBuf) -> io::Result<Blob> {

        let mut bytes = try!(Blob::read(path));
        let mut sha = sha1::Sha1::new();
        sha.update(&bytes);
        Ok(Blob {
            hash: sha.hexdigest(),
            data: LazyData::Data(bytes)
        })
    }

    pub fn write(&mut self, root_dir: &PathBuf) -> io::Result<()> {
        let objects = root_dir.join(".grit").join("objects");
        let blob_dir = objects.join(&self.hash[..2]);
        if !path_exists(&blob_dir) {
            try!(fs::create_dir(&blob_dir));
        }
        let blob = blob_dir.join(&self.hash[2..]);
        if !path_exists(&blob) {
            let mut blob_f = try!(File::create(&blob));
            try!(blob_f.write_all(&try!(self.get_data())));
        }

        Ok(())
    }

    pub fn get_data(&mut self) -> io::Result<&Vec<u8>> {
        match self.data {
            LazyData::Data(ref data) => Ok(data),
            LazyData::Filename(ref path) => {
                let mut bytes = try!(Blob::read(path));
                self.data = LazyData::Data(bytes);
                Ok(&bytes)
            }
        }
    }

    fn read(path: &PathBuf) -> io::Result<Vec<u8>> {
        let mut f = try!(File::open(path));
        let mut bytes = Vec::new();
        try!(f.read_to_end(&mut bytes));
        Ok(bytes)
    }
}

