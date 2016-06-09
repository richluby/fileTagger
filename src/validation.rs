#![deny(missing_docs)]
//! Parses and validates command-line arguments.

use docopt;
use std::io;
use std::fmt;
use std::fs;
use std::error;

// declaring interface and usage information

const USAGE: &'static str ="
Usage: file_tagger [-rd] <tag> <file>...
       file_tagger (-s | --search) [-p PATH] <tag>
	   file_tagger (-t | --tags) <PATH>...
       file_tagger (-h | --help)


Options:
	-h, --help       Show this help.
	-d, --delete     Delete the tag.
	-p <PATH>, --path <PATH>       Specify a path.
	-r, --recurse    Apply action to all files and subfolders.
	-s, --search     Search for files with the specified tag.
	-t, --tags       View tags for the specified file(s).
";

/// The error thrown by this module
#[derive(Debug)]
pub enum ValidationError{
	/// Thrown when an IO error occurs
	Io(io::Error),
	/// Thrown when there is an error with parsing the input
	Docopt(docopt::Error),
	/// Thrown when the others are not
	NotFound,
}

/// The display filter for ValidationError error
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValidationError::Io(ref err) => err.fmt(f),
			ValidationError::Docopt(ref err) => err.fmt(f),
            ValidationError::NotFound => write!(f, "The requested item was not found."),
        }
    }
}

/// Implmementing the necessary conversions for ValidationError
impl error::Error for ValidationError {
    fn description(&self) -> &str {
        match *self {
            ValidationError::Io(ref err) => err.description(),
			ValidationError::Docopt(ref err) => err.description(),
            ValidationError::NotFound => "The requested item was not found.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
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
	arg_path: Vec<String>
}

/// Validates the arguments
fn validate_args(args: &Args) -> Result<&Args, ValidationError>{
	// check if the files exist
	for array in vec![&args.arg_file, &args.arg_path]{//by grabbing all file items
		for file in array{ //and then going through each file
			if !fs::metadata(file).is_ok(){//to check if the path to the file is ok
				let error: io::Error = io::Error::new(io::ErrorKind::NotFound, file.to_string());
				return Err(ValidationError::Io(error));
			}
		}
	}
	Ok(args)
}

/// Retrieves arguments from the command-line
pub fn parse_cli_args() -> Result<Args, ValidationError> {
	let args = try!(docopt::Docopt::new(USAGE).and_then(|d| d.decode()));
	info!("{:?}", args);
	match validate_args(&args){
		Ok(_) => Ok(args),
		Err(err) => Err(err),
	}
}
