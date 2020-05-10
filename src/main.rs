use std::env;
use std::fs;

fn main() {
    println!("Starting minigrep");
    let args = env::args().collect::<Vec<String>>();

    let config = parse_config(&args);

    let content = fs::read_to_string(config.filename).expect("something went wrong");

    println!("file content \n {}", content);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    Config {
        query: args[1].clone(),
        filename: args[2].clone(),
    }
}
