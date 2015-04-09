extern crate clap;
extern crate sha1;

use clap::{App, Arg, SubCommand};
use std::{io, fs};
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;

fn main() {
    let matches = App::new("Grit")
        .subcommand(SubCommand::new("init")
                    .about("Initializes the repo"))
        .subcommand(SubCommand::new("add")
                    .about("Adds a file")
                    .arg(Arg::new("file")
                         .help("File to add")
                         .index(1)
                         .multiple(true)
                         .required(true)))
        .get_matches();

    if let Some(..) = matches.subcommand_matches("init") {
        match init() {
            Ok(()) => println!("Initialized."),
            Err(..) => println!("Error: already initialized.")
        }
    }

    if let Some(submatches) = matches.subcommand_matches("add") {
        for f in submatches.values_of("file").unwrap() {
            match add(f) {
                Ok(()) => println!("Initialized."),
                Err(e) => println!("Error: {}", e)
            }
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

fn add(to_add: &str) -> io::Result<()> {
    let path = Path::new(to_add);
    let mut f = try!(File::open(path));
    let mut bytes = Vec::new();
    try!(f.read_to_end(&mut bytes));

    let mut sha = sha1::Sha1::new();
    sha.update(&bytes);
    let hash = sha.hexdigest();

    let objects = Path::new(".grit/objects");
    let blob_dir = objects.join(&hash[..2]);
    let blob = blob_dir.join(&hash[2..]);
    println!("{}: {}", to_add, blob.to_str().unwrap());

    return Ok(());
}

