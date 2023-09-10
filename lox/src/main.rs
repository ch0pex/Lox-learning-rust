use std::env::args;
use std::io::{self, Write, BufRead};
use lox_syntax::scanner::Scanner;
use lox_syntax::token::Token;
use lox_syntax::error::LoxResult;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 2{
        std::process::exit(64);
    } else if args.len() == 2 { 
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt();
    }
}

fn run_file(file_path: &String) -> io::Result<()> {
    let buf = std::fs::read_to_string(file_path)?; 
    match run(buf.as_str()) { 
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(65),
        //_ => std::process::exit(65)
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    let _ = io::stdout().flush();
    for line in stdin.lock().lines() {
        match line{ 
            Ok(promt) => {
                let _ = run(&promt.as_str());
                print!("> ");
                let _ = io::stdout().flush();
            }
            Err(_) => std::process::exit(0)
        }
    }
}

fn run(source: &str) -> Result<(), LoxResult> {
    let mut scanner = Scanner::new(source);
    let tokens: &Vec<Token> = scanner.scan_tokens()?;
    for token in tokens{
        println!("{}", token);
    }
    Ok(())
}
