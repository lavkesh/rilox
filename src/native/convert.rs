use std::fmt::{Debug, Formatter};
use crate::error::Error;
use crate::interpreter::function::LoxCallable;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub struct ToNumberFn;

impl Debug for ToNumberFn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "to_number fn")
    }
}

impl LoxCallable for ToNumberFn {
    fn arity(&self) -> usize {
        1 // takes one argument
    }

    fn call(&self, _interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, Error> {
        let arg = &args[0];
        match arg {
            Value::Number(n) => Ok(Value::Number(*n)),
            Value::String(s) => {
                match s.trim().parse::<f64>() {
                    Ok(n) => Ok(Value::Number(n)),
                    Err(_) => Ok(Value::Nil),
                }
            }
            Value::Boolean(b) => {
                if *b {
                    Ok(Value::Number(1.0))
                } else {
                    Ok(Value::Number(0.0))
                }
            }
            _ => Ok(Value::Nil),
        }
    }
}
