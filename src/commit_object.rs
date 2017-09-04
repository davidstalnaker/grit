extern crate crypto;

use std::io;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::collections::BTreeMap;
use std::io::Write;
use self::crypto::sha1::Sha1;
use self::crypto::digest::Digest;

use errors::GritError;
use index::Index;

pub struct Commit {
    pub hash: Option<String>,
    pub parent: Option<String>,
    pub files: BTreeMap<String, String>,
}

impl Commit {
    pub fn new(parent: Option<&str>) -> Result<Commit, GritError> {
        let commit = Commit {
            hash: None,
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

    pub fn write(&mut self, root_dir: &PathBuf) -> io::Result<()> {
        let mut bytes = Vec::new();
        if let Some(ref p) = self.parent {
            writeln!(&mut bytes, "parent {}", p)?;
        }
        for (ref hash, ref path) in self.files.iter() {
            writeln!(&mut bytes, "blob {} {}", hash, path)?;
        }

        let mut sha = Sha1::new();
        sha.input(&bytes);
        let hash = sha.result_str();

        let objects = root_dir.join(".grit").join("objects");
        let blob_dir = objects.join(&hash[..2]);
        if !blob_dir.exists() {
            fs::create_dir(&blob_dir)?;
        }
        let blob = blob_dir.join(&hash[2..]);
        if !blob.exists() {
            let mut blob_f = File::create(&blob)?;
            blob_f.write_all(&bytes)?;
        }

        let mut ref_f = File::create(root_dir.join(".grit").join("refs/heads/master"))?;
        writeln!(ref_f, "{}", &hash)?;

        Ok(())
    }
}
