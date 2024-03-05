use std::collections::{HashMap, HashSet};

use super::ast::*;
use super::exceptions::{Exception, Location};
use super::tokens::Token;
use crate::binary::literals::Literal;

fn throw_parse_err(msg: String, loc: &Location) -> Exception {
  Exception::ParsingException { msg: msg, loc: loc.clone() }
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
  let mut bindings: HashSet<String> = HashSet::new();

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
              parse_definition(&mut it, &mut constants, &mut bindings, &mut errors, &loc, is_top_level)
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

fn parse_definition(
  it: &mut std::slice::Iter<Token>,
  constants: &mut HashMap<Literal, u32>,
  bindings: &mut HashSet<String>,
  errors: &mut Vec<Exception>,
  prev_loc: &Location,
  is_top_level: bool
) {
  match it.next() {
    Some(Token::Identifier { name, loc }) => {
      if bindings.contains(name) {
        errors.push(throw_parse_err("duplicated variable name ".to_string() + name + ".", loc));
      }
      else {
        bindings.insert(name.clone());
        constants.insert(Literal::Str(name.clone()), u32::try_from(constants.len()).unwrap());
        // TODO: parse expression
      }
    }
    Some(tok) => {
      errors.push(throw_parse_err("a variable name is expected, but ".to_string() + &tok.describe() + " is found.", tok.get_pos()));
    }
    _ => {
      errors.push(throw_parse_err("a variable name is expected, but EOF is found.".to_string(), prev_loc));
    }
  }
}
