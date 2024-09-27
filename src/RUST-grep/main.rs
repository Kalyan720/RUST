use std::env;
use std::fs::File;
use std::io::prelude::*;

// Structure maintains the input Arguments
struct Config{
    search_term : String,
    file_name : String
}

// Assign the args to their respective places
fn parse_args(args:&[String]) -> Config{
    let search_term = args[1].clone();
    let file_name = args[2].clone();
    Config {search_term,file_name}
}

// Hero-Section
fn main() {
    let args:Vec<String> = env::args().collect();
    println!("let us watch what happes : {:?}",args);
    
    let config = parse_args(&args);
    println!("We are searching for {} in the {}", config.search_term, config.file_name);

    let mut file = File::open("poem.txt").expect("there is an error in loading the file");
    let mut contents:String = String::new();
    file.read_to_string(&mut contents).expect("there is an error while reading the File");
    print!("the file contains : {}", contents);
}
