use crate::lox_function::LoxFunction;
use crate::lox_class::LoxClass;
use crate::lox_instance::LoxInstance;

#[derive(Debug, Clone)]
pub enum Object {
    STRING(String),
    NUMBER(f64),
    BOOL(bool),
    NIL(Option<()>),
    Function(LoxFunction),
    Class(LoxClass),
    Instance(LoxInstance),
}

impl Object {
    pub fn to_bool(&self) -> Result<bool, ()> {
        match self {
            Object::BOOL(b) => Ok(*b),
            Object::NIL(_) => Ok(false),
            _ => Err(()),
        }
    }
}
