use std::{env, fs, process};

fn main() {
    // allows your minigrep program to read any command line arguments passed to it and then collect the values into a vector.
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Proplem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text \n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // Returning a Result from Config::build
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
