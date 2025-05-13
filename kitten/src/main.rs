use std::env;
use std::fs;
use std::io::{self, Error};
use std::path::Path;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough Args");
        std::process::exit(1);
    }

    let content = match fs::read_to_string(Path::new(&args[1])) {
        Err(e) => return Err(e),
        Ok(c) => c
    };


    println!("{content}");

    Ok(())
}
