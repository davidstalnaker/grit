extern crate sha1;

use std::{io, fs};
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;

pub fn add_all(to_add: &Vec<&str>) {
    for filename in to_add {
        match add(filename) {
            Ok(()) => println!("Added {}.", filename),
            Err(e) => println!("Error: {}", e)
        }
    }
}

pub fn add(to_add: &str) -> io::Result<()> {
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

    return Ok(());
}