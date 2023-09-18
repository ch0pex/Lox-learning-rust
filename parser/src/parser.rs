use lox_syntax::token::{Token, TokenType};

struct Parser {
    tokens: Vec<Token>,
    current: i32
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0
        }
    }
}
