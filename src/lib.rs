#![deny(missing_docs)]
//! Defines the API available to the application

// Import necessary crates
extern crate rustc_serialize;
#[macro_use]
extern crate log;
extern crate env_logger;

// Declare available interfaces
pub mod validation;
