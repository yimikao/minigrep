use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();

    let _query = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).expect("file could not be read");

    println!("{}", file_contents);
}
