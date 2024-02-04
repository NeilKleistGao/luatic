use std::collections::HashMap;

use super::ast::*;
use super::tokens::Token;
use crate::binary::literals::Literal;

// TODO: error
pub fn parse(tokens: &Vec<Token>) -> Program {
  Program::new(parse_function(tokens))
}

fn parse_function(tokens: &Vec<Token>) -> FuncInfo {
  let mut constants: HashMap<Literal, u32> = HashMap::new();

  for tok in tokens {
    match tok {
      Token::LiteralValue { value, loc } => {
      }
      _ => () // TODO
    }
  }
  FuncInfo::new(constants)
}
