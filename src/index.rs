use std::io;
use std::collections::HashMap;
use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use std::path::{PathBuf};

use utils::path_exists;

pub struct Index {
    path: PathBuf,
    hashes: HashMap<String, String>
}

impl Index {
    pub fn new(root_dir: &PathBuf) -> io::Result<Index> {
        let mut index = Index {
            path: root_dir.join(".grit").join("index"),
            hashes: HashMap::new()
        };
        if !path_exists(&index.path) {
            return Ok(index);
        }
        let file = BufReader::new(try!(File::open(&index.path)));
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
        let mut index = try!(File::create(&self.path));
        for (ref hash, ref path) in self.hashes.iter() {
            try!(writeln!(&mut index, "{} {}", hash, path));
        }
        Ok(())
    }
}

