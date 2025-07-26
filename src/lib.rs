// #![warn(non_snake_case)]
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
pub fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let mut f = File::open(file_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Cleans a word by trimming non-alphanumeric characters and converting it to lowercase.
pub fn clean_word(word: &str) -> Option<String> {
    let cleaned = word
        .trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase();

    (!cleaned.is_empty()).then_some(cleaned)
}
