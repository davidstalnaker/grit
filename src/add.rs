extern crate sha1;

use std::{io, fs};
use std::collections::HashMap;
use std::io::{BufReader, BufRead, Read, Write};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::env;

pub fn add_all(root_dir: &PathBuf, to_add: &Vec<&str>) -> io::Result<()> {
    let mut index = try!(Index::new(&root_dir));
    let filepaths = build_file_list(&to_add);
    for filename in filepaths {
        match write_blob(&filename, &root_dir) {
            Ok(hash) => {
                let relative_path = relative_from(&filename, root_dir).unwrap();
                index.update(&relative_path.to_str().unwrap(), &hash)
            },
            Err(e) => return Err(e)
        }
    }
    index.write()
}

fn relative_from<'a, P: Sized + AsRef<Path>>(abs: &'a P, root: &'a P) -> Option<&'a Path> {
    // See Path.relative_from(*) which is currently unstable.
    iter_after(abs.as_ref().components(), root.as_ref().components()).map(|c| c.as_path())
}
fn iter_after<A, I, J>(mut iter: I, mut prefix: J) -> Option<I> where
    I: Iterator<Item=A> + Clone, J: Iterator<Item=A>, A: PartialEq 
{
    loop {
        let mut iter_next = iter.clone();
        match (iter_next.next(), prefix.next()) {
            (Some(x), Some(y)) => {
                if x != y { return None }
            },
            (Some(_), None) => return Some(iter),
            (None, None) => return Some(iter),
            (None, Some(_)) => return None,
        }
        iter = iter_next;
    }
}

fn build_file_list(paths: &Vec<&str>) -> Vec<PathBuf> {
    let cur_dir = env::current_dir().unwrap();
    paths.iter().map(|path| cur_dir.join(path)).collect::<Vec<_>>() 
}

pub struct Index {
    path: Box<PathBuf>,
    hashes: HashMap<String, String>
}

impl Index {
    pub fn new(root_dir: &PathBuf) -> io::Result<Index> {
        let mut index = Index {
            path: Box::new(root_dir.join(".grit").join("index")),
            hashes: HashMap::new()
        };
        if !path_exists(&*index.path) {
            return Ok(index);
        }
        let file = BufReader::new(try!(File::open(&*index.path)));
        for line in file.lines() {
            match line {
                Ok(l) => {
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
            println!("{} {}", hash, path);
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

pub fn write_blob(path: &PathBuf, root_dir: &PathBuf) -> io::Result<String> {
    let mut f = try!(File::open(path));
    let mut bytes = Vec::new();
    try!(f.read_to_end(&mut bytes));

    let mut sha = sha1::Sha1::new();
    sha.update(&bytes);
    let hash = sha.hexdigest();

    let objects = root_dir.join(".grit").join("objects");
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
