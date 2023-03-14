use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args : Vec<String> = env::args().collect();

    let c = Config::build(&args).
    unwrap_or_else(|err| {
        println!("Could not parse args: error - {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(c) {
            println!("Application error: {e}");
            process::exit(1)
    }
    
}

