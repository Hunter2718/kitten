use std::env;
use std::fs;
use std::io::{self, Error, BufRead, IsTerminal, Write};
use std::path::Path;
use std::os::unix::io::AsRawFd;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {

        let stdin = io::stdin();

        if stdin.is_terminal() {
            eprintln!("Must have a file or stdin with pipes");
            std::process::exit(1);
        }

        for line in stdin.lock().lines() {
            let line = line?;
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
