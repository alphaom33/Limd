use crate::vm::VM;
use crate::value::Value;
use crate::chunk::Chunk;

use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum Obj {
  Native(Native),
  Function(Function),
}

impl Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let _ = write!(f, "{}", match self {
        Self::Native(_) => "<native>",
        Self::Function(_) => "<fn>",
      });
      return Result::Ok(());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Native {
  pub arity: u8,
  pub function: fn(&mut HashMap<String, Value>, &mut [Value]) -> Value,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
  pub arity: u8,
  pub body: Chunk,
}

impl Function {
  fn call(&self, _args: &[Value]) -> Option<Value> {
    return Option::Some(Value::Nil);
  }
  
  fn arity(&self) -> u8 {
    return self.arity;
  }
}