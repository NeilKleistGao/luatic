pub enum Expression {
  IntLit(i64),
  NumLit(f64),
  StrLit(String),
  BoolLit(bool)
}

pub enum Statement {
  Global { name: String, value: Expression }
}

pub struct Program (Vec<Statement>);
impl Program {
  pub fn new(stmts: Vec<Statement>) -> Program {
    Program(stmts)
  }
}
