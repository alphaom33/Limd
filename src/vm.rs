use crate::obj::Obj;
use crate::stdlib;
use std::fmt;
use std::fmt::Error;
use std::vec;
use std::slice;
use std::ops::{Add, Sub, Mul, Div};
use std::collections::{LinkedList, HashMap};

use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use enum_index::IndexEnum;

pub struct VM {
  pub stack: vec::Vec<Value>,
  pub globals: HashMap<String, Value>,
}

#[derive(PartialEq)]
pub enum InterpretResult {
  Ok(Option<Value>),
  CompileError,
  RuntimeError(String),
}

impl VM {
  pub fn new() -> VM {
    let globals = stdlib::get();
    VM {
      stack: vec::Vec::new(),
      globals,
    }
  }
  
  pub fn interpret(&mut self, chunk: Box<Chunk>) -> InterpretResult {
    self.stack.clear();
    return self.run(chunk);
  }

  fn read_byte(ip: &mut slice::Iter<u8>) -> u8 {
    return *ip.next().unwrap();
  }
  
  fn read_constant(mut ip: &mut slice::Iter<u8>, chunk: Chunk) -> Value {
    let a = Self::read_byte(&mut ip) as usize;
    return chunk.constants[a].clone();
  }

  fn get_var(&mut self, name: &str) -> InterpretResult {
    match self.globals.get(name) {
      Some(v) => return InterpretResult::Ok(Some(v.clone())),
      None => return InterpretResult::RuntimeError(std::format!("{name} is not defined.")),
    }
  }

  fn call_val(&mut self) -> InterpretResult {
    let Some(Value::Vector(mut args)) = self.stack.pop() else {
      return InterpretResult::RuntimeError("expected list??".to_owned());
    };
    
    let Some(Value::Symbol(name)) = args.pop() else {
      return InterpretResult::RuntimeError("Expected Symbol".to_owned());
    };
    
    let var_result = self.get_var(&name);
    let InterpretResult::Ok(Some(to_call)) = var_result else {
      return var_result;
    };

    match to_call {
      Value::Object(o) => match *o {
        Obj::Function(f) => {
        }
        
        Obj::Native(f) => {
          if !f.varargs && args.len() != f.arity as usize {
            return InterpretResult::RuntimeError(format!("Exected {} args, but {} where given.", f.arity, args.len()))
          }
          args.reverse();
          let out = (f.function)(&mut self.globals, &mut args);
          self.stack.push(out);
        }
      },
      _ => return InterpretResult::RuntimeError("Only functions can be called.".to_owned()),
    };

    return InterpretResult::Ok(None);
  }

  fn run(&mut self, chunk: Box<Chunk>) -> InterpretResult {
    let mut ip = chunk.code.iter();
    while let Some(byte) = ip.next() {

      print!("          ");
      for slot in &self.stack {
        print!("[ {} ]", slot);
      }
      println!();
      
      let op_code = OpCode::index_enum(*byte as usize).expect("Invalid opcode");
      match op_code {
        OpCode::Constant => {
          let constant = Self::read_constant(&mut ip, *chunk.clone());
          self.stack.push(constant);
        },

        OpCode::Call => {
          let result = self.call_val();
          let InterpretResult::Ok(_) = result else {
            return result;
          };
        }
        
        OpCode::Vector => {
          let len = Self::read_byte(&mut ip);
          let mut vals: Vec<Value> = Vec::new();
          for _ in 0..len {
            vals.push(self.stack.pop().expect("Vector length didn't match number of constants."));
          }
          self.stack.push(Value::Vector(vals));
        },

        OpCode::List => {
          let len = Self::read_byte(&mut ip);
            let mut vals: LinkedList<Value> = LinkedList::new();
            for _ in 0..len {
              vals.push_front(self.stack.pop().expect(std::format!("List length was shorter than the expected {len}").as_str()));
            }
            self.stack.push(Value::List(vals));
        },

        OpCode::Return => return InterpretResult::Ok(self.stack.pop()),
      }
    }
    return InterpretResult::Ok(self.stack.pop());
  }
}