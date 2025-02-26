use std::fmt::Display;
use crate::obj::Obj;

#[derive(Clone, Debug)]
pub enum Value {
  Nil,
  Boolean(bool),
  Number(f64),
  String(String),
  Label(String),
  Object(Box<Obj>),
}

impl Value {
  pub fn is_truthy(&self) -> bool {
    return match self {
      Self::Nil => false,
      Self::Boolean(b) => *b,
      Self::Number(n) => *n != 0.0,
      Self::String(s) => !s.is_empty(),
      Self::Label(_) => true,
      Self::Object(_) => true,
    };
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Nil => write!(f, "nil"),
      Value::Boolean(b) => write!(f, "{}", b),
      Value::Number(n) => write!(f, "{}", n),
      Value::String(s) => write!(f, "{}", s),
      Value::Label(s) => write!(f, "{}", s),
      Value::Object(o) => write!(f, "{}", o),
    }
  }
}