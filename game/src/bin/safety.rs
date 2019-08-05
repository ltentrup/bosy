extern crate game;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = game::app::safety::Config::new(&args);
    config.run().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
}
