use super::tokens::{Token, TokenPack, Literal};
use super::exceptions::{Position, Location, Exception};

static mut LOC_ROW: usize = 1;
static mut LOC_COL: usize = 1;

fn new_line() {
  unsafe {
    LOC_ROW = LOC_ROW + 1;
    LOC_COL = 1;
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

fn swallow_line(cur: &mut std::str::Chars<'_>) {
  loop {
    match cur.next() {
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
    match cur.next() {
      Some('"') => break,
      Some('\\') => match cur.next() {
        Some('n') => s.push('\n'),
        Some('t') => s.push('\t'),
        Some('r') => s.push('\r'),
        _ => {
          swallow_line(cur);
          return Err(("unexpected escape.".to_string(), get_position() + 1))
        }
      }
      Some(c) => s.push(c),
      None =>
        return Err(("expect ending of string.".to_string(), get_position() + 1))
    }
  }

  let len = s.len();
  Ok(Token::LiteralValue { value: Literal::StrLit(s), loc: start_pos +  len})
}

pub fn tokenize(code: String) -> TokenPack {
  reset_location();
  let mut pack = TokenPack::new();
  let mut cur = code.chars();

  loop {
    match cur.next() {
      Some(c) => match c {
        ';' => swallow_line(&mut cur),
        '"' => match parse_string(&mut cur) {
          Ok(res) => pack.add(res),
          Err((msg, loc)) => pack.throw(msg, loc)
        }
        c => {
          print!("{:?}", c); continue
        }
      }
      None => break
    }
  }

  pack
}
