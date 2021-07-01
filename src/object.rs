pub enum Object {
    Integer { value: i32 },
    Boolean { value: bool },
    ReturnValue { value: Box<Object> },
    Null,
}

impl Object {
    pub fn literal(&self) -> String {
        match self {
            Object::Integer {value} => value.to_string(),
            Object::Boolean {value} => {
                if *value {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Object::ReturnValue {value} => value.literal(),
            Object::Null => "null".to_string(),    
        }
    }
}
