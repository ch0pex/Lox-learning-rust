//use lox_syntax::token::Token;

#[derive(Debug)]
pub enum LoxResult{
    Error {line: usize, message: String},
    RunTimeError {line: usize, message: String},
    ParseError {line: usize, lexeme: String, message: String}
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

    pub fn parse_error(line: usize, message: &str, lexeme: &str) -> LoxResult {
        let lexeme_location= if lexeme == "" {"end"} else {lexeme};
        let err = LoxResult::ParseError {
            line,
            message: message.to_string(),
            lexeme: lexeme_location.to_string()
        };
        err.report();
        err
    }

    pub fn run_time_error(line: usize, message: &str) -> LoxResult {
        let err = LoxResult::RunTimeError {
            line,
            message: message.to_string()
        };
        err.report();
        err
    }

    fn report(&self) {
        match self {
            LoxResult::Error {line, message} => eprintln!(" Error [line {line}]: {message}"),
            LoxResult::RunTimeError { line, message } => eprintln!("Runtime error [line {line}]: {message} "),
            LoxResult::ParseError {line, lexeme, message} => eprintln!("Parse error [line {line}]: {message} at {lexeme}")
        }
    }
}
