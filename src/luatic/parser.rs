use std::collections::HashMap;

use super::ast::*;
use super::exceptions::{Exception, Location};
use super::tokens::Token;
use crate::binary::literals::Literal;

fn throw_parse_err(msg: String, loc: Location) -> Exception {
  Exception::ParsingException { msg: msg, loc: loc }
}

pub fn parse(tokens: &Vec<Token>) -> Result<Program, Vec<Exception>> {
  match parse_function(tokens) {
    Ok(info) => Ok(Program::new(info)),
    Err(err) => Err(err)
  }
}

fn parse_function(tokens: &Vec<Token>) -> Result<FuncInfo, Vec<Exception>> {
  let mut constants: HashMap<Literal, u32> = HashMap::new();
  let mut errors: Vec<Exception> = Vec::new();

  for tok in tokens {
    match tok {
      Token::LiteralValue { value, loc: _ } => {
        if !constants.contains_key(value) {
          constants.insert(value.clone(), u32::try_from(constants.len()).unwrap());
        }
      }
      _ => () // TODO
    }
  }

  if errors.is_empty() {
    Ok(FuncInfo::new(constants))
  }
  else {
    Err(errors)
  }
}
