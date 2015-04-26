use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use errors::GritError;

pub fn find_root_dir() -> Result<PathBuf, GritError> {
    let mut cur_dir = env::current_dir().unwrap();
    loop {
        if is_grit_dir(&cur_dir) {
            return Ok(cur_dir)
        }
        if !cur_dir.pop() {
            return Err(GritError::NoGritDir);
        }
    }
}

pub fn is_grit_dir<P>(path: P) -> bool where P: Sized + AsRef<Path> {
    path_exists(path.as_ref().join(".grit"))
}

// These are reimplementations of unstable rust features so we can use them.

pub fn relative_from<'a, P: Sized + AsRef<Path>>(abs: &'a P, root: &'a P) -> Option<&'a Path> {
    // See Path.relative_from(*) which is currently unstable.
    iter_after(abs.as_ref().components(), root.as_ref().components()).map(|c| c.as_path())
}

fn iter_after<A, I, J>(mut iter: I, mut prefix: J) -> Option<I> where
    I: Iterator<Item=A> + Clone, J: Iterator<Item=A>, A: PartialEq 
{
    loop {
        let mut iter_next = iter.clone();
        match (iter_next.next(), prefix.next()) {
            (Some(x), Some(y)) => {
                if x != y { return None }
            },
            (Some(_), None) => return Some(iter),
            (None, None) => return Some(iter),
            (None, Some(_)) => return None,
        }
        iter = iter_next;
    }
}

pub fn path_exists<P>(path: P) -> bool where P: Sized + AsRef<Path> {
    fs::metadata(path).is_ok()
}

