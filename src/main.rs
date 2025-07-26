use Rust_Word_Frequency_Analyzer_using_HashMap::*;
use std::collections::HashMap;
use std::process;

fn main() {
    let mut count = HashMap::new();

    let message = match read_file_to_string("input.txt") {
        Ok(contents) => {
            println!(
                "✅ File opened successfully.\n\n📄 File Contents:\n\n{}",
                contents
            );
            contents
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };

    for raw_word in message.split_whitespace() {
        if let Some(word) = clean_word(raw_word) {
            *count.entry(word).or_insert(0) += 1;
        }
    }

    println!("Word counts:");
    for (word, count) in &count {
        println!("{:?}: {:?}", word, count);
    }
}
