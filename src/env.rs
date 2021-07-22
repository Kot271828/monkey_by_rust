use std::cell::RefCell;
use std::rc::Rc;

use crate::object::*;
use std::collections::HashMap;

pub struct Enviroment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Enviroment>>>,
}

impl Enviroment {
    pub fn new() -> Self {
        Enviroment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn set(&mut self, name: &str, object: Object) {
        self.store.insert(name.to_string(), object);
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        let value = self.store.get(name).map(|o| (*o).clone().to_owned());
        if value.is_none() {
            if self.outer.is_none() {
                None
            } else {
                (self.outer).as_ref().unwrap().borrow().get(name)
            }
        } else {
            value
        }
    }

    pub fn add_outer(&mut self, env: &Rc<RefCell<Enviroment>>) {
        self.outer = Some(Rc::clone(env));
    }
}
