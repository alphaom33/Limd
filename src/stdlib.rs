use crate::vm::{InterpretResult, VM};
use crate::obj::{self, Obj};
use crate::value::Value;
use std::collections::HashMap;

fn function(name: &str, arity: u8, varargs: bool, is_macro: bool, function: fn(&mut VM, &mut [Value]) -> Value) -> (String, Value) {
  return (
    name.to_owned(),
    Value::Object(Box::new(obj::Obj::Native(obj::Native{
      arity,
      varargs,
      function,
      is_macro,
    }))));
}

macro_rules! binary_op {
  ($op:tt) => {
    function(stringify!($op), 0, true, false, |_vm, args| {
      if let Some((Value::Number(sum), rest)) = args.split_first() {
        let mut sum = *sum;
        for arg in rest {
          match arg {
            Value::Number(n) => sum = sum $op n,
            _ => return Value::Nil,
          }
        }
        return Value::Number(sum);
      }
      return Value::Nil;
     
  })};
}

macro_rules! binary_op2 {
  ($op:tt) => {
    function(stringify!($op), 0, true, false, |_vm, args| {
      if let Some((Value::Number(sum), rest)) = args.split_first() {
        let mut last = *sum;
        for arg in rest {
          match arg {
            Value::Number(n) => {
              if !(last $op *n) {
                return Value::Boolean(false);
              }
              last = *n;
            },
            _ => (),
          }
        }
      }
      return Value::Boolean(true);
     
  })};
}

pub fn get() -> HashMap<String, Value> {
  return HashMap::from([
    function("!def", 2, false, true, |vm, args| {
      if let Value::Symbol(s) = args[0].clone() {
        let result = vm.do_call(args[1].clone());
        vm.globals.insert(s,
           if let InterpretResult::Ok(Some(s)) = result {
            s
          } else {
            return Value::Nil;
          });
      } 
      return Value::Nil;
    }),
    function("print", 0, true, false, |_vm, args| {
      for arg in args {
        print!("{} ", arg);
      }
      return Value::Nil;
    }),
    binary_op!(+),
    binary_op!(-),
    binary_op!(/),
    binary_op!(*),

    binary_op2!(>),
    binary_op2!(>=),
    binary_op2!(<),
    binary_op2!(<=),
  ]);
}