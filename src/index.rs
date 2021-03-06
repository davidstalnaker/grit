use std::io;
use std::collections::BTreeMap;
use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use std::path::PathBuf;

use errors::GritError;

pub struct Index {
    pub path: PathBuf,
    pub hashes: BTreeMap<String, String>,
}

impl Index {
    pub fn new(root_dir: &PathBuf) -> Result<Index, GritError> {
        let mut index = Index {
            path: root_dir.join(".grit").join("index"),
            hashes: BTreeMap::new(),
        };
        if !index.path.exists() {
            return Ok(index);
        }
        let file = BufReader::new(File::open(&index.path)?);
        for line in file.lines() {
            let l = line?;
            let blob: Vec<&str> = l.split(' ').collect();
            if blob.len() != 2 {
                return Err(GritError::InvalidIndexFile);
            }
            index.update(blob[1], blob[0]);
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
        let mut index = File::create(&self.path)?;
        for (ref hash, ref path) in self.hashes.iter() {
            writeln!(&mut index, "{} {}", hash, path)?;
        }
        Ok(())
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.hashes = BTreeMap::new();
        self.write()
    }
}
