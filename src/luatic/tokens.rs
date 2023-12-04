use super::exceptions::{Location, Exception};

pub enum Literal {
  StrLit(String),
  IntLit(i64),
  NumLit(f64),
  BoolLit(bool)
}

pub enum Token {
  Keyword{name: String, loc: Location},
  Identifier{name: String, loc: Location},
  LiteralValue{value: Literal, loc: Location},
  Punctuation{p: char, loc: Location}
}

pub struct TokenPack {
  tokens: Vec<Token>,
  exceptions: Vec<Exception>,
  filename: Option<String>
}

impl TokenPack {
  pub fn throw(&mut self, msg: String, loc: Location) {
    self.exceptions.push(Exception::ParsingException { msg: msg, loc: loc })
  }

  pub fn add(&mut self, token: Token) {
    self.tokens.push(token)
  }

  pub fn get_result(&self) -> Result<&Vec<Token>, &Vec<Exception>> {
    if self.exceptions.is_empty() {
      Result::Ok(&self.tokens)
    }
    else {
      Result::Err(&self.exceptions)
    }
  }

  pub fn new(filename: Option<String>) -> TokenPack {
    TokenPack { tokens: Vec::new(), exceptions: Vec::new(), filename: filename }
  }
}
