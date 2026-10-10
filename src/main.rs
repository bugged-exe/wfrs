use std::fs::{self, Metadata};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    filepath: String,
    #[arg(short='c', long="count")]
    length_in_bytes: bool,
    #[arg(short='l', long="lines")]
    num_of_lines: bool,
    #[arg(short='m', long="characters")]
    num_of_characters: bool,
    #[arg(short='w', long="words")]
    num_of_words: bool,
}

fn main() {
    let args = Args::parse();
    run(args);
}

fn run(args: Args) {
    let file_name = args.filepath;
    let file_contents = fs::read_to_string(&file_name).expect("Couldnt read the file content.");
    let file_metadata = fs::metadata(&file_name).expect("Couldnt read the file's metadata.");

    if 
    args.length_in_bytes != true && 
    args.num_of_lines != true &&
    args.num_of_characters != true &&
    args.num_of_words != true
    {
        display_number_of_lines(&file_name, &file_contents);   
        display_number_of_characters(&file_name, &file_contents);
        display_number_of_words(&file_name, &file_contents);
    }

    if args.length_in_bytes {
        display_length_in_bytes(&file_name, &file_metadata);
    }

    if args.num_of_lines {
        display_number_of_lines(&file_name, &file_contents);
    }

    if args.num_of_characters {
        display_number_of_characters(&file_name, &file_contents);
    }
    
    if args.num_of_words {
        display_number_of_words(&file_name, &file_contents);
    }
}

fn display_length_in_bytes(file_name: &String, metadata: &Metadata) {
    let bytes = metadata.len();
    println!("The size in bytes for: '{}' is {}", file_name, bytes);
}

fn display_number_of_lines(file_name: &String, contents: &String) {
    let lines = contents.lines().count();
    println!("The number of lines for: '{}' is {}", file_name, lines);
}

fn display_number_of_characters(file_name: &String, contents: &String) {
    let characters = contents.chars().count();
    println!("The number of characters for '{}' is {}", file_name, characters);
}

fn display_number_of_words(file_name: &String, contents: &String) {
    let contents = contents.split_whitespace();
    let words = contents.count();
    println!("The number of words for '{}' is {}", file_name, words);
}