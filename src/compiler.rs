use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::InputStream;
use crate::frontend::luaticlexer::LuaticLexer;
use crate::frontend::luaticparser::{LuaticParser, LuaticParserContextType};
use crate::frontend::luaticvisitor::LuaticVisitorCompat;
use crate::luatic::ast::AST;

struct CompileOption {
  filename: String,
  output: (String, String, String)
}
  
impl CompileOption {
  pub fn new(filename: String) -> CompileOption {
    let binary_name = filename.replace(".ltc", ".luac");
    let trans_name = filename.replace(".ltc", ".csv");
    let map_name = filename.replace(".ltc", ".map");
    CompileOption { filename: filename, output: (binary_name, trans_name, map_name) }
  }
}

struct Visitor(AST);
impl ParseTreeVisitorCompat<'_> for Visitor {
    type Node = LuaticParserContextType;
    type Return = AST;

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
  // TODO:
}

fn read_to_string(path: &String) -> Result<String, String> {
  match std::fs::read_to_string(path) {
    Ok(code) => Ok(code),
    Err(err) => Err(err.to_string())  
  }
}

pub fn compile(filename: String) -> Result<(), String> {
  let option = CompileOption::new(filename);
  let code = read_to_string(&option.filename)?;

  let lexer: LuaticLexer<'_, InputStream<&str>> = LuaticLexer::new(InputStream::new(&code));
  let token_source = CommonTokenStream::new(lexer);
  let mut parser = LuaticParser::new(token_source);

  let root = parser.string().unwrap();
  let mut visitor = Visitor(AST::default());
  let program = visitor.visit(&*root);

  Ok(())
}
