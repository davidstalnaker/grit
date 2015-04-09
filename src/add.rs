extern crate sha1;

use std::{io};
use std::io::{Read};
use std::fs::File;
use std::path::Path;

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
    let blob = blob_dir.join(&hash[2..]);
    println!("{}: {}", to_add, blob.to_str().unwrap());

    return Ok(());
}
