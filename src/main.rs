use fsrep::{
    Config,
    fsrep_failure,
    run_process,
};

fn main() {
    let program_config = Config::new(std::env::args()).unwrap_or_else(|error_flag| {
        fsrep_failure(error_flag, None);
        std::process::exit(1);
    });

    run_process(&program_config).unwrap_or_else(|error_flag| fsrep_failure(error_flag, None));
}
