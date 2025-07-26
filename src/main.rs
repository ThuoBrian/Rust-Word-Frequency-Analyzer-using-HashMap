use Rust_Word_Frequency_Analyzer_using_HashMap::*;
use std::collections::HashMap;

fn main() {
    let mut count = HashMap::new();

    let message = match read_file_to_string("input.txt") {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    for word in message.split_whitespace() {
        let word = word
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();
        if !word.is_empty() {
            *count.entry(word).or_insert(0) += 1;
        }
    }

    println!("Word counts:");
    for (word, count) in count {
        println!("{}: {}", word, count);
    }
}
