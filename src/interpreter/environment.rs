use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{token::Token, token_type::Literal};

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: RefCell<HashMap<String, Literal>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            enclosing,
            values: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Literal, String> {
        if let Some(value) = self.values.borrow().get(&name.lexeme).cloned() {
            return Ok(value);
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(&name);
        }
        Err(format!("Undefined variable '{}'.", name.lexeme))
    }

    pub fn define(&self, name: String, value: Literal) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn assign(&self, name: &Token, value: Literal) -> Result<(), String> {
        if self.values.borrow().contains_key(&name.lexeme) {
            self.values.borrow_mut().insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }
        Err(format!("Undefined variable '{}'.", name.lexeme))
    }
}
