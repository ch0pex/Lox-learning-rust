use interpreter::interpreter::Interpreter;
use lox_syntax::scanner::Scanner;
use parser::parser::Parser;
use result::result::LoxResult;
use std::env::args;
use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt();
    }
}

fn run_file(file_path: &str) -> io::Result<()> {
    let buf = std::fs::read_to_string(file_path)?;
    match run(&buf) {
        Ok(_) => std::process::exit(0),
        Err(error) => match error {
            LoxResult::Error { .. } => std::process::exit(65),
            LoxResult::RunTimeError { .. } => std::process::exit(70),
            LoxResult::ParseError { .. } => std::process::exit(65),
        },
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    let _ = io::stdout().flush();
    for line in stdin.lock().lines() {
        match line {
            Ok(prompt) => {
                let _ = run(&prompt);
                print!("> ");
                let _ = io::stdout().flush();
            }
            Err(_) => std::process::exit(0),
        }
    }
}

fn run(source: &str) -> Result<(), LoxResult> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;
    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&statements)?;

    Ok(())
}

