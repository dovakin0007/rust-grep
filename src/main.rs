use std::env;
// use std::fs;
use std::process;
// use std::error::Error;
use mini_grep::{ Config , run};

fn main(){    
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("{}", err);
        process::exit(1);
    });
    println!("Searching for {:?}", config.query);
    println!("in file {:?}", config.filename);

    if let Err(e) = run(config){
        eprintln!("Application Error {}", e);
        process::exit(1);
    };


}
