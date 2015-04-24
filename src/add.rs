use std::io::Write;
use std::path::{PathBuf};
use std::env;

use utils::relative_from;
use blob::Blob;
use index::Index;
use errors::GritError;


pub fn add_all(root_dir: &PathBuf, to_add: &Vec<&str>) -> Result<(), GritError> {
    let mut index = try!(Index::new(&root_dir));
    let filepaths = build_file_list(&to_add);
    for filename in filepaths {
        let blob = try!(Blob::from_path(&filename));
        try!(blob.write(root_dir));
        let relative_path = relative_from(&filename, root_dir).unwrap();
        index.update(&relative_path.to_str().unwrap(), &blob.hash)
    }
    try!(index.write());
    Ok(())
}

fn build_file_list(paths: &Vec<&str>) -> Vec<PathBuf> {
    let cur_dir = env::current_dir().unwrap();
    paths.iter().map(|path| cur_dir.join(path)).collect::<Vec<_>>() 
}
