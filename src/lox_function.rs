use crate::environment::{Closure, Environment};
use crate::interpreter::{Interpreter, Object, RTResult, RuntimeException};
use crate::stmt::Function;

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> RTResult;
    fn arity(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: Function,
    closure: Closure,
}

impl LoxFunction {
    pub fn new(declaration: Function, closure: Closure) -> LoxFunction {
        LoxFunction {
            declaration,
            closure,
        }
    }
}

impl Callable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> RTResult {
        let environment = Environment::from_env(self.closure.clone());
        for (i, param) in self.declaration.params.iter().enumerate() {
            environment.borrow_mut().define(param.lexeme.clone(), arguments[i].clone());
        }
        // println!("func: {:?}\n", environment);
        match interpreter.execute_block(&self.declaration.body, environment) {
            Ok(obj) => Ok(obj),
            Err(exception) => match exception {
                RuntimeException::RETURN(rv) => Ok(rv.value),
                _ => Err(exception),
            },
        }
        // Ok(Object::NIL(None))
    }
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}
