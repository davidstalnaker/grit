use errors::GritError;
use commit_object::Commit;
use index::Index;
use utils::{find_root_dir, get_head_ref, get_hash_from_ref};

pub fn commit() -> Result<(), GritError> {
    let root_dir = find_root_dir()?;
    let head_ref = get_head_ref()?;
    let head_hash = get_hash_from_ref(&head_ref);
    let mut index = Index::new(&root_dir)?;
    let mut commit = Commit::new(head_hash.as_ref().map(String::as_ref))?;
    commit.add_from_index(&index);
    commit.write(&root_dir, &head_ref)?;
    index.clear()?;
    Ok(())
}
