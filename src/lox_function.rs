use crate::environment::Environment;
use crate::interpreter::{Interpreter, RTResult, RuntimeException};
use crate::object::Object;
use crate::stmt::Function;
use gc_derive::{Finalize, Trace};

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> RTResult;
    fn arity(&self) -> usize;
}

// #[derive(Debug, Clone)]
// enum FuncClosure {
//     STRONG(Closure),
//     WEAK(WeakClosure),
// }

// impl FuncClosure {
//     pub fn to_strong(&self) -> Closure {
//         match self {
//             FuncClosure::STRONG(e) => e.clone(),
//             FuncClosure::WEAK(e) => e.upgrade().unwrap(),
//         }
//     }
// }

#[derive(Trace, Finalize, Debug, Clone)]
pub struct LoxFunction {
    #[unsafe_ignore_trace]
    declaration: Function, // 该项不会含有gc管理的对象
    closure: Environment,
}

// impl Clone for LoxFunction {
//     fn clone(&self) -> Self {
//         let closure = self.closure.to_strong();
//         LoxFunction {
//             declaration: self.declaration.clone(),
//             closure: FuncClosure::STRONG(closure)
//         }
//     }
// }

impl LoxFunction {
    pub fn new(declaration: Function, env: Environment) -> LoxFunction {
        LoxFunction {
            declaration,
            closure: env,
        }
    }
}

// impl Drop for LoxFunction {
//     fn drop(&mut self) {
//         println!("drop func");
//     }
// }

impl Callable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> RTResult {
        let mut environment = Environment::from_env(self.closure.clone());
        for (i, param) in self.declaration.params.iter().enumerate() {
            environment.define(param.lexeme.clone(), arguments[i].clone());
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
