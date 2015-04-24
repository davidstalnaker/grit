use std::fs;
use std::io::{Write};
use std::fs::File;
use std::path::Path;

use errors::GritError;

pub fn init() -> Result<(), GritError> {
    let grit_dir = Path::new(".grit");
    try!(fs::create_dir(grit_dir));
    try!(fs::create_dir(grit_dir.join("objects")));
    try!(fs::create_dir(grit_dir.join("refs")));
    try!(fs::create_dir(grit_dir.join("refs").join("heads")));
    
    let mut head = try!(File::create(grit_dir.join("HEAD")));
    try!(head.write_all("ref: refs/heads/master".as_bytes()));
    Ok(())
}
