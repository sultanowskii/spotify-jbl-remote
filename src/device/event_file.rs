use std::{
    fs::{OpenOptions, File},
    io,
};

// Open event file with its handler filename.
pub fn open_event_file(filename: &String) -> io::Result<File> {
    let mut file_options = OpenOptions::new();
    file_options.read(true);
    file_options.write(false);
    
    file_options.open(filename)
}