use crate::error::Error;
use crate::interpreter::function::LoxCallable;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use std::fmt::{Debug, Formatter};
use std::io;
use std::io::{Write};

pub struct ReadLineFn;

impl Debug for ReadLineFn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "read_line fn")
    }
}

impl LoxCallable for ReadLineFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &mut Interpreter, _args: Vec<Value>) -> Result<Value, Error> {
        io::stdout().flush().unwrap_or(());
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                Ok(Value::Nil)
            }
            Ok(_) => {
                let trimmed = input.trim_end().to_string();
                Ok(Value::String(trimmed))
            }
            Err(_) => {
                Ok(Value::Nil)
            }
        }

    }
}
