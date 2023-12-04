use lox_syntax::token::{Object, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Object),
    Unary(Token, Box<Expr>),
    Assign(Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
    Get(Box<Expr>, Token),
    Logical(Box<Expr>, Token, Box<Expr>),
    Set(Box<Expr>, Token, Box<Expr>),
    Super(Token, Token),
    This(Token),
    Variable(Token),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary(left, operator, right) => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping(expression) => visitor.visit_grouping_expr(expression),
            Expr::Literal(value) => visitor.visit_literal_expr(value),
            Expr::Unary(operator, right) => visitor.visit_unary_expr(operator, right),
            Expr::Assign(name, value) => visitor.visit_assign_expr(name, value),
            Expr::Call(callee, paren, arguments) => {
                visitor.visit_call_expr(callee, paren, arguments)
            }
            Expr::Get(object, name) => visitor.visit_get_expr(object, name),
            Expr::Logical(left, operator, right) => {
                visitor.visit_logical_expr(left, operator, right)
            }
            Expr::Set(object, name, value) => visitor.visit_set_expr(object, name, value),
            Expr::Super(keyword, method) => visitor.visit_super_expr(keyword, method),
            Expr::This(keyword) => visitor.visit_this_expr(keyword),
            Expr::Variable(name) => visitor.visit_variable_expr(name),
        }
    }
}

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
    fn visit_literal_expr(&mut self, value: &Object) -> T;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> T;
    fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &[Expr]) -> T;
    fn visit_get_expr(&mut self, object: &Expr, name: &Token) -> T;
    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_set_expr(&mut self, object: &Expr, name: &Token, value: &Expr) -> T;
    fn visit_super_expr(&mut self, keyword: &Token, method: &Token) -> T;
    fn visit_this_expr(&mut self, keyword: &Token) -> T;
    fn visit_variable_expr(&mut self, name: &Token) -> T;
}

