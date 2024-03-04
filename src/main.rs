use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing command line arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(cfg) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
