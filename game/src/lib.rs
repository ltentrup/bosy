#![warn(clippy::all, clippy::pedantic)]
#![deny(unused_must_use)]

extern crate hyperltl;
#[macro_use]
extern crate pest_derive;

mod aiger;
pub mod app;
pub mod bounded;
pub mod safety;
