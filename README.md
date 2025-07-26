# Rust Word Frequency Analyzer

A simple Rust CLI tool to analyze word frequency in a text file using a `HashMap`.

## Features

- Reads a text file (`input.txt` by default)
- Counts the frequency of each word (case-insensitive, ignores punctuation)
- Displays word counts and total unique words

## Usage

1. **Clone the repository:**
    ```sh
    git clone https://github.com/yourusername/Rust-Word-Frequency-Analyzer-using-HashMap.git
    cd Rust-Word-Frequency-Analyzer-using-HashMap
    ```

2. **Add your input file:**
    - Place your text file in the project directory and name it `input.txt`.

3. **Build and run:**
    ```sh
    cargo run
    ```

4. **Output:**
    - The program prints the file contents, word counts, and the total number of unique words.

## How it Works

- Reads the contents of `input.txt`
- Splits text into words, normalizes them (removes punctuation, converts to lowercase)
- Counts each word using a `HashMap`
- Prints results to the terminal

## Example Output
