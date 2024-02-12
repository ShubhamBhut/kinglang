use std::{env, process::exit, fs, io::{self, BufRead}};

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&contents),
    }
}

fn run(contents: &str) ->Result<(), String>{
    return Err("Not implimented!".to_string());
}

fn run_prompt() -> Result<(), String> {
    println!("> ");
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    match handle.read_line(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err("Could not read line".to_string())
    }
    println!("You wrote: {}", buffer);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 2 {
        println!("Usage: kinglang [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n{}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("ERROR:\n{}", msg);
                exit(1)
            }
        }
    }
}
