#![deny(missing_docs)]
//! Parses and validates command-line arguments.

// declaring interface and usage information
extern crate docopt;
use self::docopt::Docopt;

const USAGE: &'static str ="
Usage: file_tagger [-rd] <tag> <file>...
       file_tagger (-s | --search) [-p PATH] <tag>
       file_tagger (-h | --help)


Options:
	-h, --help       Show this help.
	-d, --delete     Delete the tag.
	-p <path>, --path <path>       Specify a path.
	-r, --recurse    Apply action to all files and subfolders.
	-s, --search     Search for the specified tag. Returns a list of files, separated by a newline.
";

/// Error handling enum
#[derive(Debug)]
pub enum ValidationError{
	/// Thrown when a file is not found
	NotFound,
}

/// The struct for the arguments. See *USAGE* for details.
#[derive(Debug, RustcDecodable)]
pub struct Args{
	flag_delete: bool,
	flag_path: String,
	flag_recurse: bool,
	flag_search: bool,
	arg_tag: String,
	arg_file: Vec<String>,
}


/// Validates the arguments
fn validate_args(args: &Args){

}

/// Retrieves arguments from the command-line
pub fn parse_cli_args() -> Args {
	let args = Docopt::new(USAGE)
                    .and_then(|d| d.decode())
                    .unwrap_or_else(|e| e.exit());
	info!("{:?}", args);
	validate_args(&args);
	return args;
}
