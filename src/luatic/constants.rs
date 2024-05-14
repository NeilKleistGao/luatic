use super::ast::{Expression, Program, Statement};

#[derive(Debug)]
pub enum Constant {
  Text { string: String, translation: bool }
  // TODO: more
}

pub type ConstTable = Vec<Constant>;

pub fn scan(prgm: &Program) -> ConstTable {
  let mut consts = vec![];
  for stmt in &prgm.0 {
    let mut res = scan_statement(stmt, false);
    consts.append(&mut res);
  }
  consts
}

fn scan_statement(stmt: &Statement, in_dialog: bool) -> ConstTable {
  let mut consts = vec![];
  match stmt {
    Statement::Dialog { name, block } => {
      consts.push(Constant::Text { string: name.clone(), translation: false });
      for s in block {
        let mut res = scan_statement(s, true);
        consts.append(&mut res);
      }
    }
    Statement::Expr(expr) => {
      let mut res = scan_expression(expr, in_dialog);
      consts.append(&mut res);
    }
    Statement::Say { name, texts } => {
      consts.push(Constant::Text { string: name.clone(), translation: true });
      for s in texts {
        consts.push(Constant::Text { string: s.clone(), translation: true });
      }
    }
  }

  consts
}

fn scan_expression(expr: &Expression, in_dialog: bool) -> ConstTable {
  let mut consts = vec![];
  match expr {
    Expression::StrLit(s) => {
      consts.push(Constant::Text { string: s.clone(), translation: in_dialog })
    }
  }

  consts
}
