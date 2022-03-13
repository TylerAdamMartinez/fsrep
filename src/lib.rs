use std::{
    error::Error,
    fmt::Display,
    process,
    fs,
};

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
    println!("File contents: \n{}", &file_contents);
    Ok(())
}

pub fn fsrep_failure(error_flag: impl Display) {
        println!("fsrep failure: {}", error_flag);
        process::exit(1);
}
