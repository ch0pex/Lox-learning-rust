use crate::token::Token;
pub enum LoxResult{
    RunTimeError { token: Token, message: String},
    Error {line: usize, message: String},
}



impl LoxResult{ 
    pub fn error(line: usize, message: &String) -> LoxResult{ 
        let err = LoxResult::Error { 
            line,
            message: message.to_string()
        };
        err.report();
        err
    }

    fn report(&self) {
        match self { 
            LoxResult::Error {line, message} => {
                eprintln!("[line {}] Error: {}", line, message); 
            }
            LoxResult::RunTimeError { token, message } => todo!(),
        }
    }

}