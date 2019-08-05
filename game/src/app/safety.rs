//! This module contains the logic for the main `safety` binary.

use crate::safety::{SafetyGame, SafetyGameSolver};
use aiger::Aiger;
use bosy::specification::Semantics;
use clap::{App, Arg};
use cudd::{CuddManager, CuddReordering};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let matches = App::new("Safety")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("Symbolic safety game solver")
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

        let aiger = Aiger::from_str(&contents)?;
        let manager = CuddManager::new();
        manager.set_auto_dyn(CuddReordering::GroupSift);
        let safety_game = SafetyGame::from(&aiger, &manager);
        let mut solver = SafetyGameSolver::new(safety_game, Semantics::Mealy);
        if solver.solve().is_none() {
            println!("unrealizable");
        } else {
            println!("realizable");
        }

        Ok(())
    }
}
