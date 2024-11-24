use std::{fs, process};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Wrong arguments provided");
        process::exit(1);
    }
    let path = &args[0];
    let file = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("An error occurred while reading the file: {e}");
        process::exit(1);
    });

    println!("{} {}", file.bytes().len(), path);
}
