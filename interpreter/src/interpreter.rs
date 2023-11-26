use ast::expr::{Expr};
use ast::expr;
use ast::stmt;
use ast::stmt::{Stmt};
use lox_syntax::token::{Object, Token, TokenType};
use result::result::LoxResult;
use crate::environment::Environment;


pub struct Interpreter {
   environment: Environment
}

impl Interpreter {
    pub fn evaluate(&mut self, expression: &Box<Expr>) -> Result<Object, LoxResult> {
        expression.accept(self)
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<(), LoxResult> {
        for statement in statements.iter() {
           self.execute(statement)?
        }
        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), LoxResult> {
       statement.accept(self)
    }

}

impl expr::Visitor<Result<Object,LoxResult>> for Interpreter {
    fn visit_binary_expr(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Result<Object, LoxResult> {
        let left_object = self.evaluate(left).unwrap();
        let right_object = self.evaluate(right).unwrap();

        match operator.ttype {
            TokenType::Minus => Object::subtract(left_object, right_object, operator.line),
            TokenType::Slash => Object::divide(left_object, right_object, operator.line),
            TokenType::Star  => Object::multiply(left_object, right_object, operator.line),
            TokenType::Plus  => Object::add(left_object, right_object, operator.line),
            TokenType::Greater => Object::greater(left_object, right_object, operator.line),
            TokenType::GreaterEqual => Object::greater_equal(left_object, right_object, operator.line),
            TokenType::Less => Object::less(left_object, right_object, operator.line),
            TokenType::LessEqual => Object::less_equal(left_object, right_object, operator.line),
            TokenType::BangEqual => Object::bang_equal(left_object, right_object, operator.line),
            TokenType::Equals => Object::equals(left_object, right_object, operator.line),
            _ => Err(LoxResult::run_time_error(operator.line, "Incorrect operator for binary expression"))
        }
    }

    fn visit_grouping_expr(&mut self, expression: &Box<Expr>) -> Result<Object, LoxResult> {
       self.evaluate(expression)
    }

    fn visit_literal_expr(&mut self, value: &Object) -> Result<Object, LoxResult> {
        Ok(value.clone())
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Box<Expr>) -> Result<Object, LoxResult> {
        let right_object = self.evaluate(right).unwrap();
        match operator.ttype {
            TokenType::Minus => right_object.negate(operator.line),
            TokenType::Bang  => Ok(right_object.is_truthy()),
            _ => Err(LoxResult::run_time_error(operator.line, "Incorrect operator for unary expression"))
        }
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Box<Expr>) -> Result<Object, LoxResult> {
        todo!()
    }

    fn visit_call_expr(&mut self, callee: &Box<Expr>, paren: &Token, arguments: &Vec<Box<Expr>>) -> Result<Object, LoxResult> {
        todo!()
    }

    fn visit_get_expr(&mut self, object: &Box<Expr>, name: &Token) -> Result<Object, LoxResult> {
        todo!()
    }

    fn visit_logical_expr(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Result<Object, LoxResult> {
        todo!()
    }

    fn visit_set_expr(&mut self, object: &Box<Expr>, name: &Token, value: &Box<Expr>) -> Result<Object, LoxResult> {
        todo!()
    }

    fn visit_super_expr(&mut self, keyword: &Token, method: &Token) -> Result<Object, LoxResult> {
        todo!()
    }

    fn visit_this_expr(&mut self, keyword: &Token) -> Result<Object, LoxResult> {
        todo!()
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<Object, LoxResult> {
        self.environment.get(name)
    }
}

impl stmt::Visitor<Result<(), LoxResult>> for Interpreter{
    fn visit_block_stmt(&mut self) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_class_stmt(&mut self) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_expression_stmt(&mut self, expression: &Box<Expr>) -> Result<(), LoxResult> {
         self.evaluate(expression)?;
         Ok(())
    }

    fn visit_function_stmt(&mut self) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_if_stmt(&mut self) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_print_stmt(&mut self, value: &Box<Expr>) -> Result<(), LoxResult> {
        let value = self.evaluate(value)?;
        println!("{}", value.stringify());
        Ok(())

    }

    fn visit_return_stmt(&mut self) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Box<Expr>>) -> Result<(), LoxResult> {
        let mut value = Object::Nil;
        if let Some(init_value) = initializer {
           value = self.evaluate(init_value)?;
        }
        self.environment.define(name.lexeme.to_string(), value);
        Ok(())
    }

    fn visit_while_stmt(&mut self) -> Result<(), LoxResult> {
        todo!()
    }
}