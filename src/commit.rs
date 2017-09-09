use errors::GritError;
use commit_object::Commit;
use index::Index;
use file_service::FileService;

pub fn commit() -> Result<(), GritError> {
    let fs = FileService::new()?;
    let head_ref = fs.get_head_ref()?;
    let parent_hash = FileService::get_hash_from_ref(&head_ref);
    let mut index = Index::new(&fs.root_dir)?;

    let parent = match parent_hash {
        Some(ref h) => Some(fs.read_commit(h)?),
        None => None
    };
    let mut commit = Commit::new(parent.as_ref());
    parent.map(|p| p.print());
    commit.add_from_index(&index);
    commit.print();

    fs.write_commit(&mut commit)?;
    index.clear()?;
    Ok(())
}
