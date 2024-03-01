use std::collections::HashMap;
use std::path::Iter;

use super::ast::*;
use super::exceptions::{Exception, Location};
use super::tokens::Token;
use crate::binary::literals::Literal;

fn throw_parse_err(msg: String, loc: Location) -> Exception {
  Exception::ParsingException { msg: msg, loc: loc }
}

pub fn parse(tokens: &Vec<Token>) -> Result<Program, Vec<Exception>> {
  match parse_function(tokens.iter(), true) {
    Ok(info) => Ok(Program::new(info)),
    Err(err) => Err(err)
  }
}

fn parse_function(mut it: std::slice::Iter<Token>, is_top_level: bool) -> Result<FuncInfo, Vec<Exception>> {
  let mut constants: HashMap<Literal, u32> = HashMap::new();
  let mut errors: Vec<Exception> = Vec::new();

 loop {
  match it.next() {
    Some(Token::LiteralValue { value, loc: _ }) => {
      if !constants.contains_key(value) {
        constants.insert(value.clone(), u32::try_from(constants.len()).unwrap());
      }
    }
    Some(Token::Punctuation { p, loc }) => {
      if *p == '(' {
        match it.next() {
          Some(Token::Identifier { name, loc }) => {
            if *name == "def" {
              match it.next() {
                Some(Token::Identifier { name, loc }) => { // TODO: more it to another function
                  constants.insert(Literal::Str(name.clone()), u32::try_from(constants.len()).unwrap());
                }
                _ => () // TODO: error here
              }
            }
          }
          _ => () // TODO: error here
        }
      }
    }
    None => break,
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
