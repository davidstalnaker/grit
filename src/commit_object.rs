//use std::path::Path;
use std::collections::HashMap;

//use blob::Blob;
use errors::GritError;
use index::Index;

pub struct Commit {
    pub hash: Option<String>,
    pub parent: Option<String>,
    pub files: HashMap<String, String>,
}

impl Commit {
    pub fn new(parent: Option<&str>) -> Result<Commit, GritError> {
        let commit = Commit {
            hash: None,
            parent: parent.map(|p| p.to_string()),
            files: HashMap::new()
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
}