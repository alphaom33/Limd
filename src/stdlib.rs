use crate::vm::{InterpretResult, VM};
use crate::obj::{self, Obj};
use crate::value::Value;
use std::collections::HashMap;

fn function(name: &str, arity: u8, varargs: bool, is_macro: bool, function: fn(&mut VM, &mut [Value]) -> InterpretResult) -> (String, Value) {
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
            _ => return InterpretResult::Err(format!("Expected number, got {arg}")),
          }
        }
        return InterpretResult::Ok(Value::Number(sum));
      } else {
        return InterpretResult::Err(format!("Expected number, got {}", args[0]));
      }
     
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
                return InterpretResult::Ok(Value::Boolean(false));
              }
              last = *n;
            },
            _ => (),
          }
        }
      }
      return InterpretResult::Ok(Value::Boolean(true));
     
  })};
}

pub fn get() -> HashMap<String, Value> {
  return HashMap::from([
    function("!if", 3, false, true, |_vm, args| {
      if let Value::Boolean(b) = args[0].clone() {
        return InterpretResult::Ok(args[if b {1} else {2}].clone());
      }
      return InterpretResult::Err(format!("Expected bool, got {}", args[0]));
    }),
    function("!def", 2, false, true, |vm, args| {
      if let Value::Symbol(s) = args[0].clone() {
        let result = vm.do_call(args[1].clone());
        vm.globals.insert(s,
           if let InterpretResult::Ok(s) = result {
            s
          } else {
            return result;
          });
        return InterpretResult::Ok(Value::Nil);
      } else { 
        return InterpretResult::Err(format!("Expected symbol, got {}", args[0]));
      }
    }),
    function ("!or", 0, true, true, |vm, args| {
      for arg in args {
        let result = vm.do_call(arg.clone());
        if let InterpretResult::Ok(arg) = result {
          if arg.is_truthy() {
            return InterpretResult::Ok(Value::Boolean(true));
          }
        } else {
          return result;
        }
      }
      return InterpretResult::Ok(Value::Boolean(false));
    }),
    function ("!and", 0, true, true, |vm, args| {
      for arg in args {
        let result = vm.do_call(arg.clone());
        if let InterpretResult::Ok(arg) = result {
          if !arg.is_truthy() {
            return InterpretResult::Ok(Value::Boolean(false));
          }
        } else {
          return result;
        }
      }
      return InterpretResult::Ok(Value::Boolean(true));
    }),
    function("print", 0, true, false, |_vm, args| {
      for arg in args {
        print!("{} ", arg);
      }
      return InterpretResult::Ok(Value::Nil);
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