use std::{
    env,
    error::Error,
    fmt::Display,
    process,
    fs,
};

fn main() {
    let system_args: Vec<String> = env::args().collect();
    let program_config = Config::new(&system_args).unwrap_or_else(|error_flag| {
        fsrep_failure(error_flag);
        process::exit(1);
    });

    println!("Welcome to fsrep");
    println!("Args: {:?}", system_args);
    println!("Searching for {}", program_config.regex_query);
    println!("In file: {}", program_config.filename);

    run_process(&program_config).unwrap_or_else(|error_flag| fsrep_failure(error_flag));

}

struct Config<'a> {
    regex_query: &'a String,
    filename: &'a String,
}

impl Config<'_> {
    fn new(system_args: &[String]) -> Result<Config, &'static str> {
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

fn run_process(program_config: &Config) -> Result<(), Box<dyn Error>> {
    let file_contents: String = fs::read_to_string(&program_config.filename)?;
    println!("File contents: \n{}", &file_contents);
    Ok(())
}

fn fsrep_failure(error_flag: impl Display) {
        println!("fsrep failure: {}", error_flag);
        process::exit(1);
} 
