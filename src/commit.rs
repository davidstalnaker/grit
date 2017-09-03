use errors::GritError;
use commit_object::Commit;
use index::Index;
use utils::find_root_dir;

pub fn commit() -> Result<(), GritError> {
    let root_dir = find_root_dir()?;
    let index = Index::new(&root_dir)?;
    let mut commit = Commit::new(None)?;
    commit.add_from_index(&index);
    commit.print();
    Ok(())
}
