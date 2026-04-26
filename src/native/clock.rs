use std::fmt::{Debug, Formatter};
use crate::error::Error;
use crate::interpreter::function::LoxCallable;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;

pub struct ClockFn;

impl Debug for ClockFn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn clock")
    }
}

impl LoxCallable for ClockFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &mut Interpreter, _args: Vec<Value>) -> Result<Value, Error> {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        Ok(Value::Number(time))
    }
}