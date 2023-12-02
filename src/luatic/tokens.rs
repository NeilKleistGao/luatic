pub enum Literal {
  StrLit(String),
  IntLit(i64),
  NumLit(f64),
  BoolLit(bool)
}

pub enum Token {
  Keyword(String),
  Identifier(String),
  LiteralValue(Literal),
  Punctuation(char)
}
