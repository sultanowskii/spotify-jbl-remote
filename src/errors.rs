use std::fmt;
use std::process;

pub fn exit_with_error(msg: &str) -> ! {
    eprintln!("{}", msg);
    eprintln!("Aborting.");
    process::exit(-1);
}

#[derive(Debug)]
pub struct InvalidArgumentError;

impl fmt::Display for InvalidArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid argument is passed.")
    }
}

#[derive(Debug)]
pub struct IncompleteBuilderError;

impl fmt::Display for IncompleteBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Builder is incomplete.")
    }
}