#![deny(missing_docs)]
//! Provides the entry point for the program.

// The dependencies for the program.
#[macro_use]
extern crate log;
extern crate env_logger;
use std::process;

// import our own modules
extern crate file_tagger;

/// Entry point for the program.
fn main(){
	env_logger::init().unwrap();
	let args = match file_tagger::validation::parse_cli_args() {
		Ok(args) => args,
		Err(err) => {error!("The argument could not be processed: {:}", err.to_string()); process::exit(1)},
	};
	debug!("Using args: {:?}", args);
	println!("Compiled and ran successfully.");
}
