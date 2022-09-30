use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::eval::object::Object;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Environment {
    // innner expand outer scope
    store: HashMap<String, Object>,
    /// store references for other environment
    /// outer contains inner scope
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Self {
        Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<Object> {
        match self.store.get(key) {
            Some(obj) => Some(obj.clone()),
            None => match &self.outer {
                Some(env) => env.borrow_mut().get(key),
                None => None,
            },
        }
    }

    pub fn set(&mut self, key: String, val: Object) -> Option<Object> {
        self.store.insert(key, val)
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
