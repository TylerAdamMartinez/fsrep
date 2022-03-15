use std::{
    error::Error,
    fmt::Display,
    process,
    fs,
};
use colored::*;
use regex::Regex;

pub struct Config {
    pub regex_query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut system_args: std::env::Args) -> Result<Config, &'static str> {
        system_args.next();

        let regex_query: String = match system_args.next() {
            Some(regex_query) => regex_query,
            None => return Err("insufficient amount of passed arguments: needs a query string"),
        };

        let filename: String = match system_args.next() {
            Some(filename) => filename,
            None => return Err("insufficient amount of passed arguments: needs at least one file argument"),
        };

        Ok(Config {
            regex_query,
            filename
        })
    }
}

pub fn run_process(program_config: &Config) -> Result<(), Box<dyn Error>> {
    let file_contents: String = fs::read_to_string(&program_config.filename)
        .unwrap_or_else(|error_flag| {
            fsrep_failure(error_flag, Some(&program_config.filename));
            process::exit(1);
        });
    let regex_query_expression: Regex = create_regex_expression(&program_config.regex_query)?;
    let regex_query_results: Vec<&str> = search(&regex_query_expression, &file_contents);
    print_results(&program_config.filename, &regex_query_results);
    Ok(())
}

fn create_regex_expression(regex_query: &str) -> Result<Regex, Box<dyn Error>> {
    let regex_query_expression: Regex = Regex::new(regex_query)?;
    Ok(regex_query_expression)
}

fn search<'a>(regex_query_expression: &Regex, file_contents: &'a str) -> Vec<&'a str> {
    file_contents.lines()
        .filter(|line| regex_query_expression.is_match(line))
        .collect()
}

fn print_results(filename: &String, regex_query_results: &Vec<&str>) {
    let fsrep_success_msg = "fsrep success".green().bold();
    println!("{}: In file: '{}' {} matches found", fsrep_success_msg, filename, regex_query_results.len().to_string().green().bold());
    for result in regex_query_results.iter() {
        println!("{}", result);
    }
}

pub fn fsrep_failure(error_flag: impl Display, additonal_info: Option<&str>) {
    let fsrep_failure_msg = "fsrep failure".red().bold();
    match additonal_info {
        Some(additonal_info) => { 
            eprintln!("{}: '{}' {}", fsrep_failure_msg, additonal_info, error_flag);
            process::exit(1);
        },
        None => {
            eprintln!("{}: {}", fsrep_failure_msg, error_flag);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[\"Lorem\"]`,\n right: `[]`")]
    fn search_not_founded() {
        let query: &Regex = &create_regex_expression(&"Spider-Man").unwrap();
        let contents: &str = "\
            Lorem
            search
            and
            destroy";

        assert_eq!(vec!["Lorem"], search(&query, &contents));
    }


    #[test]
    fn search_founded() {
        let query: &Regex = &create_regex_expression(&"Lorem").unwrap();
        let contents: &str = "\
            Lorem
            search
            and
            destroy";

        assert_eq!(vec!["Lorem"], search(&query, &contents));
    }
}
