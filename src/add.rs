use std::path::{PathBuf};
use std::env;

use utils::{find_root_dir};
use blob::Blob;
use index::Index;
use errors::GritError;


pub fn add_all(to_add: &Vec<&str>) -> Result<(), GritError> {
    let root_dir = find_root_dir()?;
    let mut index = Index::new(&root_dir)?;
    let filepaths = build_file_list(&to_add);
    for filename in filepaths {
        let blob = Blob::from_path(&filename)?;
        blob.write(&root_dir)?;
        let relative_path = filename.strip_prefix(&root_dir).unwrap();
        index.update(&relative_path.to_str().unwrap(), &blob.hash)
    }
    index.write()?;
    Ok(())
}

fn build_file_list(paths: &Vec<&str>) -> Vec<PathBuf> {
    let cur_dir = env::current_dir().unwrap();
    paths.iter().map(|path| cur_dir.join(path)).collect::<Vec<_>>() 
}
