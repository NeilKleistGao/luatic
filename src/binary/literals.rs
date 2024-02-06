use std::hash::{Hash, Hasher};
use super::binary::Binary;

static NIL_TAG: u8 = 0;
static FALSE_TAG: u8 = 1;
static TRUE_TAG: u8 = 17;
static NUM_TAG: u8 = 3;
static INT_TAG: u8 = 19;
static SHORT_STR_TAG: u8 = 4;
static LONG_STR_TAG: u8 = 20;

static LONG_STR_LEN: usize = 40;

#[derive(Clone)]
pub enum Literal {
  Nil,
  Boolean(bool),
  Number(f64),
  Int(i64),
  Str(String)
}

impl std::cmp::PartialEq for Literal {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
      (Self::Number(l0), Self::Number(r0)) => l0 == r0,
      (Self::Int(l0), Self::Int(r0)) => l0 == r0,
      (Self::Str(l0), Self::Str(r0)) => l0 == r0,
      (Self::Nil, Self::Nil) => true,
      _ => false
    }
  }
}

impl std::cmp::Eq for Literal {
}

impl Hash for Literal {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Self::Boolean(b) => b.hash(state),
      Self::Int(i) => i.hash(state),
      Self::Nil => 0.hash(state),
      Self::Str(s) => s.hash(state),
      Self::Number(n) => n.to_be_bytes().hash(state)
    }
  }
}

impl Binary for Literal {
  fn to_binary(&self, to: &mut Vec<u8>) -> Result<(), String> {
    match self {
      Self::Nil => to.push(NIL_TAG),
      Self::Boolean(b) => if *b { to.push(TRUE_TAG) } else { to.push(FALSE_TAG) },
      Self::Number(n) => {
        to.push(NUM_TAG);
        return n.to_binary(to);
      },
      Self::Int(i) => {
        to.push(INT_TAG);
        return i.to_binary(to);
      },
      Self::Str(s) => {
        if s.len() <= LONG_STR_LEN {
          to.push(SHORT_STR_TAG);
          return s.to_binary(to);
        }
        else {
          to.push(LONG_STR_TAG);
          return s.to_binary(to); // TODO: need to change?
        }
      }
    }

    Ok(())
  }
}
