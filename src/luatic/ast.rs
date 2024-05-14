#[derive(Debug)]
pub enum Expression {
  // IntLit(i64),
  // NumLit(f64),
  StrLit(String),
  // BoolLit(bool)
}

#[derive(Debug)]
pub enum Statement {
  // Global { name: String, value: Expression }
  Say { name: String, texts: Vec<String> },
  Dialog { name: String, block: Vec<Statement> },
  Expr(Expression)
}

#[derive(Debug)]
pub struct Program (pub Vec<Statement>, pub String);
impl Program {
  pub fn new(stmts: Vec<Statement>, default_lang: String) -> Program {
    Program(stmts, default_lang)
  }
}
