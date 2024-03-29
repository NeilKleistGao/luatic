use super::exceptions::{Location, Exception};
use crate::binary::literals::Literal;

pub enum Token {
  Identifier{name: String, loc: Location},
  LiteralValue{value: Literal, loc: Location},
  Punctuation{p: char, loc: Location}
}

impl Token {
  pub fn describe(&self) -> String {
    match self {
      Token::Identifier { .. } => "identifier".to_string(),
      Token::LiteralValue { .. } => "literal value".to_string(),
      Token::Punctuation { .. } => "punctuation".to_string()
    }
  }

  pub fn get_pos(&self) -> &Location {
    match self {
      Token::Identifier { name: _, loc } => loc,
      Token::LiteralValue { value: _, loc } => loc,
      Token::Punctuation { p: _, loc } => loc
    }
  }
}

pub struct TokenPack {
  tokens: Vec<Token>,
  exceptions: Vec<Exception>
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

  pub fn new() -> TokenPack {
    TokenPack { tokens: Vec::new(), exceptions: Vec::new() }
  }
}
