#![deny(missing_docs)]
//! Parses and validates command-line arguments.

use docopt;
use std::io;
use std::fmt;
use std::error::Error;

// declaring interface and usage information

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
	Io(io::Error),
	/// Thrown when there is an error with parsing the input
	Docopt(docopt::Error),
	/// Thrown when the file is not found
	NotFound,
}

/// The display filter for ValidationError error
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValidationError::Io(ref err) => err.fmt(f),
			ValidationError::Docopt(ref err) => err.fmt(f),
            ValidationError::NotFound => write!(f, "No matching cities with a \
                                             population were found."),
        }
    }
}

/// Implmementing the necessary conversions for ValidationError
impl Error for ValidationError {
    fn description(&self) -> &str {
        match *self {
            ValidationError::Io(ref err) => err.description(),
			ValidationError::Docopt(ref err) => err.description(),
            ValidationError::NotFound => "The requested item was not found.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ValidationError::Io(ref err) => Some(err),
			ValidationError::Docopt(ref err) => Some(err),
            // Our custom error doesn't have an underlying cause, but we could
            // modify it so that it does.
            ValidationError::NotFound => None,
        }
    }
}

impl From<io::Error> for ValidationError {
    fn from(err: io::Error) -> ValidationError {
        ValidationError::Io(err)
    }
}

impl From<docopt::Error> for ValidationError {
	fn from(err: docopt::Error) -> ValidationError {
		ValidationError::Docopt(err)
	}
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
fn validate_args(args: &Args) -> Result<&Args, ValidationError>{

	Ok(args)
}

/// Retrieves arguments from the command-line
pub fn parse_cli_args() -> Result<Args, ValidationError> {
	let args = try!(docopt::Docopt::new(USAGE).and_then(|d| d.decode()));
	info!("{:?}", args);
	validate_args(&args);
	Ok(args)
}
