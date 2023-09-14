use lox_syntax::token::Token;


pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
    Unary(Token, Box<Expr>),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T{
        match self {
            Expr::Binary(left, operator, right) => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping(expression) => visitor.visit_grouping_expr(expression),
            Expr::Literal(value) => visitor.visit_literal_expr(value),
            Expr::Unary(operator, right) => visitor.visit_unary_expr(operator, right),
        }
    }
}

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
    fn visit_literal_expr(&mut self, value: &Token) -> T;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;
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

