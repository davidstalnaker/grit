extern crate clap;
extern crate grit;

use clap::{App, Arg, SubCommand};
use std::fs;
use std::path::{PathBuf};
use std::env;

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
        match grit::init() {
            Ok(()) => println!("Initialized."),
            Err(..) => println!("Error: already initialized.")
        }
    }

    match find_root_dir() {
        Ok(root_dir) => {
            if let Some(submatches) = matches.subcommand_matches("add") {
                match grit::add_all(&root_dir, &submatches.values_of("file").unwrap()) {
                    Ok(()) => (),
                    Err(e) => println!("Error: {}", e)
                }
            }
        },
        Err(msg) => { println!("{}", msg); }
    }
    

}

fn find_root_dir() -> Result<PathBuf,String> {
    let mut cur_dir = env::current_dir().unwrap();
    loop {
        if is_grit_dir( &cur_dir ) {
            return Ok( cur_dir )
        }
        if !cur_dir.pop() {
            return Err("No grit directory found.".to_string());
        }
    }
}

fn path_exists(path : &PathBuf) -> bool {
    fs::metadata(path).is_ok()
}

fn is_grit_dir( path : &PathBuf ) -> bool {
    path_exists( &path.join(".grit") )
}
