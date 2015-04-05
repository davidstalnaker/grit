extern crate clap;
use clap::{App, SubCommand};

fn main() {
    let matches = App::new("Grit")
        .subcommand(SubCommand::new("init")
                    .about("Initializes the repo"))
        .get_matches();

    if let Some(ref submatches) = matches.subcommand_matches("init") {
        println!("initializing...");
    }
}
