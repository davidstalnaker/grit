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

    match grit::find_root_dir() {
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

