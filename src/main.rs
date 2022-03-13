use std::{
    env,
    process,
};
use fsrep::{
    Config,
    fsrep_failure,
    run_process,
};

fn main() {
    let system_args: Vec<String> = env::args().collect();
    let program_config = Config::new(&system_args).unwrap_or_else(|error_flag| {
        fsrep_failure(error_flag);
        process::exit(1);
    });

    run_process(&program_config).unwrap_or_else(|error_flag| fsrep_failure(error_flag));
}
