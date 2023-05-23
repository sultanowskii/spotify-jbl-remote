use std::fmt;
use std::fmt::Display;
use std::process;

// Print error message and exit gracefully.
pub fn exit_with_error<T>(msg: T) -> ! where T: Display {
    eprintln!("{}", msg);
    eprintln!("Exiting...");
    process::exit(-1);
}

// Raw-to-object conversion error.
#[derive(Debug)]
pub struct RawToObjectConversionError;

impl fmt::Display for RawToObjectConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Conversion failed. Input device file might be broken.")
    }
}

// Invalid raw data error.
// Used when bytes-to-object conversion fails.
#[derive(Debug)]
pub struct InvalidRawDataError;

impl fmt::Display for InvalidRawDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid raw data was passed to transformer.")
    }
}

// Incomplete builder error.
// Used when builder fails to build an object because not all fields are filled.
#[derive(Debug)]
pub struct IncompleteBuilderError;

impl fmt::Display for IncompleteBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Builder is incomplete.")
    }
}
