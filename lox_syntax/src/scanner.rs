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
            '0'..='9' => {self.number()?}
            'a'..='z' | 'A'..='Z' | '_' => {self.identifier()?}
            
            '\n' => {self.line += 1}
            '\r' => {},
            '\t' => {},
            ' ' => {},
            '/' => {
                match self.expect('/'){ 
                    true => {
                        while self.peek().unwrap() != '\n' && !self.at_end() {self.advance();}
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

    pub fn advance(&mut self) -> &char { 
        self.current += 1;
        self.source.get(self.current - 1).unwrap()
    }

    fn at_end(&self) -> bool{
        self.current >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> { 
        self.source.get(self.current + 1).copied()
    }

    fn is_digit(ch: Option<char>) -> bool { 
        if let Some(ch) = ch { 
            ch.is_ascii_digit()
        } else {
            false
        }
    }

    fn is_alphanumeric(ch: Option<char>) -> bool { 
        if let Some(ch) = ch {
            ch.is_alphanumeric()
        } else {
            false
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
        while let Some(ch) = self.peek(){ 
            match ch {
                '\n' => {self.line += 1},
                '"' => break,
                 _ => {self.advance();}
            }
        }
        if self.at_end(){
            return Err(LoxResult::error(self.line, "String not ended"));
        }
        self.advance();   
        let value = self.source[self.start + 1..self.current - 1].iter().collect();
        self.add_token_object(TokenType::String, Some(Object::Str(value)));
        Ok(())
    }


    fn number(&mut self) -> Result<(), LoxResult>{ 
        while Scanner::is_digit(self.peek()){ 
            self.advance();
        } 
        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            self.advance();
            while Scanner::is_digit(self.peek()){
                self.advance();
                if self.peek() == Some('.') {  
                    return Err(LoxResult::error(self.line, "Number with multiple dots"));
                }
            }
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        let number: f64 = value.parse().unwrap();
        self.add_token_object(TokenType::Number, Some(Object::Num(number)));
        Ok(())
    }

    fn identifier(&mut self) -> Result<(), LoxResult>{ 
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }
        let word: String = self.source[self.start..self.current].iter().collect();
        let token_type = Scanner::check_keyword(word.as_str());
        self.add_token(token_type);
        Ok(())
    }

    fn check_keyword(word: &str) -> TokenType {
        match word {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier 
        }
    }


}
