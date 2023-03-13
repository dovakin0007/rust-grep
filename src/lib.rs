use std::error::Error;
use std::{fs, env,};


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    }else{
        search_case_insensitve(&config.query, &contents)
    };

    for line in result{
        println!("{}", line);
    };
    Ok(())
}

pub struct Config <'a>{
    pub query: & 'a str,
    pub filename : & 'a str,
    pub case_sensitive: bool,
}
impl<'a> Config <'a> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() > 3 {
            return Err("not enough arguments")
        }
        let query = &args[1];
        let filename = &args[2];
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();     
    
        return Ok(Config { query, filename, case_sensitive})
    }
}

pub fn search <'a>(query: &str, content: & 'a str) -> Vec<& 'a str> {
    let mut result:Vec<&str> = Vec::new(); 
    for line in content.lines(){
        if line.contains(query) {
            result.push(line);
        }   
    }
    return  result;
}

pub fn search_case_insensitve <'a>(query: &str, content: & 'a str)-> Vec<& 'a str> {
    let mut result:Vec<&str> = Vec::new();
    let new_query = query.to_lowercase();
    for line in content.lines(){
        if line.to_lowercase().contains(&new_query){
            result.push(line);
        }
    }
    return result;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitve(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape" ;

        assert_eq!(vec!["safe, fast, productive."], search(&query, &contents));
    }


    #[test]
    fn case_insensitve(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me" ;

        assert_eq!(vec!["Rust:", "Trust me"], search_case_insensitve(&query, &contents));
    }
}
    


