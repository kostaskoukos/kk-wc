use std::process;

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

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let config = Config::new(&args).unwrap_or_else(|| {
        eprintln!("Wrong arguments provided");
        process::exit(1);
    });
}
