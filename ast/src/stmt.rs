use crate::expr::Expr;
use lox_syntax::token::Token;

pub enum Stmt {
    Block(Vec<Stmt>),
    Class,
    Expression(Box<Expr>),
    Function,
    If,
    Print(Box<Expr>),
    Return,
    Variable(Token, Option<Box<Expr>>),
    While,
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Block(statements) => visitor.visit_block_stmt(statements),
            Stmt::Class => visitor.visit_class_stmt(),
            Stmt::Expression(expression) => visitor.visit_expression_stmt(expression),
            Stmt::Function => visitor.visit_function_stmt(),
            Stmt::If => visitor.visit_if_stmt(),
            Stmt::Print(value) => visitor.visit_print_stmt(value),
            Stmt::Return => visitor.visit_return_stmt(),
            Stmt::Variable(name, initializer) => visitor.visit_var_stmt(name, initializer),
            Stmt::While => visitor.visit_while_stmt(),
        }
    }
}

pub trait Visitor<T> {
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> T;
    fn visit_class_stmt(&mut self) -> T;
    fn visit_expression_stmt(&mut self, expression: &Expr) -> T;
    fn visit_function_stmt(&mut self) -> T;
    fn visit_if_stmt(&mut self) -> T;
    fn visit_print_stmt(&mut self, value: &Expr) -> T;
    fn visit_return_stmt(&mut self) -> T;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Box<Expr>>) -> T;
    fn visit_while_stmt(&mut self) -> T;
}

