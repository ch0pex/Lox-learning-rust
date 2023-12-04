use crate::environment::Environment;
use ast::expr;
use ast::expr::Expr;
use ast::stmt;
use ast::stmt::Stmt;
use lox_syntax::token::{Object, Token, TokenType};
use result::result::LoxResult;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn evaluate(&mut self, expression: &Expr) -> Result<Object, LoxResult> {
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

    fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Environment,
    ) -> Result<(), LoxResult> {
        let previous = self.environment.clone();
        self.environment = Rc::new(RefCell::new(environment));
        //std::mem::swap(&mut self.environment, &mut env);
        for stmt in statements {
            self.execute(stmt)?
        }
        self.environment = previous.clone();
        //std::mem::swap(&mut self.environment, &mut env);
        Ok(())
    }
}

impl expr::Visitor<Result<Object, LoxResult>> for Interpreter {
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, LoxResult> {
        let left_object = self.evaluate(left)?;
        let right_object = self.evaluate(right)?;

        match operator.ttype {
            TokenType::Minus => Object::subtract(left_object, right_object, operator.line),
            TokenType::Slash => Object::divide(left_object, right_object, operator.line),
            TokenType::Star => Object::multiply(left_object, right_object, operator.line),
            TokenType::Plus => Object::add(left_object, right_object, operator.line),
            TokenType::Greater => Object::greater(left_object, right_object, operator.line),
            TokenType::GreaterEqual => {
                Object::greater_equal(left_object, right_object, operator.line)
            }
            TokenType::Less => Object::less(left_object, right_object, operator.line),
            TokenType::LessEqual => Object::less_equal(left_object, right_object, operator.line),
            TokenType::BangEqual => Object::bang_equal(left_object, right_object, operator.line),
            TokenType::Equals => Object::equals(left_object, right_object, operator.line),
            _ => Err(LoxResult::run_time_error(
                operator.line,
                "Incorrect operator for binary expression",
            )),
        }
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<Object, LoxResult> {
        self.evaluate(expression)
    }

    fn visit_literal_expr(&mut self, value: &Object) -> Result<Object, LoxResult> {
        Ok(value.clone())
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Object, LoxResult> {
        let right_object = self.evaluate(right).unwrap();
        match operator.ttype {
            TokenType::Minus => right_object.negate(operator.line),
            TokenType::Bang => Ok(right_object.is_truthy()),
            _ => Err(LoxResult::run_time_error(
                operator.line,
                "Incorrect operator for unary expression",
            )),
        }
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Object, LoxResult> {
        let value = self.evaluate(value)?;
        self.environment.borrow_mut().assign(name, value.clone())?;
        Ok(value)
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &[Expr],
    ) -> Result<Object, LoxResult> {
        todo!()
    }

    fn visit_get_expr(&mut self, object: &Expr, name: &Token) -> Result<Object, LoxResult> {
        todo!()
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, LoxResult> {
        todo!()
    }

    fn visit_set_expr(
        &mut self,
        object: &Expr,
        name: &Token,
        value: &Expr,
    ) -> Result<Object, LoxResult> {
        todo!()
    }

    fn visit_super_expr(&mut self, keyword: &Token, method: &Token) -> Result<Object, LoxResult> {
        todo!()
    }

    fn visit_this_expr(&mut self, keyword: &Token) -> Result<Object, LoxResult> {
        todo!()
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<Object, LoxResult> {
        self.environment.borrow_mut().get(name)
    }
}

impl stmt::Visitor<Result<(), LoxResult>> for Interpreter {
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> Result<(), LoxResult> {
        self.execute_block(
            &statements,
            Environment::new_with_enclosing(self.environment.clone()),
        )
    }

    fn visit_class_stmt(&mut self) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<(), LoxResult> {
        self.evaluate(expression)?;
        Ok(())
    }

    fn visit_function_stmt(&mut self) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_if_stmt(&mut self) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_print_stmt(&mut self, value: &Expr) -> Result<(), LoxResult> {
        let value = self.evaluate(value)?;
        println!("{}", value.stringify());
        Ok(())
    }

    fn visit_return_stmt(&mut self) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_var_stmt(
        &mut self,
        name: &Token,
        initializer: &Option<Box<Expr>>,
    ) -> Result<(), LoxResult> {
        let mut value = Object::Nil;
        if let Some(init_value) = initializer {
            value = self.evaluate(init_value)?;
        }
        self.environment.borrow_mut().define(&name.lexeme, value);

        Ok(())
    }

    fn visit_while_stmt(&mut self) -> Result<(), LoxResult> {
        todo!()
    }
}

