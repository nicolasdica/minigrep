use std::{fs, process, error::Error};
use minigrep::{search, search_case_insensitive};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    query: String,

    #[arg(short, long)]
    file_path: String,

    #[arg(short, long, default_value_t = false)]
    ignore_case: bool,
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}


fn main() {
    let args_aux = Args::parse();

    let config = Config::build(&args_aux).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error value from the current function for the caller to handle.
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

impl Config {
    fn build(args: &Args) -> Result<Config, &'static str> {
        let query = args.query.clone();
        let file_path = args.file_path.clone();
        let ignore_case = args.ignore_case.clone();

        Ok(Config { query, file_path, ignore_case })
    }
}
