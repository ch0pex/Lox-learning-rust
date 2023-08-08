use std::{fmt::{self, write}, io::LineWriter};

#[derive(Debug)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    Bang, BangEqual,
    Assign, Equals,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier, String, Number,

    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "{x}"),
            Object::Nil => write!(f, "Nil"),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false"),
        }
    }
}

pub struct Token { 
    ttype: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize
}

impl Token { 
    pub fn eof(line: usize) -> Token {
        Token {
            ttype: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lit = String::new();
        match &self.literal {
            Some(literal) => {lit = literal.to_string()} 
            None => { lit = "None".to_string()}
        }
        write!(f, "{:?} {} {}", self.ttype, self.lexeme, lit)
    }
}