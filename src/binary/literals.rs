use super::binary::Binary;

static NIL_TAG: u8 = 0;
static FALSE_TAG: u8 = 1;
static TRUE_TAG: u8 = 17;
static NUM_TAG: u8 = 3;
static INT_TAG: u8 = 19;
static SHORT_STR_TAG: u8 = 4;
static LONG_STR_TAG: u8 = 20;

static LONG_STR_LEN: usize = 40;

pub enum Literal {
  Nil,
  Boolean(bool),
  Number(f64),
  Int(i64),
  Str(String)
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
