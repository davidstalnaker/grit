extern crate clap;
extern crate grit;

use clap::{App, Arg, SubCommand};

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

    if let Some(submatches) = matches.subcommand_matches("add") {
        for f in submatches.values_of("file").unwrap() {
            match grit::add(f) {
                Ok(()) => println!("Initialized."),
                Err(e) => println!("Error: {}", e)
            }
        }
    }
}


