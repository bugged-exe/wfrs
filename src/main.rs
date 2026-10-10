use std::fs::{self, Metadata};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    filepath: String,
    #[arg(short='c', long="count")]
    length_in_bytes: bool,
}

fn main() {
    let args = Args::parse();
    dbg!(&args);

    run(args);
}

fn run(args: Args) {
    let file_content = fs::read_to_string(&args.filepath).expect("Couldnt read the file content.");
    let file_metadata = fs::metadata(&args.filepath).expect("Couldnt read the file's metadata.");

    if args.length_in_bytes {
        display_length_in_bytes(&args.filepath, file_metadata);
    }
}

fn display_length_in_bytes(file_name: &String, metadata: Metadata) {
    let bytes = metadata.len();
    println!("The size in bytes for: {} is {}", file_name, bytes);
}
