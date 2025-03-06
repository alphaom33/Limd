use enum_index::IndexEnum;
use std::io::{self, Write};

use crate::chunk::{Chunk, OpCode};

impl Chunk {
  pub fn disassemble(&self, name: &str) {
    println!("== {} ==", name);

    println!("{:?}", self.constants);
    
    let mut offset = 0;
    while offset < self.code.len() {
      offset = self.disassemble_instruction(offset);
    }
  }

  fn simple_instruction(&self, name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
  }

  fn constant_instruction(&self, name: &str, offset: usize) -> usize {
    let constant = self.code[offset + 1];
    println!("{:16} {:4} '{}'", name, constant, self.constants[constant as usize]);
    return offset + 2;
  }

  fn byte_instruction(&self, name: &str, offset: usize) -> usize {
    let byte = self.code[offset + 1];
    println!("{:16} {:4}", name, byte);
    return offset + 2;
  }

  fn disassemble_instruction(&self, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
      print!("   | ");
    } else {
      print!("{:4} ", self.lines[offset]);
    }
    
    let op_code_option = OpCode::index_enum(self.code[offset] as usize);
    return match op_code_option {
      Some(op_code) => match op_code {
        OpCode::Return => self.simple_instruction("OP_RETURN", offset),
        OpCode::Constant => self.constant_instruction("OP_CONSTANT", offset),
        OpCode::Vector => self.byte_instruction("OP_VECTOR", offset),
        OpCode::List => self.byte_instruction("OP_LIST", offset),
        OpCode::Call => self.simple_instruction("OP_CALL", offset),
        OpCode::CallMacro => self.simple_instruction("OP_CALL__MACRO", offset),
        OpCode::Yexecute => self.simple_instruction("OP_YEXECUTE", offset),
        OpCode::Nexecute => self.simple_instruction("OP_NEXECUTE", offset),
        OpCode::GetGlobal => self.constant_instruction("OP_GETGLOBAL", offset),
      },
      None => self.simple_instruction(&format!("Unknown opcode {}", self.code[offset]), offset)
    }
  }
}