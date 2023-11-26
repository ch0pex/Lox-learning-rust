use std::collections::HashMap;
use lox_syntax::token::{Object, Token};
use result::result::LoxResult;

pub struct Environment {
   values: HashMap<String, Object>
}

impl Environment {
    pub fn get(&self, name: &Token) -> Result<Object, LoxResult> {
        match self.values.get(name.lexeme.as_str()) {
            Some(value) => Ok(value.clone()),
            None => Err(LoxResult::run_time_error(name.line, "Undefined variable"))
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
       self.values.insert(name, value);
    }
}