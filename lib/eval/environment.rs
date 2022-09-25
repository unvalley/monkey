use std::collections::HashMap;

use crate::eval::object::Object;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<Object> {
        self.store.get(key).cloned()
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
