use std::fmt;
use std::process;

// Print error message and exit gracefully.
pub fn exit_with_error(msg: &str) -> ! {
    eprintln!("{}", msg);
    eprintln!("Aborting.");
    process::exit(-1);
}

// Invalid argument error.
// Used when invalid argunents are passed to function.
#[derive(Debug)]
pub struct InvalidArgumentError;

impl fmt::Display for InvalidArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid argument is passed.")
    }
}

// Incomplete builder error.
// Used when builder failes to build an object because not all fields are filled.
#[derive(Debug)]
pub struct IncompleteBuilderError;

impl fmt::Display for IncompleteBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Builder is incomplete.")
    }
}