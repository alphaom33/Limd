pub struct Scanner {
    pub line: usize,
    pub source: String,
    pub current: usize,
  }
  
  #[derive(Debug, PartialEq, Clone)]
  pub enum TokenType {
    LeftParen,  
    RightParen,  
    Number,
    Identifier,
    String,
    Error,
    EOF,
  }
  
  #[derive(Debug, Clone)]
  pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
  }
  
  impl Scanner {
    pub fn new(source: String) -> Self {
      return Self { 
        line: 0,
        source,
        current: 0
      };
    }
  
    fn make_token(&self, token_type: TokenType, value: &str) -> Token {
      return Token {
        token_type,
        value: value.to_owned(),
        line: self.line,
      }
    }
  
    fn string(&mut self) -> Token {
      let start = self.current;
      while self.peek().unwrap() != '"' {
        println!("{:?}", self.peek());
        self.advance();
      }
      self.advance();
      
      return Token{
        token_type: TokenType::String,
        value: self.source[start..self.current - 1].to_owned(),
        line: self.line,
      }
    }
    
    fn number(&mut self) -> Token {
      let start = self.current - 1;
      while let Some(s) = self.peek() {
        if !s.is_numeric() {
          break;
        }
  
        self.advance();
      }
  
      return Token{
        token_type: TokenType::Number,
        value: self.source[start..self.current].to_owned(),
        line: self.line,
      }
    }
  
    fn identifier(&mut self) -> Token {
      let start = self.current - 1;
      while let Some(s) = self.peek() {
        if " \n\t,()".contains(s) {
          break;
        }
        
        self.advance();
      }
      
      return Token{
        token_type: TokenType::Identifier,
        value: self.source[start..self.current].to_owned(),
        line: self.line};
    }
  
    fn skip_white(&mut self) {
      while let Some(c) = self.peek() {
        match c {
          '\n' => {
            self.line += 1;
            self.current += 1;
          }
          ' ' | '\r' | '\t' => self.current += 1,
          _ => break,
        }    
      }
    }
  
    fn peek(&mut self) -> Option<char> {
      return self.source.chars().nth(self.current);
    }
  
    fn advance(&mut self) -> char {
      let out = self.peek().unwrap();
      self.current += 1;
      return out;
    }
    
    pub fn scan_token(&mut self) -> Token {
      self.skip_white();
      if self.peek().is_none() {
        return self.make_token(TokenType::EOF, "");
      }
  
      let next = self.advance();
      return match next {
        '(' => self.make_token(TokenType::LeftParen, "("),
        ')' => self.make_token(TokenType::RightParen, ")"),
        '"' => self.string(),
        _ => {
          if next.is_numeric() {
            self.number()
          } else {
            self.identifier()
          }
        }
      };
    }
  }