use lox_syntax::token::{Object, Token};
use result::result::LoxResult;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn get(&mut self, name: &Token) -> Result<Object, LoxResult> {
        let name_str = name.lexeme.as_str();
        if let Entry::Occupied(object) = self.values.entry(name.lexeme.clone()) {
            Ok(object.get().clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().get(name)
        } else {
            Err(LoxResult::run_time_error(
                name.line,
                &format!("Undefined variable '{name_str}'."),
            ))
        }
    }

    pub fn define(&mut self, name: &str, value: Object) {
        self.values.insert(name.to_owned(), value);
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), LoxResult> {
        if let Entry::Occupied(mut object) = self.values.entry(name.lexeme.clone()) {
            object.insert(value);
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(LoxResult::run_time_error(
                name.line,
                &format!("Undefined variable {}.", name.lexeme),
            ))
        }
    }

    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }
}

