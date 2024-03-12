use super::tokens::{Token, TokenPack};
use super::exceptions::{Position, Location};
use super::scanner::Scanner;
use crate::binary::literals::Literal;

struct Source {
  code: Vec<char>,
  row: usize,
  col: usize,
  loc: usize,
  len: usize
}

impl Source {
  pub fn new(code: String) -> Source {
    Source { code: code.chars().collect(), row: 1, col: 1, loc: 0, len: code.len() }
  }
}

impl Scanner<char> for Source {
  fn consume(&mut self) {
    match self.cur() {
      Some('\n') => {
        self.row += 1;
        self.col = 1;
        self.loc += 1;
      },
      Some(_) => {
        self.col += 1;
        self.loc += 1;
      }
      None => {}
    }
  }

  fn cur(&self) -> Option<char> {
    if self.loc < self.len {
      Some(self.code[self.loc])
    }
    else {
      None
    }
  }

  fn consume_until(&mut self, predicate: fn(char) -> bool) {
    loop {
      match self.cur() {
        Some(c) => if !predicate(c) {
          self.consume();
        }
        else {
          break;
        }
        _ => break
      }
    }
  }

  fn look_ahead(&self, step: usize) -> Option<char> {
    if self.loc + step < self.len {
      Some(self.code[self.loc + step])
    }
    else {
      None
    }
  }

  fn get_position(&self) -> Position {
    Position::new(self.row, self.col)
  }
}

fn swallow_line(source: &mut Source) {
  source.consume_until(|c: char| c == '\n');
  source.consume();
}

fn parse_string(source: &mut Source) -> Result<Token, (String, Location)> {
  let mut s = String::new();
  let start_pos = source.get_position();
  source.consume();

  loop {
    match source.cur() {
      Some('"') => {
        source.consume();
        break;
      },
      Some('\\') => {
        let pos = source.get_position();
        source.consume();
        match source.cur() {
          Some('n') => s.push('\n'),
          Some('t') => s.push('\t'),
          Some('r') => s.push('\r'),
          Some('"') => s.push('"'),
          Some('\\') => s.push('\\'),
          _ => {
            source.consume_until(|c: char| c == '"');
            source.consume();
            return Err(("unexpected escape.".to_string(), pos + 2))
          }
        }
      },
      Some('\n') => {
        let pos = source.get_position();
        source.consume();
        return Err(("unexpected line feed.".to_string(), pos - 1));
      },
      Some(c) => {
        source.consume();
        s.push(c);
      },
      None =>
        return Err(("expect ending of string.".to_string(), source.get_position() + 1))
    }
  }

  let len = s.len();
  Ok(Token::LiteralValue { value: Literal::Str(s), loc: start_pos + len})
}

fn parse_num(source: &mut Source) -> Result<Token, (String, Location)>{
  let mut s = String::new();
  let start_pos = source.get_position();

  loop {
    match source.cur() {
      Some(c) if c.is_whitespace() || c.is_ascii_control() || (c.is_ascii_punctuation() && c != '.' && c != '-') => break,
      Some(c) => {
        source.consume();
        s.push(c);
      },
      None => break
    }
  }

  let len = s.len();
  match s.parse::<i64>() {
    Ok(num) => Ok(Token::LiteralValue { value: Literal::Int(num), loc: start_pos + len }),
    Err(_) => match s.parse::<f64>() {
      Ok(num) => Ok(Token::LiteralValue { value: Literal::Number(num), loc: start_pos + len }),
      Err(_) => Err(("wrong number format.".to_string(), start_pos + len))
    }
  }
}

fn parse_ident(source: &mut Source) -> Token {
  let mut s = String::new();
  let start_pos = source.get_position();

  loop {
    match source.cur() {
      Some(c) if c.is_whitespace() || c.is_ascii_control() => break,
      Some(c) => {
        source.consume();
        s.push(c);
      },
      None => break
    }
  }

  let len = s.len();
  if s == "nil" {
    Token::LiteralValue { value: Literal::Nil, loc: start_pos + len }
  }
  else if s == "true" {
    Token::LiteralValue { value: Literal::Boolean(true), loc: start_pos + len }
  }
  else if s == "false" {
    Token::LiteralValue { value: Literal::Boolean(false), loc: start_pos + len }
  }
  else {
    Token::Identifier { name: s, loc: start_pos + len}
  }
}

pub fn tokenize(code: String) -> TokenPack {
  let mut pack = TokenPack::new();
  let mut source = Source::new(code);

  loop {
    match source.cur() {
      Some(c) => match c {
        ';' => { // * Comment
          swallow_line(&mut source);
        },
        '"' => match parse_string(&mut source) { // * String
          Ok(res) => pack.add(res),
          Err((msg, loc)) => pack.throw(msg, loc)
        }
        '(' | ')' | ',' | '[' | ']' | '{' | '}' => {
          source.consume();
          pack.add(Token::Punctuation { p: c, loc: source.get_position() - 1 });
        },
        c if c.is_numeric() => match parse_num(&mut source) {
          Ok(res) => pack.add(res),
          Err((msg, loc)) => pack.throw(msg, loc)
        }
        c if c.is_whitespace() => {
          source.consume();
          continue;
        },
        c if c.is_ascii_control() => {
          source.consume();
          pack.throw("unexpected character.".to_string(), source.get_position() - 1);
        },
        _c => pack.add(parse_ident(&mut source))
      }
      None => break
    }
  }

  pack
}
