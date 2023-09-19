use std::arch::x86_64::__m128;
use lox_syntax::token::{Object, Token, TokenType};
use ast::expr::Expr;
use lox_syntax::token::TokenType::{Bang, BangEqual, Equals, False, Greater, GreaterEqual, Less, LessEqual, Minus, Nil, Number, Plus, RightParen, String, True};
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
            expr = Expr::Binary(Rc::from(expr), operator, Rc::from(right));
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.matches(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Rc::from(expr), operator, Rc::from(right));
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.matches(&[Minus, Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Rc::from(expr), operator, Rc::from(right));
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.matches(&[Minus, Plus]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Rc::from(expr), operator, Rc::from(right));
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.matches(&[Bang, Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(operator, Rc::from(right));
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr{
        if self.matches(&[False]) {return Expr::Literal(Object::False);}
        if self.matches(&[True]) {return Expr::Literal(Object::True);}
        if self.matches(&[Nil]) {return Expr::Literal(Object::Nil);}
        if self.matches(&[Number,String]) {return Expr::Literal(self.previous().literal.unwrap());}

        let expr = self.expression();
        self.consume(RightParen, "Expect ')' after expression");
        Expr::Grouping(Rc::from(expr))

    }

    fn consume(&self, token_type: TokenType, message: &str)  {
        todo!()
    }

    fn matches(&mut self, types: &[TokenType]) -> bool{
        for token_type in &types {
            if self.check(token_type) {
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
