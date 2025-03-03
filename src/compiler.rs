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

  fn get_name(&mut self) -> u8 {
    self.consume(TokenType::Identifier, "Expected identifier.");
    return self.add_constant(Value::String(self.parser.previous.value.clone()));
  }
  
  fn get_var(&mut self) {
    let name = self.get_name();
    self.emit_bytes(OpCode::GetGlobal, name);
  }
  
  fn args(&mut self) -> u8 {
    let mut i = 0;
    while !self.examine(TokenType::RightParen) && !self.check(TokenType::EOF) {
      self.call();
      i += 1;
    }
    return i;
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

  fn function(&mut self) {
    
  }

  fn call(&mut self) {
    if self.examine(TokenType::LeftParen) {
      match self.parser.current.token_type {
        // add macros later to make this less worse hopefully
        TokenType::Identifier if self.parser.current.value == "fn" => {
          self.function();
        },
        _ => {
          let num = self.args();
          self.emit_bytes(OpCode::Call, num);
        }
      }
    } else {
      self.immediate();
    }
  }

  fn immediate(&mut self) {
    if self.examine(TokenType::Number) {
      let constant = self.add_constant(Value::Number(self.parser.previous.value.parse::<f64>().unwrap()));
      self.emit_bytes(OpCode::Constant, constant);
    } else if self.examine(TokenType::String) {
      let constant = self.add_constant(Value::String(self.parser.previous.value.clone()));
      self.emit_bytes(OpCode::Constant, constant);
    } else if self.examine(TokenType::Label) {
      println!("labeld");
      let constant = self.add_constant(Value::Label(self.parser.previous.value.clone()));
      self.emit_bytes(OpCode::Constant, constant);
    } else if self.examine(TokenType::Nil) {
      let constant = self.add_constant(Value::Nil);
      self.emit_bytes(OpCode::Constant, constant);
    }else if self.check(TokenType::Identifier) {
      self.get_var();
    }
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
      
      self.call();
    }
  }
}