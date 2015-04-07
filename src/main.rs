extern crate clap;
use clap::{App, SubCommand};
use std::{io, fs};
use std::io::Write;
use std::fs::File;
use std::path::Path;

fn main() {
    let matches = App::new("Grit")
        .subcommand(SubCommand::new("init")
                    .about("Initializes the repo"))
        .get_matches();

    if let Some(..) = matches.subcommand_matches("init") {
        match init() {
            Ok(()) => println!("Initialized."),
            Err(..) => println!("Error: already initialized.")
        }
    }
}

fn init() -> io::Result<()> {
    let grit_dir = Path::new(".grit");
    try!(fs::create_dir(grit_dir));
    try!(fs::create_dir(grit_dir.join("objects")));
    try!(fs::create_dir(grit_dir.join("refs")));
    try!(fs::create_dir(grit_dir.join("refs").join("heads")));
    
    let mut head = try!(File::create(grit_dir.join("HEAD")));
    head.write_all("ref: refs/heads/master".as_bytes())
}
