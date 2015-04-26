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
        .subcommand(SubCommand::new("commit")
                    .about("Commits a change."))
        .get_matches();

    match matches.subcommand() {
        ("init", Some(..)) => {
            match grit::init() {
                Ok(()) => println!("Initialized."),
                Err(..) => println!("Error: already initialized.")
            }
        },
        ("add", Some(submatches)) => {
            match grit::add_all(&submatches.values_of("file").unwrap()) {
                Ok(()) => (),
                Err(e) => println!("Error: {}", e)
            }
        }
        ("commit", Some(..)) => {
            match grit::commit() {
                Ok(()) => (),
                Err(e) => println!("Error: {}", e)
            }
        }
        _ => println!("Command not recognized.")
    }
}

