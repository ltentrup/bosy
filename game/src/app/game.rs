//! This module contains the logic for the main `bosy` binary.

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

use clap::{App, Arg};

use bosy::specification::Specification;

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let matches = App::new("BoSy-Game")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("BoSy is a synthesis tool for reactive systems")
            .arg(
                Arg::with_name("INPUT")
                    .help("Sets the input file to use")
                    .long("--spec")
                    .required(true)
                    .takes_value(true),
            )
            .get_matches_from(args);

        let filename = matches.value_of("INPUT").map(|s| s.to_string()).unwrap();

        Config { filename }
    }

    pub fn run(&self) -> Result<(), Box<Error>> {
        let mut file = File::open(&self.filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let spec: Specification = serde_json::from_str(&contents)?;

        //println!("Specification {:#?}", spec);

        match spec.check() {
            Err(errors) => {
                eprintln!("Specification contains errors");
                for error in errors {
                    eprintln!("{}", error);
                }
                process::exit(1);
            }
            _ => {}
        }

        let ltl = spec.ltl();
        eprintln!("LTL specification is {}", ltl);

        let normalized = ltl.normalize();

        eprintln!("normalized {}", normalized);

        let partitioned = normalized.partition().unwrap();

        eprintln!("partitioned\n\n{}", partitioned);

        assert!(spec.hyper().is_none());

        Ok(())
    }
}
