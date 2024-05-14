pub mod luaticlexer;
pub mod luaticparser;
pub mod luaticlistener;
pub mod luaticvisitor;

use crate::luatic::ast::{Program, Statement, Expression};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, Tree};
use antlr_rust::InputStream;
use crate::frontend::luaticlexer::LuaticLexer;
use crate::frontend::luaticparser::{LuaticParser, LuaticParserContextType};
use crate::frontend::luaticvisitor::LuaticVisitorCompat;

use self::luaticparser::*;

enum VisitorResult {
  Nothing,
  Prgm(Program),
  Stmt(Statement),
  Expr(Expression),
  Block(Vec<Statement>)
}
  
impl Default for VisitorResult {
  fn default() -> Self {
    Self::Nothing
  }
}
  
struct Visitor(VisitorResult);
impl ParseTreeVisitorCompat<'_> for Visitor {
  type Node = LuaticParserContextType;
  type Return = VisitorResult;
  
  fn temp_result(&mut self) -> &mut Self::Return {
    &mut self.0
  }
  
  fn visit_terminal(&mut self, _node: &antlr_rust::tree::TerminalNode<'_, Self::Node>) -> Self::Return {
    Self::Return::default()
  }
  
  fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
    next
  }
}
  
impl LuaticVisitorCompat<'_> for Visitor {
//   fn visit_integer(&mut self, ctx: &crate::frontend::luaticparser::IntegerContext<'_>) -> Self::Return {
//     VisitorResult::Expr(Expression::IntLit(ctx.INT().unwrap_or_else(|| ctx.HEX().unwrap()).get_text().parse().unwrap()))
//   }

//   fn visit_number(&mut self, ctx: &luaticparser::NumberContext<'_>) -> Self::Return {
//     VisitorResult::Expr(Expression::NumLit(ctx.FLOAT().unwrap_or_else(|| ctx.HEX_FLOAT().unwrap()).get_text().parse().unwrap()))
//   }

//   fn visit_boolean(&mut self, ctx: &luaticparser::BooleanContext<'_>) -> Self::Return {
//     VisitorResult::Expr(Expression::BoolLit(ctx.KW_TRUE().is_some()))
//   }

  fn visit_string(&mut self, ctx: &luaticparser::StringContext<'_>) -> Self::Return {
    VisitorResult::Expr(Expression::StrLit(ctx.NORMALSTRING().unwrap().get_text()))
  }

  fn visit_character(&mut self, ctx: &CharacterContext<'_>) -> Self::Return {
    VisitorResult::Expr(Expression::StrLit(ctx.CHARASTRING().unwrap().get_text()))
  }

//   fn visit_expr(&mut self, ctx: &luaticparser::ExprContext<'_>) -> Self::Return {
//     self.visit_children(ctx)
//   }

//   fn visit_global_stat(&mut self, ctx: &luaticparser::Global_statContext<'_>) -> Self::Return {
//     let name = ctx.IDENT().unwrap().get_text();
//     let value = match self.visit_expr(&*ctx.expr().unwrap()) {
//       VisitorResult::Expr(expr) => expr,
//       _ => panic!("unexpected error.")
//     };
//     VisitorResult::Stmt(Statement::Global { name, value })
//   }

  fn visit_block(&mut self, ctx: &BlockContext<'_>) -> Self::Return {
    let mut statements = Vec::<Statement>::new();
    for rs in &*ctx.stat_all() {
      statements.push(match self.visit_stat(rs.as_ref()) {
        VisitorResult::Stmt(stmt) => stmt,
        _ => panic!("unexpected error.")
      });
    }
    VisitorResult::Block(statements)
  }

  fn visit_say_block(&mut self, ctx: &Say_blockContext<'_>) -> Self::Return {
    let mut statements = Vec::<Statement>::new();
    for rs in &*ctx.string_all() {
      statements.push(match self.visit_string(rs.as_ref()) {
        VisitorResult::Expr(expr) => Statement::Expr(expr),
        _ => panic!("unexpected error.")
      });
    }
    VisitorResult::Block(statements)
  }

  fn visit_say_stat(&mut self, ctx: &Say_statContext<'_>) -> Self::Return {
    let name = match self.visit_character(&*ctx.character().unwrap()) {
      VisitorResult::Expr(Expression::StrLit(c)) => c,
      _ => panic!("unexpected error.")
    };

    let texts = match ctx.say_block() {
      Some(blk) => match self.visit_say_block(&blk) {
        VisitorResult::Block(stmts) =>
          stmts.iter().map(|s| match s {
            Statement::Expr(Expression::StrLit(res)) => res.clone(),
            _ => panic!("unexpected error.")
          }).collect::<Vec<String>>(),
        _ => panic!("unexpected error.")
      }
      _ => vec![match self.visit_string(&*ctx.string().unwrap()) {
        VisitorResult::Expr(Expression::StrLit(s)) => s,
        _ => panic!("unexpected error.")
      }]
    };

    VisitorResult::Stmt(Statement::Say { name, texts })
  }

  fn visit_dialog_block(&mut self, ctx: &Dialog_blockContext<'_>) -> Self::Return {
    let statements = match self.visit_block(&*ctx.block().unwrap()) {
      VisitorResult::Block(stmts) => stmts,
      _ => panic!("unexpected error.")
    };
    VisitorResult::Stmt(Statement::Dialog { name: ctx.IDENT().unwrap().get_text(), block: statements })
  }

  fn visit_stat(&mut self, ctx: &luaticparser::StatContext<'_>) -> Self::Return {
    self.visit_children(ctx)
  }

  fn visit_prgm(&mut self, ctx: &luaticparser::PrgmContext<'_>) -> Self::Return {
    let mut statements = Vec::<Statement>::new();
    for rs in &*ctx.dialog_block_all() {
      statements.push(match self.visit_dialog_block(rs.as_ref()) {
        VisitorResult::Stmt(stmt) => stmt,
        _ => panic!("unexpected error.")
      });
    }

    VisitorResult::Prgm(Program::new(statements))
  }
}

pub fn parse(code: String) -> Result<Program, String> {
  let lexer: LuaticLexer<'_, InputStream<&str>> = LuaticLexer::new(InputStream::new(&code));
  let token_source = CommonTokenStream::new(lexer);
  let mut parser = LuaticParser::new(token_source);

  let root = parser.prgm().unwrap();
  let mut visitor = Visitor(VisitorResult::Nothing);
  match visitor.visit(&*root) {
    VisitorResult::Prgm(program) => Ok(program),
    _ => Err("unexpected error.".to_string())
  }
}
