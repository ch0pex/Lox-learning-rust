use std::env::args;
use std::io::{self, Write};
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

fn run_file(file_path: &String) -> io::Result<()>{
    let buf = std::fs::read_to_string(file_path)?; 
    match run(buf.as_str()) { 
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(65),
        //_ => std::process::exit(65)
    }
}

fn run_prompt() {
    let mut line = String::new();
    loop{ 
        print!("> ");
        let _ = io::stdout().flush();
        if io::stdin().read_line(&mut line).expect("Error reading prompt") <= 2{
            break;
        }
        let _ = run(line.as_str());
    }
}

fn run(source: &str) -> Result<(), LoxResult> {
    println!("{}", source.to_string());
    let _ = io::stdout().flush();
    let mut scanner = Scanner::new(source);
    let tokens: &Vec<Token> = scanner.scan_tokens()?;


    for token in tokens{
        println!("{}", token);
    }
    Ok(())
}