#![deny(missing_docs)]
//! Provides the entry point for the program.

/// The dependencies for the program.
#[macro_use]
extern crate log;
extern crate env_logger;

// import our own modules
extern crate file_tagger;

/// Entry point for the program.
fn main(){
	env_logger::init().unwrap();
	let args = file_tagger::validation::parse_cli_args();
	println!("Compiled and ran successfully.");
}
