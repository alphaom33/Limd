use std::fmt::Display;
use crate::obj::Obj;

pub enum Value {
  Nil,
  Boolean(bool),
  Number(f64),
  Object(Obj),
}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Nil => write!(f, "nil"),
      Value::Boolean(b) => write!(f, "{}", b),
      Value::Number(n) => write!(f, "{}", n),
      Value::Object(o) => write!(f, "{}", o),
    }
  }
}