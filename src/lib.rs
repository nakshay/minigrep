use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    
   pub fn new(args: &[String]) -> Result<Config, &'static str> {

    if args.len() < 3 {
        return Err("not enough arguments"); 
    }

    let config  =  Config {
        query: args[1].clone(),
        filename: args[2].clone(),
    };
    
    Ok(config)
   
   }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
   
    let content = fs::read_to_string(config.filename)?;

    println!("file content \n {}", content);

    Ok(())
}


