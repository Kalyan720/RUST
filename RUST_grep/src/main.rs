use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

// Structure maintains the input Arguments
struct Config{
    search_term : String,
    file_name : String
}

// Creating a constructor for config
impl Config{
    fn parse_args(args:&[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("Not Enough arguments : -_-");
        }
        let search_term = args[1].clone();
        let file_name = args[2].clone();
        Ok(Config {search_term,file_name})
    }
}


// Hero-Section
fn main() {
    let args:Vec<String> = env::args().collect();
    println!("let us watch what happes : {:?}",args);
    
    let config = Config::parse_args(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("We are searching for {} in the {}", config.search_term, config.file_name);

    if let Err(e) = run(config){
        println!("Application Error : {}", e);
        process::exit(1);
    };
}

// Other logic lying in the main function
fn run(_config: Config) -> Result<(), Box<dyn Error>>{
    let mut file = File::open("poem.txt")?;
    let mut contents:String = String::new();
    file.read_to_string(&mut contents)?;
    print!("the file contains : {}", contents);
    Ok(())
}
