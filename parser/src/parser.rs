use lox_syntax::token::{Token, TokenType};
use ast::expr::Expr;
use lox_syntax::token::TokenType::{BangEqual, Equals};
use std::rc::Rc;

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

    fn expression(&mut self) -> Expr{
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.matches(&[BangEqual, Equals]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Rc::clone(expr), operator, Rc::clone(right));
        }
        expr
    }

    fn comparison(&self) -> Expr {
        todo!()
    }

    fn matches(&mut self, types: &[TokenType]) -> bool{
        for ttype in &types {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> Token {
        if !self.at_end(){
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        self.tokens[self.current]
    }

    fn check(&mut self, ttype: TokenType) -> bool {
       if !self.at_end() {
           self.peek().ttype == ttype
       }
       false
    }

    fn at_end(&self) -> bool {
        self.peek().ttype == TokenType::Eof
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }

}
