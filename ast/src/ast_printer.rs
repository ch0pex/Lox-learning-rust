use crate::ast::*;
use lox_syntax::token::{Object,Token};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> AstPrinter{
        AstPrinter{}
    }

    pub fn print(&mut self, expr: Expr) -> String {
        expr.accept(self)
    }

    pub fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut s = format!("({name}");
        for expr in exprs {
            s = format!("{s} {}",expr.accept(self));
        }
        s.push_str(")");
        s
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme.as_str(), &[left, right])
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> String {
        self.parenthesize("group", &[expression])
    }

    fn visit_literal_expr(&mut self, value: &Object) -> String {
        match value { 
            Object::Nil => "nil".to_string(),
            _ => value.to_string()
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme.as_str(), &[right])
    }
}
