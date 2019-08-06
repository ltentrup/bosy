//! This module contains the logic for the main `bosy` binary.

use crate::safety::{SafetyGame, SafetyGameSolver};
use bosy::specification::Specification;
use clap::{App, Arg};
use cudd::{CuddManager, CuddReordering};
use log::info;
use simplelog::{CombinedLogger, LevelFilter, TermLogger};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

pub struct Config {
    filename: String,
    verbosity: LevelFilter,
    bound: u32,
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
            .arg(
                Arg::with_name("v")
                    .short("v")
                    .multiple(true)
                    .help("Sets the level of verbosity"),
            )
            .arg(
                Arg::with_name("BOUND")
                    .help("Sets the bound (number of states in the implementation)")
                    .long("--bound")
                    .takes_value(true)
                    .required(true),
            )
            .get_matches_from(args);

        let filename = matches.value_of("INPUT").map(|s| s.to_string()).unwrap();

        let verbosity = match matches.occurrences_of("v") {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            3 | _ => LevelFilter::Trace,
        };

        let bound = matches
            .value_of("BOUND")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap();

        Config {
            filename,
            verbosity,
            bound,
        }
    }

    pub fn run(&self) -> Result<(), Box<Error>> {
        CombinedLogger::init(vec![
            TermLogger::new(
                self.verbosity,
                simplelog::Config::default(),
                simplelog::TerminalMode::Mixed,
            )
            .expect("Could not initialize `TermLogger`"),
            //WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ])
        .expect("Could not initialize logging");

        let mut file = File::open(&self.filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let spec: Specification = serde_json::from_str(&contents)?;

        info!("specification {:?}", spec);

        if let Err(errors) = spec.check() {
            eprintln!("Specification contains errors");
            for error in errors {
                eprintln!("{}", error);
            }
            process::exit(1);
        }

        assert!(spec.hyper().is_none());

        let bound = self.bound;

        let manager = CuddManager::new();
        manager.set_auto_dyn(CuddReordering::GroupSift);
        let safety_game = SafetyGame::from_bosy(&spec, &manager, bound);
        let mut solver = SafetyGameSolver::new(safety_game, spec.semantics());
        if solver.solve().is_none() {
            println!("result: unknown with bound {}", bound);
        } else {
            println!("result: realizable with bound {}", bound);
        }

        Ok(())
    }
}
