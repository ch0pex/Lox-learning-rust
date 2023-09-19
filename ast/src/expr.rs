use std::rc::Box;
use lox_syntax::token::{Token, Object};

pub enum Expr<'a> {
    Binary(Box<Expr<'a>>, &'a Token, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
    Literal(Object),
    Unary(&'a Token, Box<Expr<'a>>),
    Assign(&'a Token, Box<Expr<'a>>),
    Call(Box<Expr<'a>>, &'a Token, Vec<Box<Expr<'a>>>),
    Get(Box<Expr<'a>>, &'a Token),
    Logical(Box<Expr<'a>>, &'a Token, Box<Expr<'a>>),
    Set(Box<Expr<'a>>, &'a Token, Box<Expr<'a>>),
    Super(&'a Token, &'a Token),
    This(&'a Token),
    Variable(&'a Token),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T{
        match self {
            Expr::Binary(left, operator, right) => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping(expression) => visitor.visit_grouping_expr(expression),
            Expr::Literal(value) => visitor.visit_literal_expr(value),
            Expr::Unary(operator, right) => visitor.visit_unary_expr(operator, right),
            Expr::Assign(name, value) => visitor.visit_assign_expr(name, value),
            Expr::Call(callee, paren, arguments) => visitor.visit_call_expr(callee, paren, arguments),
            Expr::Get(object, name) => visitor.visit_get_expr(object, name),
            Expr::Logical(left, operator, right) => visitor.visit_logical_expr(left, operator, right),
            Expr::Set(object, name, value) => visitor.visit_set_expr(object, name, value),
            Expr::Super(keyword, method) => visitor.visit_super_expr(keyword, method),
            Expr::This(keyword) => visitor.visit_this_expr(keyword),
            Expr::Variable(name) => visitor.visit_variable_expr(name)
        }
    }
}

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
    fn visit_literal_expr(&mut self, value: &Object) -> T;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> T;
    fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &Vec<Expr>) -> T;
    fn visit_get_expr(&mut self, object: &Box<Expr>, name: &Token) -> T;
    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_set_expr(&mut self, object: &Expr, name: &Token, value: &Expr) -> T;
    fn visit_super_expr(&mut self, keyword: &Token, method: &Token) -> T;
    fn visit_this_expr(&mut self, keyword: &Token) -> T;
    fn visit_variable_expr(&mut self, name: &Token) -> T;
}

/*

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> T;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> T;
    fn visit_literal_expr(&mut self, expr: &Literal) -> T;
    fn visit_unary_expr(&mut self, expr: &Unary) -> T;
}

trait BaseExpr {
    fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T;
}

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary)
}

pub struct Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>
}

impl Binary{
    fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Binary {
        Binary {left,  operator,  right }
    }
}

impl BaseExpr for Binary {
    fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        visitor.visit_binary_expr(self)
    }
}

pub struct Grouping {
    expression: Box<Expr>
}

impl Grouping{
    fn new(expression: Box<Expr>) -> Grouping {
        Grouping {expression}
    }
}

impl BaseExpr for Grouping {
    fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        visitor.visit_grouping_expr(self)
    }
}

pub struct Literal {
    value: Token
}

impl Literal{
    fn new(value: Token) -> Literal {
        Literal {value}
    }
}

impl BaseExpr for Literal {
    fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        visitor.visit_literal_expr(self)
    }
}

pub struct Unary {
    operator: Token,
    right: Box<Expr>
}

impl Unary {
    fn new(operator: Token, right: Box<Expr>) -> Unary {
        Unary{operator, right}
    }
}

impl BaseExpr for Unary {
    fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        visitor.visit_unary_expr(self)
    }
}

*/

