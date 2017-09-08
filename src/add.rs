use std::env;

use file_service::FileService;
use blob::Blob;
use index::Index;
use errors::GritError;


pub fn add_all(to_add: &Vec<&str>) -> Result<(), GritError> {
    let file_service = FileService::new()?;
    let cur_dir = env::current_dir().unwrap();
    let mut index = Index::new(&file_service.root_dir)?;

    for filename in to_add {
        let full_path = cur_dir.join(filename);
        let blob = Blob::from_path(&full_path)?;
        blob.write(&file_service.root_dir)?;
        let relative_path = full_path.strip_prefix(&file_service.root_dir).unwrap();
        index.update(&relative_path.to_str().unwrap(), &blob.hash)
    }
    index.write()?;
    Ok(())
}
