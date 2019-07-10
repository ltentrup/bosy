extern crate bosy;

use std::env;
use std::process;
use std::thread;

const STACK_SIZE: usize = 64 * 1024 * 1024;

fn run() {
    let args: Vec<String> = env::args().collect();

    let config = bosy::app::bosy::Config::new(&args);
    config.run().unwrap_or_else(|err| {
        eprintln!("Problem while executing BoSy: {}", err);
        process::exit(1);
    });
}

fn main() {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
