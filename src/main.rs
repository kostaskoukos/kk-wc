use std::{error::Error, fs, process};

enum CountOptions {
    Bytes,
    Chars,
    Words,
    Lines,
}

struct Config {
    path: String,
    counts: Vec<CountOptions>,
}

impl Config {
    fn new(args: &[String]) -> Option<Config> {
        Some(Config {
            path: args.first()?.to_string(),
            counts: args
                .iter()
                .skip(1)
                .map(|arg| match arg.as_ref() {
                    "-b" => Some(CountOptions::Bytes),
                    "-c" => Some(CountOptions::Chars),
                    "-w" => Some(CountOptions::Words),
                    "-l" => Some(CountOptions::Lines),
                    _ => None,
                })
                .collect::<Option<Vec<CountOptions>>>()?,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let str = fs::read_to_string(config.path)?;
    if config.counts.is_empty() {
        println!(
            "Bytes: {} Characters: {} Words: {} Lines: {}",
            str.bytes().len(),
            str.chars().count(),
            str.split_ascii_whitespace().count(),
            str.lines().count()
        )
    } else {
        for count in config.counts {
            match count {
                CountOptions::Bytes => print!("Bytes: {} ", str.bytes().len()),
                CountOptions::Chars => print!("Characters: {} ", str.chars().count()),
                CountOptions::Words => print!("Words: {} ", str.split_ascii_whitespace().count()),
                CountOptions::Lines => print!("Lines: {} ", str.lines().count()),
            }
        }
        println!()
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let config = Config::new(&args).unwrap_or_else(|| {
        eprintln!("Wrong arguments provided");
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
