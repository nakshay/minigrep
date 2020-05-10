use std::process;
use minigrep::Config;
use std::env;

fn main() {
    println!("Starting minigrep");

    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else( |err| {
        println!("problem parsing arguments {}!",err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error {}",e);
        process::exit(1);
    }
}


