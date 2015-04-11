extern crate sha1;

use std::{io, fs};
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;

pub fn add_all(to_add: &Vec<&str>) {
    let mut index = Index::new();
    for filename in to_add {
        match add(filename) {
            Ok(hash) => {
                println!("Added {}.", filename);
                index.update(filename, &hash);
            },
            Err(e) => println!("Error: {}", e)
        }
    }
    index.print();
    //index.write();
}

struct Index {
    hashes: Vec<(String, String)>
}

impl Index {
    fn new() -> Index {
        Index {
            hashes: Vec::new()
        }
    }

    fn update(&mut self, path: &str, hash: &str) {
        self.hashes.push((path.to_string(), hash.to_string()));
    }

    fn print(&self) {
        for &(ref hash, ref path) in &self.hashes {
            println!("{}, {}", hash, path);
        }
    }
}

pub fn add(to_add: &str) -> io::Result<String> {
    let path = Path::new(to_add);
    let mut f = try!(File::open(path));
    let mut bytes = Vec::new();
    try!(f.read_to_end(&mut bytes));

    let mut sha = sha1::Sha1::new();
    sha.update(&bytes);
    let hash = sha.hexdigest();

    let objects = Path::new(".grit/objects");
    let blob_dir = objects.join(&hash[..2]);
    if !fs::metadata(&blob_dir).is_ok() {
        try!(fs::create_dir(&blob_dir));
    }
    let blob = blob_dir.join(&hash[2..]);
    if !fs::metadata(&blob).is_ok() {
        let mut blob_f = try!(File::create(&blob));
        try!(blob_f.write_all(&bytes[..]));
    }

    return Ok(hash);
}
