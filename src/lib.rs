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

pub struct Config {
    pub query: String,
    pub filename : String,
    pub case_sensitive: bool,
}
impl<'a> Config  {
    pub fn new(mut args: env::Args) -> Result<Config, & 'static str> {
        args.next();

        let query =match args.next(){
            Some(arg) =>arg,
            None=> return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None=> return Err("The file doesn't exists in the given location"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();     
    
        return Ok(Config { query, filename, case_sensitive})
    }
}

pub fn search <'a>(query: &str, content: & 'a str) -> Vec<& 'a str> {
    // let mut result:Vec<&str> = Vec::new(); 
    // for line in content.lines(){
    //     if line.contains(query) {
    //         result.push(line);
    //     }   
    // }
    // return  result;
    content.lines().into_iter().filter(|line| {
        line.contains(query)    
    }).collect()

}

pub fn search_case_insensitve <'a>(query: &str, content: & 'a str)-> Vec<& 'a str> {
    // let mut result:Vec<&str> = Vec::new();
    // let new_query = query.to_lowercase();
    // for line in content.lines(){
    //     if line.to_lowercase().contains(&new_query){
    //         result.push(line);
    //     }
    // }
    // return result;

    let new_query= query.to_lowercase();
    content.lines().into_iter().filter(|line|{
        line.contains(&new_query)
    }).collect()
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
    


