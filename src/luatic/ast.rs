pub trait ExpressionLike {
  
}

pub trait StatementLike {
  
}

pub enum Literal {
  NilLit,
  BoolLit(bool),
  NumLit(f64),
  IntLit(i64),
  StrLit(String)
}

impl ExpressionLike for Literal {
  
}

impl StatementLike for Literal {
  
}

pub struct Program {
  instructions: Vec<Box<dyn StatementLike>>
}

impl Program {
  pub fn new(ins: Vec<Box<dyn StatementLike>>) -> Program {
    Program {instructions: ins}
  }
}
