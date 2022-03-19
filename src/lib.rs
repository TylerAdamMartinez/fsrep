//! #fsrep
//!
//! `fsrep` is a command line tool. A grep like alternative written in rust

use std::{
    error::Error,
    fmt::Display,
    process,
    thread,
    thread::JoinHandle,
    fs,
};
use colored::*;
use regex::Regex;

pub struct Config {
    pub regex_query: String,
    pub filenames: Vec<String>,
}

impl Config {
    pub fn new(mut system_args: std::env::Args) -> Result<Config, &'static str> {
        system_args.next();

        let regex_query: String = match system_args.next() {
            Some(regex_query) => regex_query,
            None => return Err("insufficient amount of passed arguments: needs a query string"),
        };

        let mut filenames: Vec<String> = Vec::<String>::new();
        match system_args.next() {
            Some(filename) => filenames.push(filename),
            None => return Err("insufficient amount of passed arguments: needs at least one file argument"),
        };

        loop {
            match system_args.next() {
                Some(filename) => filenames.push(filename),
                None => break,
            }
        }

        Ok(Config {
            regex_query,
            filenames
        })
    }
}

pub fn run_process(program_config: Config) -> Result<(), Box<dyn Error + 'static>> {
    let regex_query_expression: Regex = create_regex_expression(&program_config.regex_query)?;
    let mut thread_join_handler: Vec<JoinHandle<_>> = Vec::<JoinHandle<_>>::new();

    for filename in program_config.filenames.into_iter() {
        let regex_query_expression_clone = regex_query_expression.clone();
        thread_join_handler.push(thread::spawn(move || {
            let file_contents: String = fs::read_to_string(&filename)
                .unwrap_or_else(|error_flag| {
                    fsrep_failure(error_flag, Some(&filename));
                    process::exit(1);
                });
            let regex_query_results: Vec<SearchResults> = search(&regex_query_expression_clone, file_contents);
            print_results(&filename, &regex_query_expression_clone, &regex_query_results);
        }));
    }

    for thread_join_handle in thread_join_handler {
        thread_join_handle.join()
            .expect(&format!("{}{}", "fsrep failure".red().bold(), ": thread panic"));
    }
    Ok(())
}

fn create_regex_expression(regex_query: &str) -> Result<Regex, Box<dyn Error>> {
    let regex_query_expression: Regex = Regex::new(regex_query)?;
    Ok(regex_query_expression)
}

#[derive(Debug, PartialEq)]
struct SearchResults {
    line_number: u128,
    line_content: String,
}

fn search<'a>(regex_query_expression: &Regex, file_contents: String) -> Vec<SearchResults> {
    let mut line_number: u128 = 0;
    file_contents.lines()
        .map(|line_content| {
            line_number += 1;
            SearchResults {
                line_number,
                line_content: line_content.to_string(),
            }
        })
        .filter(|search_result| regex_query_expression.is_match(&search_result.line_content))
        .collect()
}

fn print_results(filename: &String, regex_query_expression: &Regex, regex_query_results: &Vec<SearchResults>) {
    let fsrep_success_msg = "fsrep success".green().bold();
    let number_query_of_results = regex_query_results.len().to_string().green().bold();
    println!("{}: In file: '{}' {} matches found", fsrep_success_msg, filename, number_query_of_results);

    for result in regex_query_results.iter() {
        let colorised_result_line_content = highlight_regex_result(&regex_query_expression, &result.line_content);
        println!("{}: {}", result.line_number.to_string().cyan().bold(), colorised_result_line_content);
    }
}

fn highlight_regex_result(regex_query_expression: &Regex, line_content: &String) -> String {
    let query_match = regex_query_expression.find(&line_content).unwrap();

    if query_match.start() == 0 {
        let color_match = &line_content[query_match.start()..query_match.end()].green().bold();
        let end = &line_content[query_match.end()..];
        return format!("{}{}", &color_match, &end);
    }
    else if query_match.end() == line_content.len() {
        let begin = &line_content[0..query_match.start()];
        let color_match = &line_content[query_match.start()..query_match.end()].green().bold();
        return format!("{}{}", &begin, &color_match);
    }
    else {
        let begin = &line_content[0..query_match.start()];
        let end = &line_content[query_match.end()..];
        let color_match = &line_content[query_match.start()..query_match.end()].green().bold();
        return format!("{}{}{}", &begin, &color_match, &end);
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
    #[should_panic(expected = "assertion failed: `(left == right)`\n  left: `[SearchResults { line_number: 16, line_content: \"Black\" }]`,\n right: `[]`")]
    fn search_not_found() {
        let query: &Regex = &create_regex_expression(&"Black").unwrap();
        let contents: &str = "\
            Itachi
            Madara
            Obito
            Pain
            Orochimaru
            Shin
            Konan
            Kakuzu
            Kisame
            Deidara
            Sasori
            Hidan
            Yahiko";

        assert_eq!(vec![SearchResults{ line_number: 16, line_content: "Black" }], search(&query, &contents));
    }


    #[test]
    fn search_found() {
        let query: &Regex = &create_regex_expression(&"Orochimaru").unwrap();
        let contents: &str = "\
            Itachi
            Madara
            Obito
            Pain
            Orochimaru
            Shin
            Konan
            Kakuzu
            Kisame
            Deidara
            Sasori
            Hidan
            Yahiko";

        assert_eq!(vec![SearchResults{ line_number: 5, line_content: "            Orochimaru" }], search(&query, &contents));
    }
}
