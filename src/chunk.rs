use std::vec;
use crate::value;

#[derive(EnumIndex, IndexEnum)]
pub enum OpCode {
  OpReturn,
  OpConstant,
  OpFunction,
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
      return self as u8;
    }
}

pub struct Chunk {
  pub code: vec::Vec<u8>,
  pub constants: vec::Vec<value::Value>,
  pub lines: vec::Vec<i32>,
}

impl Chunk {
  pub fn new() -> Chunk {
    Chunk {
      code: vec::Vec::new(),
      constants: vec::Vec::new(),
      lines: vec::Vec::new(),
    }
  }

  pub fn write<T>(&mut self, byte: T, line: i32) where T: Into<u8> {
    self.lines.push(line);
    self.code.push(byte.into());
  }

  pub fn add_constant(&mut self, value: value::Value) -> u8 {
    self.constants.push(value);
    return (self.constants.len() - 1) as u8;
  }
}