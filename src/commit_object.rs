extern crate crypto;

use std::collections::BTreeMap;
use std::io::Write;
use self::crypto::sha1::Sha1;
use self::crypto::digest::Digest;

use errors::GritError;
use index::Index;

pub struct Commit {
    pub hash: Option<String>,
    pub data: Option<Vec<u8>>,
    pub parent: Option<String>,
    pub files: BTreeMap<String, String>,
}

impl Commit {
    pub fn new(parent: Option<&str>) -> Result<Commit, GritError> {
        let commit = Commit {
            hash: None,
            data: None,
            parent: parent.map(|p| p.to_string()),
            files: BTreeMap::new(),
        };
        Ok(commit)
    }

    pub fn add_from_index(&mut self, index: &Index) {
        for (ref path, ref hash) in index.hashes.iter() {
            self.files.insert(path.to_string(), hash.to_string());
        }
    }

    pub fn print(&self) {
        for (ref hash, ref path) in self.files.iter() {
            println!("{} {}", hash, path);
        }
    }

    pub fn update(&mut self) {
        let mut data = Vec::new();
        if let Some(ref p) = self.parent {
            writeln!(&mut data, "parent {}", p).unwrap();
        }
        for (ref hash, ref path) in self.files.iter() {
            writeln!(&mut data, "blob {} {}", hash, path).unwrap();
        }

        let mut sha = Sha1::new();
        sha.input(&data);
        self.hash = Some(sha.result_str());
        self.data = Some(data);
    }
}
