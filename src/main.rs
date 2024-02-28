use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing command line arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(cfg) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
