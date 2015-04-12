extern crate sha1;

use std::{io, fs};
use std::collections::HashMap;
use std::io::{BufReader, BufRead, Read, Write};
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn add_all(to_add: &Vec<&str>) -> io::Result<()> {
    let mut index = try!(Index::new());
    for filename in to_add {
        match write_blob(filename) {
            Ok(hash) => index.update(filename, &hash),
            Err(e) => return Err(e)
        }
    }
    index.write()
}

pub struct Index {
    path: Box<PathBuf>,
    hashes: HashMap<String, String>
}

impl Index {
    pub fn new() -> io::Result<Index> {
        let mut index = Index {
            path: Box::new(Path::new(".grit").join("index")),
            hashes: HashMap::new()
        };
        if !path_exists(&*index.path) {
            return Ok(index);
        }
        let file = BufReader::new( try!(File::open(&*index.path)) );
        for line in file.lines() {
            match line {
                Ok( l ) => {
                    let blob : Vec<&str> = l.split(' ').collect();
                    index.update(blob[0], blob[1]);
                },
                Err(e) => println!("Error: {}",e)
            }
        }
        Ok(index)
    }

    pub fn update(&mut self, path: &str, hash: &str) {
        self.hashes.insert(path.to_string(), hash.to_string());
    }

    pub fn print(&self) {
        for (ref hash, ref path) in self.hashes.iter() {
            println!("{}, {}", hash, path);
        }
    }

    pub fn write(&self) -> io::Result<()> {
        let mut index = try!(File::create(&*self.path));
        for (ref hash, ref path) in self.hashes.iter() {
            try!(writeln!(&mut index, "{} {}", hash, path));
        }
        Ok(())
    }
}

fn path_exists(path : &PathBuf) -> bool {
    fs::metadata(path).is_ok()
}

pub fn write_blob(to_add: &str) -> io::Result<String> {
    let path = Path::new(to_add);
    let mut f = try!(File::open(path));
    let mut bytes = Vec::new();
    try!(f.read_to_end(&mut bytes));

    let mut sha = sha1::Sha1::new();
    sha.update(&bytes);
    let hash = sha.hexdigest();

    let objects = Path::new(".grit/objects");
    let blob_dir = objects.join(&hash[..2]);
    if !path_exists(&blob_dir) {
        try!(fs::create_dir(&blob_dir));
    }
    let blob = blob_dir.join(&hash[2..]);
    if !path_exists(&blob) {
        let mut blob_f = try!(File::create(&blob));
        try!(blob_f.write_all(&bytes[..]));
    }

    Ok(hash)
}
