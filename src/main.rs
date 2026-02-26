use clap::Parser;
use minigrep::{search, search_case_insensitive};
use owo_colors::OwoColorize;
use std::{error::Error, fs, process};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    query: String,

    #[arg(short, long)]
    file_path: String,

    #[arg(short = 'n', long, default_value_t = false)]
    line_number: bool,

    #[arg(short, long, default_value_t = false)]
    ignore_case: bool,
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
    line_number: bool,
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

    for (line_num, line) in results {
        if config.line_number {
            let line_lowercase = &line.to_lowercase().to_string();
            let mut matches: Vec<(usize, &str)> = line_lowercase.match_indices(&config.query.to_lowercase()).collect();
            matches.reverse();

            let mut aux = String::from(line);
            
            for (start, word) in matches {
                let length = start + word.len() - 1;
                let aux_slice = &line[start..=length];
                aux.replace_range(start..=length, &aux_slice.red().to_string());
            }

            println!(
                "{}:{}",
                line_num.green(),
                aux
            );
        } else {
            let line_lowercase = &line.to_lowercase().to_string();
            let mut matches: Vec<(usize, &str)> = line_lowercase
                .match_indices(&config.query.to_lowercase())
                .collect();
            matches.reverse();

            let mut aux = String::from(line);

            for (start, word) in matches {
                let length = start + word.len() - 1;
                let aux_slice = &line[start..=length];
                aux.replace_range(start..=length, &aux_slice.red().to_string());
            }

            println!("{}", aux);
        }
    }

    Ok(())
}

impl Config {
    fn build(args: &Args) -> Result<Config, &'static str> {
        let query = args.query.clone();
        let file_path = args.file_path.clone();
        let ignore_case = args.ignore_case.clone();
        let line_number = args.line_number.clone();

        Ok(Config {
            query,
            file_path,
            ignore_case,
            line_number,
        })
    }
}
