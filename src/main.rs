use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args : Vec<String> = env::args().collect();

    let c = Config::build(&args).
    unwrap_or_else(|err| {
        println!("Could not parse args: error - {err}");
        process::exit(1);
    });

    if let Err(e) = run(c) {
            println!("Application error: {e}")
    }
    
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args.")
        }
        Ok(Config{query: args[1].clone(), filename: args[2].clone() })
    }
}

fn run(c: Config) -> Result<(), Box<dyn Error>> {
    // error return
    let file_contents: String = fs::read_to_string(c.filename)?;
    println!("{}", file_contents);
    Ok(())
}