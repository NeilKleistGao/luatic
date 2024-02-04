use super::tokens::{Token, TokenPack};
use super::exceptions::{Position, Location};
use crate::binary::literals::Literal;

static mut LOC_ROW: usize = 1;
static mut LOC_COL: usize = 1;

fn new_line() {
  unsafe {
    LOC_ROW = LOC_ROW + 1;
    LOC_COL = 1;
  }
}

fn step() {
  unsafe {
    LOC_COL = LOC_COL + 1;
  }
}

fn reset_location() {
  unsafe {
    LOC_ROW = 1;
    LOC_COL = 1;
  }
}

fn get_position() -> Position {
  unsafe {
    Position::new(LOC_ROW, LOC_COL)
  }
}

fn next(cur: &mut std::str::Chars<'_>) -> Option<char> {
  loop {
    match cur.next() {
      res @ Some(c) => {
        if c == '\n' {
          new_line();
        }
        else {
          step();
        }

        return res
      }
      res => return res
    }
  }
}

fn swallow_line(cur: &mut std::str::Chars<'_>) {
  loop {
    match next(cur) {
      Some('\n') => {
        new_line();
        break
      }
      Some(_) => continue,
      None => break
    }
  }
}

fn parse_string(cur: &mut std::str::Chars<'_>) -> Result<Token, (String, Location)> {
  let mut s = String::new();
  let start_pos = get_position();
  loop {
    match next(cur) {
      Some('"') => break,
      Some('\\') => match next(cur) {
        Some('n') => s.push('\n'),
        Some('t') => s.push('\t'),
        Some('r') => s.push('\r'),
        _ => {
          swallow_line(cur);
          return Err(("unexpected escape.".to_string(), get_position() + 1))
        }
      }
      Some('\n') => return Err(("unexpected line feed.".to_string(), get_position() + 1)),
      Some(c) => s.push(c),
      None =>
        return Err(("expect ending of string.".to_string(), get_position() + 1))
    }
  }

  let len = s.len();
  Ok(Token::LiteralValue { value: Literal::Str(s), loc: start_pos + len})
}

fn parse_num(cur: &mut std::str::Chars<'_>, first: char) -> Result<Token, (String, Location)>{
  let mut s = first.to_string();
  let start_pos = get_position();

  loop {
    match next(cur) {
      Some(c) if c.is_whitespace() || c.is_ascii_control() || (c.is_ascii_punctuation() && c != '.') => break,
      Some(c) => s.push(c),
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

fn parse_ident(cur: &mut std::str::Chars<'_>, first: char) -> Token {
  let mut s = first.to_string();
  let start_pos = get_position();

  loop {
    match next(cur) {
      Some(c) if c.is_whitespace() || c.is_ascii_control() => break,
      Some(c) => s.push(c),
      None => break
    }
  }

  let len = s.len();
  Token::Identifier { name: s, loc: start_pos + len}
}

pub fn tokenize(code: String) -> TokenPack {
  reset_location();
  let mut pack = TokenPack::new();
  let mut cur = code.chars();

  loop {
    match next(&mut cur) {
      Some(c) => match c {
        ';' => swallow_line(&mut cur),
        '"' => match parse_string(&mut cur) {
          Ok(res) => pack.add(res),
          Err((msg, loc)) => pack.throw(msg, loc)
        }
        '(' | ')' | ',' | '[' | ']' | '{' | '}' => pack.add(Token::Punctuation { p: c, loc: get_position() + 1 }),
        c if c.is_numeric() => match parse_num(&mut cur, c) {
          Ok(res) => pack.add(res),
          Err((msg, loc)) => pack.throw(msg, loc)
        }
        c if c.is_whitespace() || c.is_ascii_control() => continue,
        c => pack.add(parse_ident(&mut cur, c))
      }
      None => break
    }
  }

  pack
}
