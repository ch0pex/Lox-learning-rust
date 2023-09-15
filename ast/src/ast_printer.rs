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
   
    fn visit_assing_expr(&mut self, name: &Token, value: &Expr) -> String {
        todo!()
    }

    fn visit_call_expr(&mut self, callee: &Box<Expr>, paren: &Token, arguments: &Vec<Box<Expr>>) -> String { 
        todo!()
    } 

    fn visit_get_expr(&mut self, object: &Box<Expr>, name: &Token) -> String {
        todo!()
    }

    fn visit_logical_expr(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> String {
        todo!()
    }

    fn visit_set_expr(&mut self, object: &Box<Expr>, name: &Token, value: &Box<Expr>) -> String {
        todo!()
    }

    fn visit_super_expr(&mut self, keyword: &Token, method: &Token) -> String { 
        todo!()
    }

    fn visit_this_expr(&mut self, keyword: &Token) -> String {
        todo!()
    } 

    fn visit_variable_expr(&mut self, keyword: &Token) -> String {
        todo!()
    } 
}
