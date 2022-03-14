use std::{
    error::Error,
    fmt::Display,
    process,
    fs,
};
use colored::*;

pub struct Config<'a> {
    pub regex_query: &'a String,
    pub filename: &'a String,
}

impl Config<'_> {
    pub fn new(system_args: &[String]) -> Result<Config, &'static str> {
        if system_args.len() < 3 {
            return Err("insufficient amount of passed arguments");
        }

        let regex_query: &String = &system_args[1];
        let filename: &String = &system_args[2];

        Ok(Config {
            regex_query,
            filename
        })
    }
}

pub fn run_process(program_config: &Config) -> Result<(), Box<dyn Error>> {
    let file_contents: String = fs::read_to_string(&program_config.filename)?;
    let regex_query_results: Vec<&str> = search(&program_config.regex_query, &file_contents);
    print_results(&regex_query_results);
    Ok(())
}

fn search<'a>(regex_query: &str, file_contents: &'a str) -> Vec<&'a str> {
    file_contents.lines()
        .filter(|line| line.contains(regex_query))
        .collect()
}

fn print_results(regex_query_results: &Vec<&str>) {
    for result in regex_query_results.iter() {
        println!("{}", result);
    }
}

pub fn fsrep_failure(error_flag: impl Display) {
        println!("fsrep failure: {}", error_flag);
        process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_founded() {
        let query: &str = "Lorem";
        let contents: &str = "\
            Lorem
            search
            and
            destroy";

        assert_eq!(vec!["Lorem"], search(&query, &contents));
    }
}
