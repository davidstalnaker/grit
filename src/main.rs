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

    match matches.subcommand() {
        ("init", Some(..)) => {
            match grit::init() {
                Ok(()) => println!("Initialized."),
                Err(..) => println!("Error: already initialized.")
            }
        },
        ("add", Some(submatches)) => {
            match grit::find_root_dir() {
                Ok(root_dir) => {
                    match grit::add_all(&root_dir, &submatches.values_of("file").unwrap()) {
                        Ok(()) => (),
                        Err(e) => println!("Error: {}", e)
                    }
                },
                Err(msg) => { println!("{}", msg); }
            }
        }
        _ => println!("Command not recognized.")
    }
}

