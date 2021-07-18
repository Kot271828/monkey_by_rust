use crate::object::*;
use std::collections::HashMap;

pub struct Enviroment {
    store: HashMap<String, Object>,
}

impl Enviroment {
    pub fn new() -> Self {
        Enviroment {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, object: Object) {
        self.store.insert(name, object);
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        self.store.get(name).map(|o| (*o).clone().to_owned())
    }
}
