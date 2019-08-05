extern crate game;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = game::app::game::Config::new(&args);
    config.run().unwrap_or_else(|err| {
        eprintln!("Problem while executing BoSy: {}", err);
        process::exit(1);
    });
}
