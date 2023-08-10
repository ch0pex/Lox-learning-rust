use crate::error::LoxResult;
use crate::token::{Token, TokenType, Object};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner{
    pub fn new(source: &str) -> Scanner{ 
        Scanner {
            source: source.chars().collect(),
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
        let character = self.advance(); 
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
            '!' => {if self.expect('='){self.add_token(TokenType::BangEqual)} else {self.add_token(TokenType::Bang)}},
            '=' => {if self.expect('='){self.add_token(TokenType::Equals)} else {self.add_token(TokenType::Assign)}},
            '<' => {if self.expect('='){self.add_token(TokenType::LessEqual)} else {self.add_token(TokenType::Less)}},
            '>' => {if self.expect('='){self.add_token(TokenType::GreaterEqual)} else {self.add_token(TokenType::Greater)}}
            '"' => {self.string()?}
            '\n' => {self.line += 1}
            '\r' => {},
            '\t' => {},
            ' ' => {},
            '/' => {
                match self.expect('/'){ 
                    true => {
                        while self.peek() != '\n' && !self.at_end() { self.advance(); }
                    }
                    false => self.add_token(TokenType::Slash)

                }
            }
            _  => {return Err(LoxResult::error(self.line, "Unexpected character."))}
        };
        Ok(())
    }


    fn add_token(&mut self, ttype: TokenType) {
        self.add_token_object(ttype, None );
    }

    fn add_token_object(&mut self, ttype: TokenType, obj: Option<Object>){ 
        let text = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(ttype, text, obj, self.line));
    }

    pub fn advance(&mut self) -> &char{ 
        self.current += 1;
        self.source.get(self.current - 1).unwrap()
    }

    fn at_end(&self) -> bool{
        self.current >= self.source.len()
    }

    fn peek(&self ) -> char {
        if self.at_end() {
            '\0'
        }
        else{
            *self.source.get(self.current).unwrap()
        }
    }

    fn expect(&mut self, expected: char) -> bool{
        match self.source.get(self.current){
            Some(ch) => {
                if expected == *ch { 
                    self.current += 1;
                    true
                } else {
                    false
                }
            }
            None => false
        }
    }

    fn string(&mut self) -> Result<(), LoxResult>{ 
        while self.peek() != '"' && !self.at_end() { 
            if self.peek() == '\n'{ 
                self.line += 1;
            }
            
            if self.at_end(){
                Err(LoxResult::error(self.line, "String not ended"))
            }
        }
        Ok(())
    }

}
