use std::env;

use utils::{find_root_dir};
use blob::Blob;
use index::Index;
use errors::GritError;


pub fn add_all(to_add: &Vec<&str>) -> Result<(), GritError> {
    let root_dir = find_root_dir()?;
    let cur_dir = env::current_dir().unwrap();
    let mut index = Index::new(&root_dir)?;
    
    for filename in to_add {
        let full_path = cur_dir.join(filename);
        let blob = Blob::from_path(&full_path)?;
        blob.write(&root_dir)?;
        let relative_path = full_path.strip_prefix(&root_dir).unwrap();
        index.update(&relative_path.to_str().unwrap(), &blob.hash)
    }
    index.write()?;
    Ok(())
}