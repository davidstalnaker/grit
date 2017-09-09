use errors::GritError;
use commit_object::Commit;
use index::Index;
use file_service::FileService;

pub fn commit() -> Result<(), GritError> {
    let fs = FileService::new()?;
    let head_ref = fs.get_head_ref()?;
    let parent_hash = FileService::get_hash_from_ref(&head_ref);
    let mut index = Index::new(&fs.root_dir)?;
    let mut commit = Commit::new(parent_hash.as_ref().map(String::as_ref))?;
    commit.add_from_index(&index);
    fs.write_commit(&mut commit)?;
    index.clear()?;
    Ok(())
}
