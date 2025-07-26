use std::fs::File;
use std::io::{self, Read};

/// Opens a file at the given path and returns its contents as a String.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path of the file to open.
///
/// # Errors
///
/// Returns an `io::Error` if the file cannot be opened or read.
///

pub fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let mut f = File::open(file_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}
