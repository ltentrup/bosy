//! This module contains the logic for the main `bosy` binary.

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

use clap::{App, Arg};

use super::super::specification::Specification;

pub struct Config {
    filename: String,
    bound: usize,
    bounds: Vec<usize>,
    encoding_file_name: String,
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
                    .long("--spec")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("BOUND")
                    .help("Sets the bound (number of states in the implementation)")
                    .long("--bound")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("BOUNDS")
                    .help("Sets the bound (number of states in the implementation)")
                    .long("--bounds")
                    .takes_value(true)
                    .use_delimiter(true)
                    .multiple(true)
                    .required(false),
            )
            .arg(
                Arg::with_name("OUTPUT")
                    .help("Sets the file name where the ecoding is written to")
                    .long("--smt")
                    .required(true)
                    .takes_value(true),
            )
            .get_matches_from(args);

        let filename = matches.value_of("INPUT").map(|s| s.to_string()).unwrap();
        let encoding_file_name = matches.value_of("OUTPUT").map(|s| s.to_string()).unwrap();
        let bound = matches
            .value_of("BOUND")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap();
        let bounds: Vec<usize> = matches
            .values_of("BOUNDS")
            .map(|inner| {
                inner
                    .map(|s| {
                        s.parse::<usize>()
                            .expect("BOUNDS argument has to be an integer")
                    })
                    .collect()
            })
            .unwrap_or_else(Vec::new);

        Config {
            filename,
            bound,
            bounds,
            encoding_file_name,
        }
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

        eprintln!("LTL specification is {}", spec.ltl());

        if let Some(hyper) = &spec.hyper {
            for (i, spec) in hyper.iter().enumerate() {
                eprintln!("{}. HyperLTL specification is {}", i + 1, spec);
            }
        }

        eprintln!(
            "system bound {}, hyper-bounds {:?}",
            self.bound, self.bounds
        );

        let mut encoding = crate::encoding::BoSyEncoding::new(&spec);

        encoding.solve(self.bound, &self.bounds, &self.encoding_file_name);

        println!(
            "Written encoding for bound {} to file `{}`",
            self.bound, self.encoding_file_name
        );
        Ok(())
    }
}
