static nil_tag: u8 = 0;
static false_tag: u8 = 1;
static true_tag: u8 = 17;
static num_tag: u8 = 3;
static int_tag: u8 = 19;
static short_str_tag: u8 = 4;
static long_str_tag: u8 = 20;

pub enum Literal {
  Nil,
  Boolean(bool),
  Number(f64),
  Int(i64),
  Str(String)
}

pub fn read_literal() -> Result<Literal, String> {
  Err("not implemented yet.".to_string())
}
