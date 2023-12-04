use ast::expr::Expr;
use ast::stmt::Stmt;
use lox_syntax::token::TokenType::*;
use lox_syntax::token::{Object, Token, TokenType};
use result::result::LoxResult;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxResult> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(lox_result) => {
                    self.synchronize();
                    return Err(lox_result);
                }
            }
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, LoxResult> {
        if self.matches(&[Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxResult> {
        let name = self.consume(Identifier, "Expect variable name.")?.clone();
        let mut initializer: Option<Box<Expr>> = None;

        if self.matches(&[Assign]) {
            initializer = Some(self.expression()?);
        }
        self.consume(Semicolon, "Expected ';' after variable declaration.")?;
        Ok(Stmt::Variable(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, LoxResult> {
        if self.matches(&[Print]) {
            return self.print_statement();
        }
        if self.matches(&[LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxResult> {
        let value = self.expression()?;
        self.consume(Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxResult> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Box<Expr>, LoxResult> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Box<Expr>, LoxResult> {
        let expr = self.equality()?;
        if self.matches(&[Assign]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;
            match *expr {
                Expr::Variable(name) => Ok(Box::new(Expr::Assign(name, value))),
                _ => Err(LoxResult::parse_error(
                    equals.line,
                    "Invalid assignment target.",
                    "=",
                )),
            }
        } else {
            Ok(expr)
        }
    }

    fn equality(&mut self) -> Result<Box<Expr>, LoxResult> {
        let mut expr = self.comparison()?;
        while self.matches(&[BangEqual, Equals]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>, LoxResult> {
        let mut expr = self.term()?;
        while self.matches(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, LoxResult> {
        let mut expr = self.factor()?;
        while self.matches(&[Minus, Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>, LoxResult> {
        let mut expr = self.unary()?;
        while self.matches(&[Slash, Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, LoxResult> {
        if self.matches(&[Bang, Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary(operator, right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expr>, LoxResult> {
        if self.matches(&[False]) {
            return Ok(Box::new(Expr::Literal(Object::False)));
        }
        if self.matches(&[True]) {
            return Ok(Box::new(Expr::Literal(Object::True)));
        }
        if self.matches(&[Nil]) {
            return Ok(Box::new(Expr::Literal(Object::Nil)));
        }
        if self.matches(&[Number, String]) {
            return Ok(Box::new(Expr::Literal(
                self.previous()
                    .clone()
                    .literal
                    .expect("Non previous literal"),
            )));
        }

        if self.matches(&[Identifier]) {
            return Ok(Box::new(Expr::Variable(self.previous().clone())));
        }

        if self.matches(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(RightParen, "Expect ')' after expression")?;
            return Ok(Box::new(Expr::Grouping(expr)));
        }

        Err(self.error(self.peek(), "Expect expression"))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxResult> {
        let mut statements = vec![];
        while !self.check(RightBrace) && !self.at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, LoxResult> {
        match self.check(token_type) {
            true => Ok(self.advance()),
            false => Err(self.error(self.peek(), message)),
        }
    }

    fn error(&self, token: &Token, message: &str) -> LoxResult {
        LoxResult::parse_error(token.line, message, &token.lexeme)
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        for &token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> &Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn check(&mut self, ttype: TokenType) -> bool {
        if !self.at_end() {
            return self.peek().ttype == ttype;
        }
        false
    }

    fn at_end(&self) -> bool {
        self.peek().ttype == Eof
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.at_end() {
            if self.previous().ttype == Semicolon {
                return;
            }
            if matches!(
                self.peek().ttype,
                Class | Fun | For | If | While | Print | Return
            ) {
                return;
            }
            self.advance();
        }
    }
}

