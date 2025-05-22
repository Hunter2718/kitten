use std::fs;
use std::io::{self, Error, BufRead, IsTerminal, ErrorKind};
use std::path::Path;
use std::env;

const HELP_FILE_PATH: &'static str = "assets/helpfile.txt";
const VERSION_INFO_FILE_PATH: &'static str = "assets/versionfile.txt";

struct Config {
    show_help: bool,
    show_version: bool,
    show_line_numbers: bool,
    show_line_numbers_nonblank: bool,
    squeeze_blank: bool,
    show_ends: bool,
    show_tabs: bool,
    show_nonprint: bool,
    files: Vec<String>,
}

fn main() -> Result<(), Error> {
    let mut output: String = String::new();
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);

    if config.show_help {
        println!("{}", read_file(HELP_FILE_PATH)?);
        return Ok(());
    }

    if config.show_version {
        println!("{}", read_file(VERSION_INFO_FILE_PATH)?);
        return Ok(());
    }

    
    if config.files.is_empty() {

        output.push_str( 
            &match read_stdin() {
                Err(e) => return Err(e),
                Ok(c) => c,
            }
        );


    } else {
    
        for file in config.files {

            if file == "-" {
                output.push_str(
                    &match read_stdin() {
                        Err(e) => return Err(e),
                        Ok(c) => c,
                    }
                );


            } else {
                output.push_str(
                    &match read_file(&file) {
                        Err(e) => {
                            println!("{}",
                                match read_file(&HELP_FILE_PATH.to_string()) {
                                    Err(e) => return Err(e),
                                    Ok(c) => c,
                                }  
                            );
                            return Err(e);
                        },

                        Ok(c) => c,
                    }
                );


            }

        }
    }

    
    if config.squeeze_blank {
        output = add_squeeze_blank(output);
    }

    if config.show_tabs {
        output = add_show_tabs(output);
    }

    if config.show_nonprint {
        output = add_show_nonprint(output);
    }

    if config.show_ends {
        output = add_show_ends(output);
    }

    if config.show_line_numbers {
        output = add_line_numbers(output);
    } 

    if config.show_line_numbers_nonblank {
        output = add_line_numbers_nonblank(output);
    }



    print!("{}", output);

    Ok(())
}


fn read_file(path_to_file: &str) -> Result<String, Error> {
    return fs::read_to_string(Path::new(path_to_file));
}


fn read_stdin() -> Result<String, Error> {
    let mut result: String = String::new();
    let stdin = io::stdin();

    if stdin.is_terminal() {
        println!("{}", 
            match read_file(HELP_FILE_PATH) {
                Err(e) => return Err(e),
                Ok(c) => c,
            }
        );
        return Err(Error::new(ErrorKind::Other, "Stdin is a TTY"));
    }

    for line in stdin.lock().lines() {
        let line = line?;
        result.push_str(&line);
        result.push('\n');
    }

    return Ok(result);
}


fn parse_args(args: &[String]) -> Config {
    let mut config = Config {
        show_help: false,
        show_version: false,
        show_line_numbers: false,
        show_line_numbers_nonblank: false,
        squeeze_blank: false,
        show_ends: false,
        show_tabs: false,
        show_nonprint: false,
        files: Vec::new(),
    };

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--help" => config.show_help = true,
            "--version" => config.show_version = true,
            "-n" | "--number" => config.show_line_numbers = true,
            "-b" | "--number-nonblank" => config.show_line_numbers_nonblank = true,
            "-s" | "--squeeze-blank" => config.squeeze_blank = true,
            "-E" | "--show-ends" => config.show_ends = true,
            "-T" | "--show-tabs" => config.show_tabs = true,
            "-v" | "--show-nonprinting" => config.show_nonprint = true,
            "-A" | "--show-all" => {
                config.show_nonprint = true;
                config.show_ends = true;
                config.show_tabs = true;
            },
            "-e" => {
                config.show_nonprint = true;
                config.show_ends = true;
            },
            "-t" => {
                config.show_nonprint = true;
                config.show_tabs = true;
            },
            "-" => config.files.push(arg.to_string()),
            _ if arg.starts_with('-') => {
                config.show_help = true;
                break;
            }
            _ => config.files.push(arg.to_string()),
        }
    }

    if config.show_line_numbers_nonblank && config.show_line_numbers {
        config.show_line_numbers = false;
    }

    config
}


fn add_line_numbers(input: String) -> String {
    let mut result = String::new();
    for (i, line) in input.lines().enumerate() {
        result.push_str(&format!("{:>6}  {}\n", i + 1, line));
    }
    result
}


fn add_line_numbers_nonblank(input: String) -> String {
    let mut result = String::new();
    let mut line_number = 1;

    for line in input.lines() {
        if line.is_empty() {
            result.push_str("\n");
            continue;
        }

        result.push_str(&format!("{:>6}  {}\n", line_number, line));
        line_number += 1;
    }
    result
}


fn add_squeeze_blank(input: String) -> String {
    let mut result = String::new();
    let mut previous_blank = false;

    for line in input.lines() {
        if line.trim().is_empty() {
            if previous_blank {
                continue;
            } else {
                result.push_str("\n");
                previous_blank = true;
            }

        } else {
            result.push_str(&format!("{}\n", line));
            previous_blank = false;
        }
    }

    result
}


fn add_show_ends(input: String) -> String {
    let mut result = String::new();
    for line in input.lines() {
        result.push_str(&format!("{}$\n", line));
    }
    result
}


fn add_show_tabs(input: String) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c == '\t' {
            result.push_str("^I");
        } else {
            result.push(c);
        }
    }
    result
}


fn add_show_nonprint(input: String) -> String {
    let mut result = String::new();

    for c in input.chars() {
        if c == '\t' || c == '\n' {
            result.push(c);
        } else {
            let ascii = c as u32;

            if ascii < 32 {
                result.push('^');
                result.push((ascii + 64) as u8 as char);
            } else if ascii == 127 {
                result.push('^');
                result.push('?');
            } else {
                result.push(c);
            }
        }
    }

    result
}
