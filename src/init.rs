use std::fs;
use std::io::{Write};
use std::fs::File;
use std::path::Path;

use errors::GritError;

pub fn init() -> Result<(), GritError> {
    let grit_dir = Path::new(".grit");
    fs::create_dir(grit_dir)?;
    fs::create_dir(grit_dir.join("objects"))?;
    fs::create_dir(grit_dir.join("refs"))?;
    fs::create_dir(grit_dir.join("refs").join("heads"))?;
    
    let mut head = File::create(grit_dir.join("HEAD"))?;
    head.write_all("ref: refs/heads/master".as_bytes())?;
    Ok(())
}
