
use std::f32::consts::E;

use crate::error::LoxResult;
use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner{
    pub fn new(source: &str) -> Scanner{ 
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),   
            start: 0,
            current: 0,
            line: 1 
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxResult> {
        let mut had_error: Option<LoxResult> = None;
        while self.at_end() == false{
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {},
                Err(e) => {
                    had_error = Some(e);
                }
            }
        };

        self.tokens.push(Token::eof(self.line));
        
        match had_error {
            Some(e) => Err(e),
            None => Ok(&self.tokens)
        }
    }

    fn scan_token(&mut self) -> Result<(),LoxResult>{ 
       let character: char = self.advance(); 
       match character {
          '(' => {self.add_token(TokenType::LeftParen)},
          ')' => {self.add_token(TokenType::RightParen)},
          '{' => {self.add_token(TokenType::LeftBrace)},
          '}' => {self.add_token(TokenType::RightBrace)},
          ',' => {self.add_token(TokenType::Comma)},
          '.' => {self.add_token(TokenType::Dot)},
          '-' => {self.add_token(TokenType::Minus)},
          '+' => {self.add_token(TokenType::Plus)},
          ';' => {self.add_token(TokenType::Semicolon)},
          '*' => {self.add_token(TokenType::Star)},
           _  => {return Err(LoxResult::error(self.line, &" ".to_string()))}
       };
       Ok(())
    }

    fn add_token(&mut self, ttype: TokenType) {
        
    }

    fn advance(&mut self) -> char{ 
        self.current += 1; 
        self.source.chars().nth(self.current).unwrap()
    }

    fn at_end(&self) -> bool{
        self.current >= self.source.len()
    }
}