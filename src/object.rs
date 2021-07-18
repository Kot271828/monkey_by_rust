use std::cell::RefCell;
use std::rc::Rc;

use crate::{ast::StatementNode, env::Enviroment};

#[derive(Clone)]
pub enum Object {
    Integer {
        value: i32,
    },
    Boolean {
        value: bool,
    },
    ReturnValue {
        value: Box<Object>,
    },
    FunctionObject {
        parameters: Vec<String>,
        body: Box<StatementNode>,
        env: Option<Rc<RefCell<Enviroment>>>,
    },
    Null,
}

impl Object {
    pub fn literal(&self) -> String {
        match &self {
            Object::Integer { value } => value.to_string(),
            Object::Boolean { value } => {
                if *value {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Object::ReturnValue { value } => value.literal(),
            &Object::FunctionObject {
                parameters,
                body,
                env: _,
            } => {
                let parameters = parameters.join(", ");
                format!("fn({}) {}", parameters, body.literal())
            }
            Object::Null => "null".to_string(),
        }
    }
}
