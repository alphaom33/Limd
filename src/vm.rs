use std::vec;
use std::slice;
use std::ops::{Add, Sub, Mul, Div};

use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use crate::obj;
use enum_index::IndexEnum;

pub struct VM<'a> {
  pub chunk: &'a Chunk,
  pub ip: slice::Iter<'a, u8>,
  pub stack: vec::Vec<Value>,
}

pub enum InterpretResult {
  Ok,
  CompileError,
  RuntimeError,
}

impl <'a> VM<'a> {
  pub fn new(chunk: &'a Chunk) -> VM<'a> {
    VM {
      chunk,
      ip: chunk.code.iter(),
      stack: vec::Vec::new(),
    }
  }
  
  pub fn interpret(&mut self) -> InterpretResult {
    return self.run();
  }

  fn read_byte(&mut self) -> u8 {
    return *self.ip.next().unwrap();
  }
  
  fn read_constant(&mut self) -> Value {
    let a = self.read_byte() as usize;
    return self.chunk.constants[a];
  }

  fn push(&mut self, value: Value) {
    self.stack.push(value);
  }

  fn pop(&mut self) -> Value {
    return self.stack.pop().unwrap();
  }

  fn run(&mut self) -> InterpretResult {
    while let Some(byte) = self.ip.next() {
      let op_code = OpCode::index_enum(*byte as usize).expect("Invalid opcode");
      match op_code {
        
        OpCode::OpConstant => {
          let constant = self.read_constant();
          self.push(constant);
        },

        OpCode::OpFunction => {
          let function = match self.pop() {
            Value::Object(o) => match o {
              obj::Obj::Function(f) => f,
              _ => return InterpretResult::RuntimeError,
            },
            _ => return InterpretResult::RuntimeError,
          };
          let args: Vec<Value> = (0 .. function.arity).into_iter().map(|_| self.pop()).collect();
          function.call(&args);
        }
        
        OpCode::OpReturn => return InterpretResult::Ok,
      }
    }
    return InterpretResult::Ok;
  }
}