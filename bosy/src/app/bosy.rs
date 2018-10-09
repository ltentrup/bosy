//! This module contains the logic for the main `bosy` binary.

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

use clap::{App, Arg};

use super::super::specification::Specification;

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let matches = App::new("BoSy")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("BoSy is a synthesis tool for reactive systems")
            .arg(
                Arg::with_name("INPUT")
                    .help("Sets the input file to use")
                    .required(true)
                    .index(1),
            ).get_matches_from(args);

        let filename = matches.value_of("INPUT").map(|s| s.to_string()).unwrap();

        Config { filename }
    }

    pub fn run(&self) -> Result<(), Box<Error>> {
        let mut file = File::open(&self.filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let spec: Specification = serde_json::from_str(&contents)?;

        println!("Specification {:#?}", spec);

        unimplemented!();
    }
}
