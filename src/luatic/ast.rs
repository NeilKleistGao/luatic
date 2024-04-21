pub enum AST {
  File(Vec<AST>)
}

impl Default for AST {
  fn default() -> Self {
    AST::File(vec![])
  }
}
