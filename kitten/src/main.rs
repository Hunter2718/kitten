use std::env;
use std::fs;
use std::io::{self, Error, BufRead};
use std::path::Path;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {

        // TODO: Why does it not end until I manually put EOF with ctrl + D
        for line in io::stdin().lock().lines() {
            let line = line.expect("Failed to read line");
            println!("{}", line);
        }

    } else {
    
        let mut i = 1;

        while i < args.len() {

        println!("{}",
                match fs::read_to_string(Path::new(&args[i])) {
                    Err(e) => return Err(e),
                    Ok(c) => c,
             }
         );

          i += 1;
        }
    }

    Ok(())
}
