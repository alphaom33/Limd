use std::collections::LinkedList;
use std::fmt::Display;
use crate::obj::Obj;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
  Nil,
  Boolean(bool),
  Number(f64),
  String(String),
  Label(String),
  List(LinkedList<Value>),
  Vector(Vec<Value>),
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
      Self::List(l) => l.len() != 0,
      Self::Vector(v) => v.len() != 0,
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
      Value::List(l) => {
        if let Result::Err(e) =  write!(f, "( ") {return Result::Err(e);}
        for val in l {
          if let Result::Err(e) = write!(f, "{} ", val) {return Result::Err(e);}
        }
        return write!(f, ")");
      },
      Value::Vector(v) => {
        if let Result::Err(e) =  write!(f, "[ ") {return Result::Err(e);}
        for val in v {
          if let Result::Err(e) = write!(f, "{} ", val) {return Result::Err(e);}
        }
        return write!(f, "]");
      },
      Value::Object(o) => write!(f, "{}", o),
    }
  }
}

impl Default for Value {
    fn default() -> Self {
      return Value::Nil;
    }
}