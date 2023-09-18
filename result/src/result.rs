//use lox_syntax::token::Token;

#[derive(Debug)]
pub enum LoxResult{
    RunTimeError {line: usize, message: String},
    Error {line: usize, message: String},
}

impl LoxResult{
    pub fn error(line: usize, message: &str) -> LoxResult{
        let err = LoxResult::Error {
            line,
            message: message.to_string()
        };
        err.report();
        err
    }

    fn report(&self) {
        match self {
            LoxResult::Error {line, message} => eprintln!("[line {}] Error: {}", line, message),
            LoxResult::RunTimeError { line, message } => eprintln!("Runtime error [line {}]: {} ", line, message),
        }
    }
}
