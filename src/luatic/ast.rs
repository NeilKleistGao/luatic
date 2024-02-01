use crate::binary::literals::Literal;

pub trait ExpressionLike {
  
}

pub trait StatementLike {
  
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
