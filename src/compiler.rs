use crate::value::Value;
use crate::chunk::{self, OpCode};
use crate::scanner::{Scanner, TokenType, Token};

struct Parser {
  pub previous: Token,
  pub current: Token,
}

impl Parser {
  pub fn new() -> Parser {
    Parser { 
      previous: Token{token_type: TokenType::Error, value: String::new(), line: 0},
      current: Token{token_type: TokenType::Error, value: String::new(), line: 0},
    }
  }
}

pub struct Compiler {
  current: usize,
  parser: Parser,
  scanner: Scanner,
  pub chunk: chunk::Chunk,
  pub had_error: bool,
}

impl Compiler {
  pub fn new(to_run: String) -> Self {
    Self{
      current: 0,
      scanner: Scanner::new(to_run),
      parser: Parser::new(),
      chunk: chunk::Chunk::new(),
      had_error: false,
    }
  }

  fn add_constant(&mut self, value: Value) -> u8 {
    println!("{:?}", value);
    self.chunk.add_constant(value);
    return u8::try_from(self.chunk.constants.len() - 1).ok().unwrap();
  }
  
  fn emit_bytes(&mut self, byte1: OpCode, byte2: u8) {
    self.chunk.write(byte1, self.current);
    self.chunk.write(byte2, self.current + 1);
  }
  
  fn emit_byte(&mut self, byte: OpCode) {
    self.chunk.write(byte, self.scanner.line);
  }
  
  fn items(&mut self, end: TokenType, op_code: OpCode) {
    let mut i = 0;
    while !self.examine(end.clone()) && !self.check(TokenType::EOF) {
      self.list();
      i += 1;
    }
    self.emit_bytes(op_code, i);
  }

  fn error(&mut self, message: &str) {
    self.had_error = true;
    println!("{}", message);
  }

  fn consume(&mut self, token_type: TokenType, message: &str) {
    if self.check(token_type) {
      self.advance();
      return;
    }
    self.error(message);
  }

  fn list(&mut self) {
    if self.examine(TokenType::BackTick) {
      if self.examine(TokenType::LeftParen) {
        self.emit_byte(OpCode::Nexecute);
        self.items(TokenType::RightParen, OpCode::List);
        self.emit_byte(OpCode::Yexecute);
      }
    } else if self.examine(TokenType::LeftParen) {
      if self.check(TokenType::Macro) {
        self.emit_byte(OpCode::Nexecute);
        self.items(TokenType::RightParen, OpCode::Vector);
        self.emit_byte(OpCode::Yexecute);
        self.emit_byte(OpCode::CallMacro)
      } else {
        self.items(TokenType::RightParen, OpCode::Vector);
        self.emit_byte(OpCode::Call);
      }
    } else if self.examine(TokenType::LeftSquare) {
      self.items(TokenType::RightSquare, OpCode::List);
    } else {
      self.identifier();
    }
  }

  fn constant_instruction(&mut self, value: Value) {
    self.advance();
    let constant = self.add_constant(value);
    self.emit_bytes(OpCode::Constant, constant);
  }

  fn identifier(&mut self) {
    if self.examine(TokenType::Identifier) || self.examine(TokenType::Macro) {
      let index = self.add_constant(Value::Symbol(self.parser.previous.value.clone()));
      self.emit_bytes(OpCode::GetGlobal, index);
    } else {
      self.immediate();
    }
  }

  fn immediate(&mut self) {
    let val = match self.parser.current.token_type {
      TokenType::Number => Value::Number(self.parser.current.value.parse::<f64>().unwrap()),
      TokenType::String => Value::String(self.parser.current.value.clone()),
      TokenType::Nil => Value::Nil,
      TokenType::Bool => Value::Boolean(self.parser.current.value == "true"),
      _ => return,
    };
    self.constant_instruction(val);
  }

  fn advance(&mut self) {
    self.parser.previous = self.parser.current.clone();
    self.parser.current = self.scanner.scan_token();
  }

  fn check(&mut self, token: TokenType) -> bool {
    return self.parser.current.token_type == token;
  }

  fn examine(&mut self, token: TokenType) -> bool {
    if self.check(token) {
      self.advance();
      return true;
    }
    return false;
  }
  
  pub fn compile(&mut self) {
    self.advance();
    loop {
      if self.check(TokenType::EOF) {
        self.emit_byte(OpCode::Return);
        break;
      }
      
      self.list();
    }
  }
}