// Generated from Luatic.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::luaticlistener::*;
use super::luaticvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const KW_GLOBAL:isize=1; 
		pub const KW_TRUE:isize=2; 
		pub const KW_FALSE:isize=3; 
		pub const KW_LOCAL:isize=4; 
		pub const KW_IN:isize=5; 
		pub const KW_IF:isize=6; 
		pub const KW_ELSE:isize=7; 
		pub const KW_WHILE:isize=8; 
		pub const KW_FOR:isize=9; 
		pub const KW_FUNC:isize=10; 
		pub const KW_DO:isize=11; 
		pub const KW_RET:isize=12; 
		pub const KW_AND:isize=13; 
		pub const KW_OR:isize=14; 
		pub const KW_NOT:isize=15; 
		pub const PT_SEMI:isize=16; 
		pub const PT_COMMA:isize=17; 
		pub const PT_ASGN:isize=18; 
		pub const PT_LT:isize=19; 
		pub const PT_RT:isize=20; 
		pub const PT_LP:isize=21; 
		pub const PT_RP:isize=22; 
		pub const PT_LB:isize=23; 
		pub const PT_RB:isize=24; 
		pub const PT_LS:isize=25; 
		pub const PT_RS:isize=26; 
		pub const PT_PLUS:isize=27; 
		pub const PT_MINUS:isize=28; 
		pub const PT_MUL:isize=29; 
		pub const PT_DIV:isize=30; 
		pub const PT_IDIV:isize=31; 
		pub const PT_MOD:isize=32; 
		pub const PT_LE:isize=33; 
		pub const PT_GE:isize=34; 
		pub const PT_EQ:isize=35; 
		pub const PT_NE:isize=36; 
		pub const PT_AND:isize=37; 
		pub const PT_OR:isize=38; 
		pub const PT_XN:isize=39; 
		pub const PT_LSF:isize=40; 
		pub const PT_RSF:isize=41; 
		pub const PT_POW:isize=42; 
		pub const IDENT:isize=43; 
		pub const NORMALSTRING:isize=44; 
		pub const INT:isize=45; 
		pub const HEX:isize=46; 
		pub const FLOAT:isize=47; 
		pub const HEX_FLOAT:isize=48; 
		pub const COMMENT:isize=49; 
		pub const LINE_COMMENT:isize=50; 
		pub const WS:isize=51;
	pub const RULE_expr:usize = 0; 
	pub const RULE_stat:usize = 1; 
	pub const RULE_prgm:usize = 2; 
	pub const RULE_block:usize = 3; 
	pub const RULE_global_stat:usize = 4; 
	pub const RULE_local_stat:usize = 5; 
	pub const RULE_local_in_stat:usize = 6; 
	pub const RULE_full_if_stat:usize = 7; 
	pub const RULE_while_stat:usize = 8; 
	pub const RULE_do_stat:usize = 9; 
	pub const RULE_for_in_stat:usize = 10; 
	pub const RULE_func_stat:usize = 11; 
	pub const RULE_ret_stat:usize = 12; 
	pub const RULE_boolean:usize = 13; 
	pub const RULE_integer:usize = 14; 
	pub const RULE_number:usize = 15; 
	pub const RULE_string:usize = 16; 
	pub const RULE_if_stat:usize = 17; 
	pub const RULE_elif_stat:usize = 18; 
	pub const RULE_else_state:usize = 19; 
	pub const RULE_param_list:usize = 20; 
	pub const RULE_array:usize = 21; 
	pub const RULE_kv_pair:usize = 22; 
	pub const RULE_table:usize = 23; 
	pub const RULE_lambda:usize = 24;
	pub const ruleNames: [&'static str; 25] =  [
		"expr", "stat", "prgm", "block", "global_stat", "local_stat", "local_in_stat", 
		"full_if_stat", "while_stat", "do_stat", "for_in_stat", "func_stat", "ret_stat", 
		"boolean", "integer", "number", "string", "if_stat", "elif_stat", "else_state", 
		"param_list", "array", "kv_pair", "table", "lambda"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;43] = [
		None, Some("'global'"), Some("'true'"), Some("'false'"), Some("'local'"), 
		Some("'in'"), Some("'if'"), Some("'else'"), Some("'while'"), Some("'for'"), 
		Some("'function'"), Some("'do'"), Some("'return'"), Some("'and'"), Some("'or'"), 
		Some("'not'"), Some("';'"), Some("','"), Some("'='"), Some("'<'"), Some("'>'"), 
		Some("'('"), Some("')'"), Some("'{'"), Some("'}'"), Some("'['"), Some("']'"), 
		Some("'+'"), Some("'-'"), Some("'*'"), Some("'/'"), Some("'//'"), Some("'%'"), 
		Some("'<='"), Some("'>='"), Some("'=='"), Some("'~='"), Some("'&'"), Some("'|'"), 
		Some("'~'"), Some("'<<'"), Some("'>>'"), Some("'^'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;52]  = [
		None, Some("KW_GLOBAL"), Some("KW_TRUE"), Some("KW_FALSE"), Some("KW_LOCAL"), 
		Some("KW_IN"), Some("KW_IF"), Some("KW_ELSE"), Some("KW_WHILE"), Some("KW_FOR"), 
		Some("KW_FUNC"), Some("KW_DO"), Some("KW_RET"), Some("KW_AND"), Some("KW_OR"), 
		Some("KW_NOT"), Some("PT_SEMI"), Some("PT_COMMA"), Some("PT_ASGN"), Some("PT_LT"), 
		Some("PT_RT"), Some("PT_LP"), Some("PT_RP"), Some("PT_LB"), Some("PT_RB"), 
		Some("PT_LS"), Some("PT_RS"), Some("PT_PLUS"), Some("PT_MINUS"), Some("PT_MUL"), 
		Some("PT_DIV"), Some("PT_IDIV"), Some("PT_MOD"), Some("PT_LE"), Some("PT_GE"), 
		Some("PT_EQ"), Some("PT_NE"), Some("PT_AND"), Some("PT_OR"), Some("PT_XN"), 
		Some("PT_LSF"), Some("PT_RSF"), Some("PT_POW"), Some("IDENT"), Some("NORMALSTRING"), 
		Some("INT"), Some("HEX"), Some("FLOAT"), Some("HEX_FLOAT"), Some("COMMENT"), 
		Some("LINE_COMMENT"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,LuaticParserExt<'input>, I, LuaticParserContextType , dyn LuaticListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type LuaticTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, LuaticParserContextType , dyn LuaticListener<'input> + 'a>;

/// Parser for Luatic grammar
pub struct LuaticParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				LuaticParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> LuaticParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> LuaticParser<'input, I, DefaultErrorStrategy<'input,LuaticParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for LuaticParser
pub trait LuaticParserContext<'input>:
	for<'x> Listenable<dyn LuaticListener<'input> + 'x > + 
	for<'x> Visitable<dyn LuaticVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=LuaticParserContextType>
{}

antlr_rust::coerce_from!{ 'input : LuaticParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn LuaticParserContext<'input> + 'input
where
    T: LuaticVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn LuaticVisitor<'input> + 'x))
    }
}

impl<'input> LuaticParserContext<'input> for TerminalNode<'input,LuaticParserContextType> {}
impl<'input> LuaticParserContext<'input> for ErrorNode<'input,LuaticParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn LuaticParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn LuaticListener<'input> + 'input }

pub struct LuaticParserContextType;
antlr_rust::tid!{LuaticParserContextType}

impl<'input> ParserNodeType<'input> for LuaticParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn LuaticParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct LuaticParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> LuaticParserExt<'input>{
}
antlr_rust::tid! { LuaticParserExt<'a> }

impl<'input> TokenAware<'input> for LuaticParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for LuaticParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for LuaticParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "Luatic.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn LuaticParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					0 => LuaticParser::<'input,I,_>::expr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> LuaticParser<'input, I, DefaultErrorStrategy<'input,LuaticParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expr_sempred(_localctx: Option<&ExprContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 7)
				}
				1=>{
					recog.precpred(None, 5)
				}
				2=>{
					recog.precpred(None, 4)
				}
				3=>{
					recog.precpred(None, 3)
				}
				4=>{
					recog.precpred(None, 2)
				}
				5=>{
					recog.precpred(None, 1)
				}
				6=>{
					recog.precpred(None, 9)
				}
				7=>{
					recog.precpred(None, 8)
				}
			_ => true
		}
	}
}
//------------------- expr ----------------
pub type ExprContextAll<'input> = ExprContext<'input>;


pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for ExprContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expr(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_expr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for ExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_expr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExprContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<ExprContextExt<'input>>{

fn boolean(&self) -> Option<Rc<BooleanContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn integer(&self) -> Option<Rc<IntegerContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn number(&self) -> Option<Rc<NumberContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn string(&self) -> Option<Rc<StringContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn array(&self) -> Option<Rc<ArrayContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn table(&self) -> Option<Rc<TableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lambda(&self) -> Option<Rc<LambdaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token PT_MINUS
/// Returns `None` if there is no child corresponding to token PT_MINUS
fn PT_MINUS(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_MINUS, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_NOT
/// Returns `None` if there is no child corresponding to token KW_NOT
fn KW_NOT(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_NOT, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_POW
/// Returns `None` if there is no child corresponding to token PT_POW
fn PT_POW(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_POW, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_MUL
/// Returns `None` if there is no child corresponding to token PT_MUL
fn PT_MUL(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_MUL, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_DIV
/// Returns `None` if there is no child corresponding to token PT_DIV
fn PT_DIV(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_DIV, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_IDIV
/// Returns `None` if there is no child corresponding to token PT_IDIV
fn PT_IDIV(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_IDIV, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_MOD
/// Returns `None` if there is no child corresponding to token PT_MOD
fn PT_MOD(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_MOD, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_PLUS
/// Returns `None` if there is no child corresponding to token PT_PLUS
fn PT_PLUS(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_PLUS, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_LT
/// Returns `None` if there is no child corresponding to token PT_LT
fn PT_LT(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LT, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_RT
/// Returns `None` if there is no child corresponding to token PT_RT
fn PT_RT(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_RT, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_LE
/// Returns `None` if there is no child corresponding to token PT_LE
fn PT_LE(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LE, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_GE
/// Returns `None` if there is no child corresponding to token PT_GE
fn PT_GE(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_GE, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_EQ
/// Returns `None` if there is no child corresponding to token PT_EQ
fn PT_EQ(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_EQ, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_NE
/// Returns `None` if there is no child corresponding to token PT_NE
fn PT_NE(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_NE, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_AND
/// Returns `None` if there is no child corresponding to token KW_AND
fn KW_AND(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_AND, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_OR
/// Returns `None` if there is no child corresponding to token KW_OR
fn KW_OR(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_OR, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_LS
/// Returns `None` if there is no child corresponding to token PT_LS
fn PT_LS(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LS, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_RS
/// Returns `None` if there is no child corresponding to token PT_RS
fn PT_RS(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_RS, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_LP
/// Returns `None` if there is no child corresponding to token PT_LP
fn PT_LP(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LP, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_RP
/// Returns `None` if there is no child corresponding to token PT_RP
fn PT_RP(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_RP, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token PT_COMMA in current rule
fn PT_COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,LuaticParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PT_COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token PT_COMMA is less or equal than `i`.
fn PT_COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_COMMA, i)
}

}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		self.expr_rec(0)
	}

	fn expr_rec(&mut self, _p: isize)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 0, RULE_expr, _p);
	    let mut _localctx: Rc<ExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 0;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(60);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(0,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule boolean*/
					recog.base.set_state(51);
					recog.boolean()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule integer*/
					recog.base.set_state(52);
					recog.integer()?;

					}
				}
			,
				3 =>{
					{
					/*InvokeRule number*/
					recog.base.set_state(53);
					recog.number()?;

					}
				}
			,
				4 =>{
					{
					/*InvokeRule string*/
					recog.base.set_state(54);
					recog.string()?;

					}
				}
			,
				5 =>{
					{
					/*InvokeRule array*/
					recog.base.set_state(55);
					recog.array()?;

					}
				}
			,
				6 =>{
					{
					/*InvokeRule table*/
					recog.base.set_state(56);
					recog.table()?;

					}
				}
			,
				7 =>{
					{
					/*InvokeRule lambda*/
					recog.base.set_state(57);
					recog.lambda()?;

					}
				}
			,
				8 =>{
					{
					recog.base.set_state(58);
					_la = recog.base.input.la(1);
					if { !(_la==KW_NOT || _la==PT_MINUS) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule expr*/
					recog.base.set_state(59);
					recog.expr_rec(6)?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(100);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(98);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(3,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExprContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(62);
							if !({recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(63);
							recog.base.match_token(PT_POW,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(64);
							recog.expr_rec(7)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExprContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(65);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(66);
							_la = recog.base.input.la(1);
							if { !(((((_la - 29)) & !0x3f) == 0 && ((1usize << (_la - 29)) & ((1usize << (PT_MUL - 29)) | (1usize << (PT_DIV - 29)) | (1usize << (PT_IDIV - 29)) | (1usize << (PT_MOD - 29)))) != 0)) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(67);
							recog.expr_rec(6)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExprContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(68);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(69);
							_la = recog.base.input.la(1);
							if { !(_la==PT_PLUS || _la==PT_MINUS) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(70);
							recog.expr_rec(5)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExprContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(71);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(72);
							_la = recog.base.input.la(1);
							if { !(((((_la - 19)) & !0x3f) == 0 && ((1usize << (_la - 19)) & ((1usize << (PT_LT - 19)) | (1usize << (PT_RT - 19)) | (1usize << (PT_LE - 19)) | (1usize << (PT_GE - 19)) | (1usize << (PT_EQ - 19)) | (1usize << (PT_NE - 19)))) != 0)) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(73);
							recog.expr_rec(4)?;

							}
						}
					,
						5 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExprContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(74);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(75);
							recog.base.match_token(KW_AND,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(76);
							recog.expr_rec(3)?;

							}
						}
					,
						6 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExprContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(77);
							if !({recog.precpred(None, 1)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(78);
							recog.base.match_token(KW_OR,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(79);
							recog.expr_rec(2)?;

							}
						}
					,
						7 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExprContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(80);
							if !({recog.precpred(None, 9)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 9)".to_owned()), None))?;
							}
							recog.base.set_state(81);
							recog.base.match_token(PT_LS,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(82);
							recog.expr_rec(0)?;

							recog.base.set_state(83);
							recog.base.match_token(PT_RS,&mut recog.err_handler)?;

							}
						}
					,
						8 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExprContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(85);
							if !({recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(86);
							recog.base.match_token(PT_LP,&mut recog.err_handler)?;

							recog.base.set_state(95);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << KW_TRUE) | (1usize << KW_FALSE) | (1usize << KW_FUNC) | (1usize << KW_NOT) | (1usize << PT_LB) | (1usize << PT_MINUS))) != 0) || ((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & ((1usize << (NORMALSTRING - 44)) | (1usize << (INT - 44)) | (1usize << (HEX - 44)) | (1usize << (FLOAT - 44)) | (1usize << (HEX_FLOAT - 44)))) != 0) {
								{
								/*InvokeRule expr*/
								recog.base.set_state(87);
								recog.expr_rec(0)?;

								recog.base.set_state(92);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								while _la==PT_COMMA {
									{
									{
									recog.base.set_state(88);
									recog.base.match_token(PT_COMMA,&mut recog.err_handler)?;

									/*InvokeRule expr*/
									recog.base.set_state(89);
									recog.expr_rec(0)?;

									}
									}
									recog.base.set_state(94);
									recog.err_handler.sync(&mut recog.base)?;
									_la = recog.base.input.la(1);
								}
								}
							}

							recog.base.set_state(97);
							recog.base.match_token(PT_RP,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(102);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- stat ----------------
pub type StatContextAll<'input> = StatContext<'input>;


pub type StatContext<'input> = BaseParserRuleContext<'input,StatContextExt<'input>>;

#[derive(Clone)]
pub struct StatContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for StatContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for StatContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stat(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for StatContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_stat(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stat }
}
antlr_rust::tid!{StatContextExt<'a>}

impl<'input> StatContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<StatContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PT_SEMI
/// Returns `None` if there is no child corresponding to token PT_SEMI
fn PT_SEMI(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_SEMI, 0)
}
fn global_stat(&self) -> Option<Rc<Global_statContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn local_stat(&self) -> Option<Rc<Local_statContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn local_in_stat(&self) -> Option<Rc<Local_in_statContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn full_if_stat(&self) -> Option<Rc<Full_if_statContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn while_stat(&self) -> Option<Rc<While_statContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn do_stat(&self) -> Option<Rc<Do_statContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn for_in_stat(&self) -> Option<Rc<For_in_statContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn func_stat(&self) -> Option<Rc<Func_statContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ret_stat(&self) -> Option<Rc<Ret_statContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatContextAttrs<'input> for StatContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stat(&mut self,)
	-> Result<Rc<StatContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_stat);
        let mut _localctx: Rc<StatContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(113);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(5,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(103);
					recog.base.match_token(PT_SEMI,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule global_stat*/
					recog.base.set_state(104);
					recog.global_stat()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule local_stat*/
					recog.base.set_state(105);
					recog.local_stat()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule local_in_stat*/
					recog.base.set_state(106);
					recog.local_in_stat()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule full_if_stat*/
					recog.base.set_state(107);
					recog.full_if_stat()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule while_stat*/
					recog.base.set_state(108);
					recog.while_stat()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule do_stat*/
					recog.base.set_state(109);
					recog.do_stat()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule for_in_stat*/
					recog.base.set_state(110);
					recog.for_in_stat()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule func_stat*/
					recog.base.set_state(111);
					recog.func_stat()?;

					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					/*InvokeRule ret_stat*/
					recog.base.set_state(112);
					recog.ret_stat()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- prgm ----------------
pub type PrgmContextAll<'input> = PrgmContext<'input>;


pub type PrgmContext<'input> = BaseParserRuleContext<'input,PrgmContextExt<'input>>;

#[derive(Clone)]
pub struct PrgmContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for PrgmContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for PrgmContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_prgm(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_prgm(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for PrgmContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_prgm(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrgmContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_prgm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_prgm }
}
antlr_rust::tid!{PrgmContextExt<'a>}

impl<'input> PrgmContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrgmContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrgmContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrgmContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<PrgmContextExt<'input>>{

fn stat_all(&self) ->  Vec<Rc<StatContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stat(&self, i: usize) -> Option<Rc<StatContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PrgmContextAttrs<'input> for PrgmContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn prgm(&mut self,)
	-> Result<Rc<PrgmContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrgmContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_prgm);
        let mut _localctx: Rc<PrgmContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(118);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << KW_GLOBAL) | (1usize << KW_LOCAL) | (1usize << KW_IF) | (1usize << KW_WHILE) | (1usize << KW_FOR) | (1usize << KW_FUNC) | (1usize << KW_DO) | (1usize << KW_RET) | (1usize << PT_SEMI))) != 0) {
				{
				{
				/*InvokeRule stat*/
				recog.base.set_state(115);
				recog.stat()?;

				}
				}
				recog.base.set_state(120);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- block ----------------
pub type BlockContextAll<'input> = BlockContext<'input>;


pub type BlockContext<'input> = BaseParserRuleContext<'input,BlockContextExt<'input>>;

#[derive(Clone)]
pub struct BlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for BlockContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for BlockContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_block(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_block(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for BlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_block(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_block }
	//fn type_rule_index() -> usize where Self: Sized { RULE_block }
}
antlr_rust::tid!{BlockContextExt<'a>}

impl<'input> BlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlockContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<BlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PT_LB
/// Returns `None` if there is no child corresponding to token PT_LB
fn PT_LB(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LB, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_RB
/// Returns `None` if there is no child corresponding to token PT_RB
fn PT_RB(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_RB, 0)
}
fn stat_all(&self) ->  Vec<Rc<StatContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stat(&self, i: usize) -> Option<Rc<StatContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> BlockContextAttrs<'input> for BlockContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn block(&mut self,)
	-> Result<Rc<BlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_block);
        let mut _localctx: Rc<BlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(121);
			recog.base.match_token(PT_LB,&mut recog.err_handler)?;

			recog.base.set_state(125);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << KW_GLOBAL) | (1usize << KW_LOCAL) | (1usize << KW_IF) | (1usize << KW_WHILE) | (1usize << KW_FOR) | (1usize << KW_FUNC) | (1usize << KW_DO) | (1usize << KW_RET) | (1usize << PT_SEMI))) != 0) {
				{
				{
				/*InvokeRule stat*/
				recog.base.set_state(122);
				recog.stat()?;

				}
				}
				recog.base.set_state(127);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(128);
			recog.base.match_token(PT_RB,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- global_stat ----------------
pub type Global_statContextAll<'input> = Global_statContext<'input>;


pub type Global_statContext<'input> = BaseParserRuleContext<'input,Global_statContextExt<'input>>;

#[derive(Clone)]
pub struct Global_statContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Global_statContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Global_statContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_global_stat(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_global_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Global_statContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_global_stat(self);
	}
}

impl<'input> CustomRuleContext<'input> for Global_statContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_global_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_global_stat }
}
antlr_rust::tid!{Global_statContextExt<'a>}

impl<'input> Global_statContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Global_statContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Global_statContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Global_statContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Global_statContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_GLOBAL
/// Returns `None` if there is no child corresponding to token KW_GLOBAL
fn KW_GLOBAL(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_GLOBAL, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_ASGN
/// Returns `None` if there is no child corresponding to token PT_ASGN
fn PT_ASGN(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_ASGN, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PT_SEMI
/// Returns `None` if there is no child corresponding to token PT_SEMI
fn PT_SEMI(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_SEMI, 0)
}

}

impl<'input> Global_statContextAttrs<'input> for Global_statContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn global_stat(&mut self,)
	-> Result<Rc<Global_statContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Global_statContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_global_stat);
        let mut _localctx: Rc<Global_statContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(130);
			recog.base.match_token(KW_GLOBAL,&mut recog.err_handler)?;

			recog.base.set_state(131);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(132);
			recog.base.match_token(PT_ASGN,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(133);
			recog.expr_rec(0)?;

			recog.base.set_state(134);
			recog.base.match_token(PT_SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- local_stat ----------------
pub type Local_statContextAll<'input> = Local_statContext<'input>;


pub type Local_statContext<'input> = BaseParserRuleContext<'input,Local_statContextExt<'input>>;

#[derive(Clone)]
pub struct Local_statContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Local_statContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Local_statContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_local_stat(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_local_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Local_statContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_local_stat(self);
	}
}

impl<'input> CustomRuleContext<'input> for Local_statContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_local_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_local_stat }
}
antlr_rust::tid!{Local_statContextExt<'a>}

impl<'input> Local_statContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Local_statContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Local_statContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Local_statContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Local_statContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_LOCAL
/// Returns `None` if there is no child corresponding to token KW_LOCAL
fn KW_LOCAL(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_LOCAL, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_ASGN
/// Returns `None` if there is no child corresponding to token PT_ASGN
fn PT_ASGN(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_ASGN, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PT_SEMI
/// Returns `None` if there is no child corresponding to token PT_SEMI
fn PT_SEMI(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_SEMI, 0)
}

}

impl<'input> Local_statContextAttrs<'input> for Local_statContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn local_stat(&mut self,)
	-> Result<Rc<Local_statContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Local_statContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_local_stat);
        let mut _localctx: Rc<Local_statContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(136);
			recog.base.match_token(KW_LOCAL,&mut recog.err_handler)?;

			recog.base.set_state(137);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(138);
			recog.base.match_token(PT_ASGN,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(139);
			recog.expr_rec(0)?;

			recog.base.set_state(140);
			recog.base.match_token(PT_SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- local_in_stat ----------------
pub type Local_in_statContextAll<'input> = Local_in_statContext<'input>;


pub type Local_in_statContext<'input> = BaseParserRuleContext<'input,Local_in_statContextExt<'input>>;

#[derive(Clone)]
pub struct Local_in_statContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Local_in_statContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Local_in_statContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_local_in_stat(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_local_in_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Local_in_statContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_local_in_stat(self);
	}
}

impl<'input> CustomRuleContext<'input> for Local_in_statContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_local_in_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_local_in_stat }
}
antlr_rust::tid!{Local_in_statContextExt<'a>}

impl<'input> Local_in_statContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Local_in_statContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Local_in_statContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Local_in_statContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Local_in_statContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_LOCAL
/// Returns `None` if there is no child corresponding to token KW_LOCAL
fn KW_LOCAL(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_LOCAL, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_ASGN
/// Returns `None` if there is no child corresponding to token PT_ASGN
fn PT_ASGN(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_ASGN, 0)
}
fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token KW_IN
/// Returns `None` if there is no child corresponding to token KW_IN
fn KW_IN(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_IN, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_SEMI
/// Returns `None` if there is no child corresponding to token PT_SEMI
fn PT_SEMI(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_SEMI, 0)
}

}

impl<'input> Local_in_statContextAttrs<'input> for Local_in_statContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn local_in_stat(&mut self,)
	-> Result<Rc<Local_in_statContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Local_in_statContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_local_in_stat);
        let mut _localctx: Rc<Local_in_statContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(142);
			recog.base.match_token(KW_LOCAL,&mut recog.err_handler)?;

			recog.base.set_state(143);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(144);
			recog.base.match_token(PT_ASGN,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(145);
			recog.expr_rec(0)?;

			recog.base.set_state(146);
			recog.base.match_token(KW_IN,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(147);
			recog.expr_rec(0)?;

			recog.base.set_state(148);
			recog.base.match_token(PT_SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- full_if_stat ----------------
pub type Full_if_statContextAll<'input> = Full_if_statContext<'input>;


pub type Full_if_statContext<'input> = BaseParserRuleContext<'input,Full_if_statContextExt<'input>>;

#[derive(Clone)]
pub struct Full_if_statContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Full_if_statContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Full_if_statContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_full_if_stat(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_full_if_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Full_if_statContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_full_if_stat(self);
	}
}

impl<'input> CustomRuleContext<'input> for Full_if_statContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_full_if_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_full_if_stat }
}
antlr_rust::tid!{Full_if_statContextExt<'a>}

impl<'input> Full_if_statContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Full_if_statContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Full_if_statContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Full_if_statContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Full_if_statContextExt<'input>>{

fn if_stat(&self) -> Option<Rc<If_statContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn elif_stat_all(&self) ->  Vec<Rc<Elif_statContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn elif_stat(&self, i: usize) -> Option<Rc<Elif_statContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn else_state_all(&self) ->  Vec<Rc<Else_stateContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn else_state(&self, i: usize) -> Option<Rc<Else_stateContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Full_if_statContextAttrs<'input> for Full_if_statContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn full_if_stat(&mut self,)
	-> Result<Rc<Full_if_statContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Full_if_statContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_full_if_stat);
        let mut _localctx: Rc<Full_if_statContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule if_stat*/
			recog.base.set_state(150);
			recog.if_stat()?;

			recog.base.set_state(154);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(8,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule elif_stat*/
					recog.base.set_state(151);
					recog.elif_stat()?;

					}
					} 
				}
				recog.base.set_state(156);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(8,&mut recog.base)?;
			}
			recog.base.set_state(158); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule else_state*/
				recog.base.set_state(157);
				recog.else_state()?;

				}
				}
				recog.base.set_state(160); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==KW_ELSE) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- while_stat ----------------
pub type While_statContextAll<'input> = While_statContext<'input>;


pub type While_statContext<'input> = BaseParserRuleContext<'input,While_statContextExt<'input>>;

#[derive(Clone)]
pub struct While_statContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for While_statContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for While_statContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_while_stat(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_while_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for While_statContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_while_stat(self);
	}
}

impl<'input> CustomRuleContext<'input> for While_statContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_while_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_while_stat }
}
antlr_rust::tid!{While_statContextExt<'a>}

impl<'input> While_statContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<While_statContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,While_statContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait While_statContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<While_statContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_WHILE
/// Returns `None` if there is no child corresponding to token KW_WHILE
fn KW_WHILE(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_WHILE, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_LP
/// Returns `None` if there is no child corresponding to token PT_LP
fn PT_LP(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LP, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PT_RP
/// Returns `None` if there is no child corresponding to token PT_RP
fn PT_RP(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_RP, 0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> While_statContextAttrs<'input> for While_statContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn while_stat(&mut self,)
	-> Result<Rc<While_statContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = While_statContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_while_stat);
        let mut _localctx: Rc<While_statContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(162);
			recog.base.match_token(KW_WHILE,&mut recog.err_handler)?;

			recog.base.set_state(163);
			recog.base.match_token(PT_LP,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(164);
			recog.expr_rec(0)?;

			recog.base.set_state(165);
			recog.base.match_token(PT_RP,&mut recog.err_handler)?;

			/*InvokeRule block*/
			recog.base.set_state(166);
			recog.block()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- do_stat ----------------
pub type Do_statContextAll<'input> = Do_statContext<'input>;


pub type Do_statContext<'input> = BaseParserRuleContext<'input,Do_statContextExt<'input>>;

#[derive(Clone)]
pub struct Do_statContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Do_statContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Do_statContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_do_stat(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_do_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Do_statContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_do_stat(self);
	}
}

impl<'input> CustomRuleContext<'input> for Do_statContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_do_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_do_stat }
}
antlr_rust::tid!{Do_statContextExt<'a>}

impl<'input> Do_statContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Do_statContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Do_statContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Do_statContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Do_statContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_DO
/// Returns `None` if there is no child corresponding to token KW_DO
fn KW_DO(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_DO, 0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token KW_WHILE
/// Returns `None` if there is no child corresponding to token KW_WHILE
fn KW_WHILE(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_WHILE, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_LP
/// Returns `None` if there is no child corresponding to token PT_LP
fn PT_LP(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LP, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PT_RP
/// Returns `None` if there is no child corresponding to token PT_RP
fn PT_RP(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_RP, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_SEMI
/// Returns `None` if there is no child corresponding to token PT_SEMI
fn PT_SEMI(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_SEMI, 0)
}

}

impl<'input> Do_statContextAttrs<'input> for Do_statContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn do_stat(&mut self,)
	-> Result<Rc<Do_statContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Do_statContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_do_stat);
        let mut _localctx: Rc<Do_statContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(168);
			recog.base.match_token(KW_DO,&mut recog.err_handler)?;

			/*InvokeRule block*/
			recog.base.set_state(169);
			recog.block()?;

			recog.base.set_state(170);
			recog.base.match_token(KW_WHILE,&mut recog.err_handler)?;

			recog.base.set_state(171);
			recog.base.match_token(PT_LP,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(172);
			recog.expr_rec(0)?;

			recog.base.set_state(173);
			recog.base.match_token(PT_RP,&mut recog.err_handler)?;

			recog.base.set_state(174);
			recog.base.match_token(PT_SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- for_in_stat ----------------
pub type For_in_statContextAll<'input> = For_in_statContext<'input>;


pub type For_in_statContext<'input> = BaseParserRuleContext<'input,For_in_statContextExt<'input>>;

#[derive(Clone)]
pub struct For_in_statContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for For_in_statContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for For_in_statContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_for_in_stat(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_for_in_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for For_in_statContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_for_in_stat(self);
	}
}

impl<'input> CustomRuleContext<'input> for For_in_statContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_for_in_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_for_in_stat }
}
antlr_rust::tid!{For_in_statContextExt<'a>}

impl<'input> For_in_statContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<For_in_statContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,For_in_statContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait For_in_statContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<For_in_statContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_FOR
/// Returns `None` if there is no child corresponding to token KW_FOR
fn KW_FOR(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_FOR, 0)
}
fn param_list(&self) -> Option<Rc<Param_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token KW_IN
/// Returns `None` if there is no child corresponding to token KW_IN
fn KW_IN(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_IN, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> For_in_statContextAttrs<'input> for For_in_statContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn for_in_stat(&mut self,)
	-> Result<Rc<For_in_statContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = For_in_statContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_for_in_stat);
        let mut _localctx: Rc<For_in_statContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(176);
			recog.base.match_token(KW_FOR,&mut recog.err_handler)?;

			/*InvokeRule param_list*/
			recog.base.set_state(177);
			recog.param_list()?;

			recog.base.set_state(178);
			recog.base.match_token(KW_IN,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(179);
			recog.expr_rec(0)?;

			/*InvokeRule block*/
			recog.base.set_state(180);
			recog.block()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- func_stat ----------------
pub type Func_statContextAll<'input> = Func_statContext<'input>;


pub type Func_statContext<'input> = BaseParserRuleContext<'input,Func_statContextExt<'input>>;

#[derive(Clone)]
pub struct Func_statContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Func_statContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Func_statContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_func_stat(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_func_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Func_statContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_func_stat(self);
	}
}

impl<'input> CustomRuleContext<'input> for Func_statContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_stat }
}
antlr_rust::tid!{Func_statContextExt<'a>}

impl<'input> Func_statContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Func_statContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_statContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Func_statContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Func_statContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_FUNC
/// Returns `None` if there is no child corresponding to token KW_FUNC
fn KW_FUNC(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_FUNC, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_LP
/// Returns `None` if there is no child corresponding to token PT_LP
fn PT_LP(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LP, 0)
}
fn param_list(&self) -> Option<Rc<Param_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PT_RP
/// Returns `None` if there is no child corresponding to token PT_RP
fn PT_RP(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_RP, 0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Func_statContextAttrs<'input> for Func_statContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn func_stat(&mut self,)
	-> Result<Rc<Func_statContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_statContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_func_stat);
        let mut _localctx: Rc<Func_statContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(182);
			recog.base.match_token(KW_FUNC,&mut recog.err_handler)?;

			recog.base.set_state(183);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(184);
			recog.base.match_token(PT_LP,&mut recog.err_handler)?;

			/*InvokeRule param_list*/
			recog.base.set_state(185);
			recog.param_list()?;

			recog.base.set_state(186);
			recog.base.match_token(PT_RP,&mut recog.err_handler)?;

			/*InvokeRule block*/
			recog.base.set_state(187);
			recog.block()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- ret_stat ----------------
pub type Ret_statContextAll<'input> = Ret_statContext<'input>;


pub type Ret_statContext<'input> = BaseParserRuleContext<'input,Ret_statContextExt<'input>>;

#[derive(Clone)]
pub struct Ret_statContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Ret_statContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Ret_statContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_ret_stat(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_ret_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Ret_statContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_ret_stat(self);
	}
}

impl<'input> CustomRuleContext<'input> for Ret_statContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ret_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ret_stat }
}
antlr_rust::tid!{Ret_statContextExt<'a>}

impl<'input> Ret_statContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Ret_statContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Ret_statContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Ret_statContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Ret_statContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_RET
/// Returns `None` if there is no child corresponding to token KW_RET
fn KW_RET(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_RET, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PT_SEMI
/// Returns `None` if there is no child corresponding to token PT_SEMI
fn PT_SEMI(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_SEMI, 0)
}

}

impl<'input> Ret_statContextAttrs<'input> for Ret_statContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ret_stat(&mut self,)
	-> Result<Rc<Ret_statContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Ret_statContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_ret_stat);
        let mut _localctx: Rc<Ret_statContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(189);
			recog.base.match_token(KW_RET,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(190);
			recog.expr_rec(0)?;

			recog.base.set_state(191);
			recog.base.match_token(PT_SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- boolean ----------------
pub type BooleanContextAll<'input> = BooleanContext<'input>;


pub type BooleanContext<'input> = BaseParserRuleContext<'input,BooleanContextExt<'input>>;

#[derive(Clone)]
pub struct BooleanContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for BooleanContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for BooleanContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_boolean(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_boolean(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for BooleanContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_boolean(self);
	}
}

impl<'input> CustomRuleContext<'input> for BooleanContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_boolean }
	//fn type_rule_index() -> usize where Self: Sized { RULE_boolean }
}
antlr_rust::tid!{BooleanContextExt<'a>}

impl<'input> BooleanContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BooleanContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BooleanContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BooleanContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<BooleanContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_TRUE
/// Returns `None` if there is no child corresponding to token KW_TRUE
fn KW_TRUE(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_TRUE, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_FALSE
/// Returns `None` if there is no child corresponding to token KW_FALSE
fn KW_FALSE(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_FALSE, 0)
}

}

impl<'input> BooleanContextAttrs<'input> for BooleanContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn boolean(&mut self,)
	-> Result<Rc<BooleanContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BooleanContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_boolean);
        let mut _localctx: Rc<BooleanContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(193);
			_la = recog.base.input.la(1);
			if { !(_la==KW_TRUE || _la==KW_FALSE) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- integer ----------------
pub type IntegerContextAll<'input> = IntegerContext<'input>;


pub type IntegerContext<'input> = BaseParserRuleContext<'input,IntegerContextExt<'input>>;

#[derive(Clone)]
pub struct IntegerContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for IntegerContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for IntegerContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_integer(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_integer(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for IntegerContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_integer(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntegerContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_integer }
	//fn type_rule_index() -> usize where Self: Sized { RULE_integer }
}
antlr_rust::tid!{IntegerContextExt<'a>}

impl<'input> IntegerContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IntegerContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IntegerContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IntegerContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<IntegerContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INT
/// Returns `None` if there is no child corresponding to token INT
fn INT(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(INT, 0)
}
/// Retrieves first TerminalNode corresponding to token HEX
/// Returns `None` if there is no child corresponding to token HEX
fn HEX(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(HEX, 0)
}

}

impl<'input> IntegerContextAttrs<'input> for IntegerContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn integer(&mut self,)
	-> Result<Rc<IntegerContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntegerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_integer);
        let mut _localctx: Rc<IntegerContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(195);
			_la = recog.base.input.la(1);
			if { !(_la==INT || _la==HEX) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- number ----------------
pub type NumberContextAll<'input> = NumberContext<'input>;


pub type NumberContext<'input> = BaseParserRuleContext<'input,NumberContextExt<'input>>;

#[derive(Clone)]
pub struct NumberContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for NumberContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for NumberContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_number(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_number(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for NumberContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_number(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_number }
	//fn type_rule_index() -> usize where Self: Sized { RULE_number }
}
antlr_rust::tid!{NumberContextExt<'a>}

impl<'input> NumberContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumberContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumberContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NumberContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<NumberContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FLOAT
/// Returns `None` if there is no child corresponding to token FLOAT
fn FLOAT(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(FLOAT, 0)
}
/// Retrieves first TerminalNode corresponding to token HEX_FLOAT
/// Returns `None` if there is no child corresponding to token HEX_FLOAT
fn HEX_FLOAT(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(HEX_FLOAT, 0)
}

}

impl<'input> NumberContextAttrs<'input> for NumberContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn number(&mut self,)
	-> Result<Rc<NumberContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_number);
        let mut _localctx: Rc<NumberContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(197);
			_la = recog.base.input.la(1);
			if { !(_la==FLOAT || _la==HEX_FLOAT) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- string ----------------
pub type StringContextAll<'input> = StringContext<'input>;


pub type StringContext<'input> = BaseParserRuleContext<'input,StringContextExt<'input>>;

#[derive(Clone)]
pub struct StringContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for StringContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for StringContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_string(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_string(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for StringContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_string(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_string }
	//fn type_rule_index() -> usize where Self: Sized { RULE_string }
}
antlr_rust::tid!{StringContextExt<'a>}

impl<'input> StringContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StringContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StringContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<StringContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NORMALSTRING
/// Returns `None` if there is no child corresponding to token NORMALSTRING
fn NORMALSTRING(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(NORMALSTRING, 0)
}

}

impl<'input> StringContextAttrs<'input> for StringContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn string(&mut self,)
	-> Result<Rc<StringContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_string);
        let mut _localctx: Rc<StringContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(199);
			recog.base.match_token(NORMALSTRING,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- if_stat ----------------
pub type If_statContextAll<'input> = If_statContext<'input>;


pub type If_statContext<'input> = BaseParserRuleContext<'input,If_statContextExt<'input>>;

#[derive(Clone)]
pub struct If_statContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for If_statContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for If_statContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_if_stat(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_if_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for If_statContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_if_stat(self);
	}
}

impl<'input> CustomRuleContext<'input> for If_statContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_if_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_if_stat }
}
antlr_rust::tid!{If_statContextExt<'a>}

impl<'input> If_statContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<If_statContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,If_statContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait If_statContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<If_statContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_IF
/// Returns `None` if there is no child corresponding to token KW_IF
fn KW_IF(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_IF, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_LP
/// Returns `None` if there is no child corresponding to token PT_LP
fn PT_LP(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LP, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PT_RP
/// Returns `None` if there is no child corresponding to token PT_RP
fn PT_RP(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_RP, 0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> If_statContextAttrs<'input> for If_statContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn if_stat(&mut self,)
	-> Result<Rc<If_statContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = If_statContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_if_stat);
        let mut _localctx: Rc<If_statContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(201);
			recog.base.match_token(KW_IF,&mut recog.err_handler)?;

			recog.base.set_state(202);
			recog.base.match_token(PT_LP,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(203);
			recog.expr_rec(0)?;

			recog.base.set_state(204);
			recog.base.match_token(PT_RP,&mut recog.err_handler)?;

			/*InvokeRule block*/
			recog.base.set_state(205);
			recog.block()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- elif_stat ----------------
pub type Elif_statContextAll<'input> = Elif_statContext<'input>;


pub type Elif_statContext<'input> = BaseParserRuleContext<'input,Elif_statContextExt<'input>>;

#[derive(Clone)]
pub struct Elif_statContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Elif_statContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Elif_statContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_elif_stat(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_elif_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Elif_statContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_elif_stat(self);
	}
}

impl<'input> CustomRuleContext<'input> for Elif_statContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_elif_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_elif_stat }
}
antlr_rust::tid!{Elif_statContextExt<'a>}

impl<'input> Elif_statContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Elif_statContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Elif_statContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Elif_statContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Elif_statContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_ELSE
/// Returns `None` if there is no child corresponding to token KW_ELSE
fn KW_ELSE(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_ELSE, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_IF
/// Returns `None` if there is no child corresponding to token KW_IF
fn KW_IF(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_IF, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_LP
/// Returns `None` if there is no child corresponding to token PT_LP
fn PT_LP(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LP, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PT_RP
/// Returns `None` if there is no child corresponding to token PT_RP
fn PT_RP(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_RP, 0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Elif_statContextAttrs<'input> for Elif_statContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn elif_stat(&mut self,)
	-> Result<Rc<Elif_statContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Elif_statContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_elif_stat);
        let mut _localctx: Rc<Elif_statContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(207);
			recog.base.match_token(KW_ELSE,&mut recog.err_handler)?;

			recog.base.set_state(208);
			recog.base.match_token(KW_IF,&mut recog.err_handler)?;

			recog.base.set_state(209);
			recog.base.match_token(PT_LP,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(210);
			recog.expr_rec(0)?;

			recog.base.set_state(211);
			recog.base.match_token(PT_RP,&mut recog.err_handler)?;

			/*InvokeRule block*/
			recog.base.set_state(212);
			recog.block()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- else_state ----------------
pub type Else_stateContextAll<'input> = Else_stateContext<'input>;


pub type Else_stateContext<'input> = BaseParserRuleContext<'input,Else_stateContextExt<'input>>;

#[derive(Clone)]
pub struct Else_stateContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Else_stateContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Else_stateContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_else_state(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_else_state(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Else_stateContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_else_state(self);
	}
}

impl<'input> CustomRuleContext<'input> for Else_stateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_else_state }
	//fn type_rule_index() -> usize where Self: Sized { RULE_else_state }
}
antlr_rust::tid!{Else_stateContextExt<'a>}

impl<'input> Else_stateContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Else_stateContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Else_stateContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Else_stateContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Else_stateContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_ELSE
/// Returns `None` if there is no child corresponding to token KW_ELSE
fn KW_ELSE(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_ELSE, 0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Else_stateContextAttrs<'input> for Else_stateContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn else_state(&mut self,)
	-> Result<Rc<Else_stateContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Else_stateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_else_state);
        let mut _localctx: Rc<Else_stateContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(214);
			recog.base.match_token(KW_ELSE,&mut recog.err_handler)?;

			/*InvokeRule block*/
			recog.base.set_state(215);
			recog.block()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- param_list ----------------
pub type Param_listContextAll<'input> = Param_listContext<'input>;


pub type Param_listContext<'input> = BaseParserRuleContext<'input,Param_listContextExt<'input>>;

#[derive(Clone)]
pub struct Param_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Param_listContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Param_listContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_param_list(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_param_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Param_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_param_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Param_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_param_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_param_list }
}
antlr_rust::tid!{Param_listContextExt<'a>}

impl<'input> Param_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Param_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Param_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Param_listContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Param_listContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENT in current rule
fn IDENT_all(&self) -> Vec<Rc<TerminalNode<'input,LuaticParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENT, starting from 0.
/// Returns `None` if number of children corresponding to token IDENT is less or equal than `i`.
fn IDENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(IDENT, i)
}
/// Retrieves all `TerminalNode`s corresponding to token PT_COMMA in current rule
fn PT_COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,LuaticParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PT_COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token PT_COMMA is less or equal than `i`.
fn PT_COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_COMMA, i)
}

}

impl<'input> Param_listContextAttrs<'input> for Param_listContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn param_list(&mut self,)
	-> Result<Rc<Param_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Param_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_param_list);
        let mut _localctx: Rc<Param_listContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(225);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IDENT {
				{
				recog.base.set_state(217);
				recog.base.match_token(IDENT,&mut recog.err_handler)?;

				recog.base.set_state(222);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==PT_COMMA {
					{
					{
					recog.base.set_state(218);
					recog.base.match_token(PT_COMMA,&mut recog.err_handler)?;

					recog.base.set_state(219);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					}
					}
					recog.base.set_state(224);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- array ----------------
pub type ArrayContextAll<'input> = ArrayContext<'input>;


pub type ArrayContext<'input> = BaseParserRuleContext<'input,ArrayContextExt<'input>>;

#[derive(Clone)]
pub struct ArrayContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for ArrayContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for ArrayContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_array(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_array(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for ArrayContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_array(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_array }
	//fn type_rule_index() -> usize where Self: Sized { RULE_array }
}
antlr_rust::tid!{ArrayContextExt<'a>}

impl<'input> ArrayContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArrayContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArrayContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArrayContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<ArrayContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PT_LB
/// Returns `None` if there is no child corresponding to token PT_LB
fn PT_LB(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LB, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_RB
/// Returns `None` if there is no child corresponding to token PT_RB
fn PT_RB(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_RB, 0)
}
fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token PT_COMMA in current rule
fn PT_COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,LuaticParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PT_COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token PT_COMMA is less or equal than `i`.
fn PT_COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_COMMA, i)
}

}

impl<'input> ArrayContextAttrs<'input> for ArrayContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn array(&mut self,)
	-> Result<Rc<ArrayContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArrayContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_array);
        let mut _localctx: Rc<ArrayContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(227);
			recog.base.match_token(PT_LB,&mut recog.err_handler)?;

			recog.base.set_state(239);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << KW_TRUE) | (1usize << KW_FALSE) | (1usize << KW_FUNC) | (1usize << KW_NOT) | (1usize << PT_LB) | (1usize << PT_MINUS))) != 0) || ((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & ((1usize << (NORMALSTRING - 44)) | (1usize << (INT - 44)) | (1usize << (HEX - 44)) | (1usize << (FLOAT - 44)) | (1usize << (HEX_FLOAT - 44)))) != 0) {
				{
				/*InvokeRule expr*/
				recog.base.set_state(228);
				recog.expr_rec(0)?;

				recog.base.set_state(233);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(229);
						recog.base.match_token(PT_COMMA,&mut recog.err_handler)?;

						/*InvokeRule expr*/
						recog.base.set_state(230);
						recog.expr_rec(0)?;

						}
						} 
					}
					recog.base.set_state(235);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
				}
				recog.base.set_state(237);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==PT_COMMA {
					{
					recog.base.set_state(236);
					recog.base.match_token(PT_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(241);
			recog.base.match_token(PT_RB,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- kv_pair ----------------
pub type Kv_pairContextAll<'input> = Kv_pairContext<'input>;


pub type Kv_pairContext<'input> = BaseParserRuleContext<'input,Kv_pairContextExt<'input>>;

#[derive(Clone)]
pub struct Kv_pairContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Kv_pairContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Kv_pairContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_kv_pair(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_kv_pair(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Kv_pairContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_kv_pair(self);
	}
}

impl<'input> CustomRuleContext<'input> for Kv_pairContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_kv_pair }
	//fn type_rule_index() -> usize where Self: Sized { RULE_kv_pair }
}
antlr_rust::tid!{Kv_pairContextExt<'a>}

impl<'input> Kv_pairContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Kv_pairContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Kv_pairContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Kv_pairContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Kv_pairContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PT_LS
/// Returns `None` if there is no child corresponding to token PT_LS
fn PT_LS(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LS, 0)
}
fn string(&self) -> Option<Rc<StringContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PT_RS
/// Returns `None` if there is no child corresponding to token PT_RS
fn PT_RS(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_RS, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_ASGN
/// Returns `None` if there is no child corresponding to token PT_ASGN
fn PT_ASGN(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_ASGN, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Kv_pairContextAttrs<'input> for Kv_pairContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn kv_pair(&mut self,)
	-> Result<Rc<Kv_pairContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Kv_pairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_kv_pair);
        let mut _localctx: Rc<Kv_pairContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(243);
			recog.base.match_token(PT_LS,&mut recog.err_handler)?;

			/*InvokeRule string*/
			recog.base.set_state(244);
			recog.string()?;

			recog.base.set_state(245);
			recog.base.match_token(PT_RS,&mut recog.err_handler)?;

			recog.base.set_state(246);
			recog.base.match_token(PT_ASGN,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(247);
			recog.expr_rec(0)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- table ----------------
pub type TableContextAll<'input> = TableContext<'input>;


pub type TableContext<'input> = BaseParserRuleContext<'input,TableContextExt<'input>>;

#[derive(Clone)]
pub struct TableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for TableContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for TableContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_table(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_table(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for TableContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_table(self);
	}
}

impl<'input> CustomRuleContext<'input> for TableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_table }
	//fn type_rule_index() -> usize where Self: Sized { RULE_table }
}
antlr_rust::tid!{TableContextExt<'a>}

impl<'input> TableContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TableContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TableContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<TableContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PT_LB
/// Returns `None` if there is no child corresponding to token PT_LB
fn PT_LB(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LB, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_RB
/// Returns `None` if there is no child corresponding to token PT_RB
fn PT_RB(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_RB, 0)
}
fn kv_pair_all(&self) ->  Vec<Rc<Kv_pairContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn kv_pair(&self, i: usize) -> Option<Rc<Kv_pairContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token PT_COMMA in current rule
fn PT_COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,LuaticParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PT_COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token PT_COMMA is less or equal than `i`.
fn PT_COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_COMMA, i)
}

}

impl<'input> TableContextAttrs<'input> for TableContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn table(&mut self,)
	-> Result<Rc<TableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_table);
        let mut _localctx: Rc<TableContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(249);
			recog.base.match_token(PT_LB,&mut recog.err_handler)?;

			recog.base.set_state(261);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==PT_LS {
				{
				/*InvokeRule kv_pair*/
				recog.base.set_state(250);
				recog.kv_pair()?;

				recog.base.set_state(255);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(251);
						recog.base.match_token(PT_COMMA,&mut recog.err_handler)?;

						/*InvokeRule kv_pair*/
						recog.base.set_state(252);
						recog.kv_pair()?;

						}
						} 
					}
					recog.base.set_state(257);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
				}
				recog.base.set_state(259);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==PT_COMMA {
					{
					recog.base.set_state(258);
					recog.base.match_token(PT_COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(263);
			recog.base.match_token(PT_RB,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- lambda ----------------
pub type LambdaContextAll<'input> = LambdaContext<'input>;


pub type LambdaContext<'input> = BaseParserRuleContext<'input,LambdaContextExt<'input>>;

#[derive(Clone)]
pub struct LambdaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for LambdaContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for LambdaContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lambda(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_lambda(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for LambdaContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_lambda(self);
	}
}

impl<'input> CustomRuleContext<'input> for LambdaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lambda }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lambda }
}
antlr_rust::tid!{LambdaContextExt<'a>}

impl<'input> LambdaContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LambdaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LambdaContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LambdaContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<LambdaContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_FUNC
/// Returns `None` if there is no child corresponding to token KW_FUNC
fn KW_FUNC(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_FUNC, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_LP
/// Returns `None` if there is no child corresponding to token PT_LP
fn PT_LP(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LP, 0)
}
fn param_list(&self) -> Option<Rc<Param_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PT_RP
/// Returns `None` if there is no child corresponding to token PT_RP
fn PT_RP(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_RP, 0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PT_SEMI
/// Returns `None` if there is no child corresponding to token PT_SEMI
fn PT_SEMI(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_SEMI, 0)
}

}

impl<'input> LambdaContextAttrs<'input> for LambdaContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lambda(&mut self,)
	-> Result<Rc<LambdaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LambdaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_lambda);
        let mut _localctx: Rc<LambdaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(265);
			recog.base.match_token(KW_FUNC,&mut recog.err_handler)?;

			recog.base.set_state(266);
			recog.base.match_token(PT_LP,&mut recog.err_handler)?;

			/*InvokeRule param_list*/
			recog.base.set_state(267);
			recog.param_list()?;

			recog.base.set_state(268);
			recog.base.match_token(PT_RP,&mut recog.err_handler)?;

			/*InvokeRule block*/
			recog.base.set_state(269);
			recog.block()?;

			recog.base.set_state(270);
			recog.base.match_token(PT_SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x35\u{113}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x03\x02\x03\x02\x03\
	\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x05\x02\x3f\
	\x0a\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\
	\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\
	\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\
	\x03\x02\x03\x02\x07\x02\x5d\x0a\x02\x0c\x02\x0e\x02\x60\x0b\x02\x05\x02\
	\x62\x0a\x02\x03\x02\x07\x02\x65\x0a\x02\x0c\x02\x0e\x02\x68\x0b\x02\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\
	\x03\x05\x03\x74\x0a\x03\x03\x04\x07\x04\x77\x0a\x04\x0c\x04\x0e\x04\x7a\
	\x0b\x04\x03\x05\x03\x05\x07\x05\x7e\x0a\x05\x0c\x05\x0e\x05\u{81}\x0b\x05\
	\x03\x05\x03\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x07\
	\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x08\x03\x08\x03\x08\x03\x08\
	\x03\x08\x03\x08\x03\x08\x03\x08\x03\x09\x03\x09\x07\x09\u{9b}\x0a\x09\x0c\
	\x09\x0e\x09\u{9e}\x0b\x09\x03\x09\x06\x09\u{a1}\x0a\x09\x0d\x09\x0e\x09\
	\u{a2}\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\
	\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\
	\x0c\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\
	\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x10\x03\x10\x03\
	\x11\x03\x11\x03\x12\x03\x12\x03\x13\x03\x13\x03\x13\x03\x13\x03\x13\x03\
	\x13\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x15\x03\
	\x15\x03\x15\x03\x16\x03\x16\x03\x16\x07\x16\u{df}\x0a\x16\x0c\x16\x0e\x16\
	\u{e2}\x0b\x16\x05\x16\u{e4}\x0a\x16\x03\x17\x03\x17\x03\x17\x03\x17\x07\
	\x17\u{ea}\x0a\x17\x0c\x17\x0e\x17\u{ed}\x0b\x17\x03\x17\x05\x17\u{f0}\x0a\
	\x17\x05\x17\u{f2}\x0a\x17\x03\x17\x03\x17\x03\x18\x03\x18\x03\x18\x03\x18\
	\x03\x18\x03\x18\x03\x19\x03\x19\x03\x19\x03\x19\x07\x19\u{100}\x0a\x19\
	\x0c\x19\x0e\x19\u{103}\x0b\x19\x03\x19\x05\x19\u{106}\x0a\x19\x05\x19\u{108}\
	\x0a\x19\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\
	\x03\x1a\x03\x1a\x02\x03\x02\x1b\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\
	\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x02\x09\x04\
	\x02\x11\x11\x1e\x1e\x03\x02\x1f\x22\x03\x02\x1d\x1e\x04\x02\x15\x16\x23\
	\x26\x03\x02\x04\x05\x03\x02\x2f\x30\x03\x02\x31\x32\x02\u{11f}\x02\x3e\
	\x03\x02\x02\x02\x04\x73\x03\x02\x02\x02\x06\x78\x03\x02\x02\x02\x08\x7b\
	\x03\x02\x02\x02\x0a\u{84}\x03\x02\x02\x02\x0c\u{8a}\x03\x02\x02\x02\x0e\
	\u{90}\x03\x02\x02\x02\x10\u{98}\x03\x02\x02\x02\x12\u{a4}\x03\x02\x02\x02\
	\x14\u{aa}\x03\x02\x02\x02\x16\u{b2}\x03\x02\x02\x02\x18\u{b8}\x03\x02\x02\
	\x02\x1a\u{bf}\x03\x02\x02\x02\x1c\u{c3}\x03\x02\x02\x02\x1e\u{c5}\x03\x02\
	\x02\x02\x20\u{c7}\x03\x02\x02\x02\x22\u{c9}\x03\x02\x02\x02\x24\u{cb}\x03\
	\x02\x02\x02\x26\u{d1}\x03\x02\x02\x02\x28\u{d8}\x03\x02\x02\x02\x2a\u{e3}\
	\x03\x02\x02\x02\x2c\u{e5}\x03\x02\x02\x02\x2e\u{f5}\x03\x02\x02\x02\x30\
	\u{fb}\x03\x02\x02\x02\x32\u{10b}\x03\x02\x02\x02\x34\x35\x08\x02\x01\x02\
	\x35\x3f\x05\x1c\x0f\x02\x36\x3f\x05\x1e\x10\x02\x37\x3f\x05\x20\x11\x02\
	\x38\x3f\x05\x22\x12\x02\x39\x3f\x05\x2c\x17\x02\x3a\x3f\x05\x30\x19\x02\
	\x3b\x3f\x05\x32\x1a\x02\x3c\x3d\x09\x02\x02\x02\x3d\x3f\x05\x02\x02\x08\
	\x3e\x34\x03\x02\x02\x02\x3e\x36\x03\x02\x02\x02\x3e\x37\x03\x02\x02\x02\
	\x3e\x38\x03\x02\x02\x02\x3e\x39\x03\x02\x02\x02\x3e\x3a\x03\x02\x02\x02\
	\x3e\x3b\x03\x02\x02\x02\x3e\x3c\x03\x02\x02\x02\x3f\x66\x03\x02\x02\x02\
	\x40\x41\x0c\x09\x02\x02\x41\x42\x07\x2c\x02\x02\x42\x65\x05\x02\x02\x09\
	\x43\x44\x0c\x07\x02\x02\x44\x45\x09\x03\x02\x02\x45\x65\x05\x02\x02\x08\
	\x46\x47\x0c\x06\x02\x02\x47\x48\x09\x04\x02\x02\x48\x65\x05\x02\x02\x07\
	\x49\x4a\x0c\x05\x02\x02\x4a\x4b\x09\x05\x02\x02\x4b\x65\x05\x02\x02\x06\
	\x4c\x4d\x0c\x04\x02\x02\x4d\x4e\x07\x0f\x02\x02\x4e\x65\x05\x02\x02\x05\
	\x4f\x50\x0c\x03\x02\x02\x50\x51\x07\x10\x02\x02\x51\x65\x05\x02\x02\x04\
	\x52\x53\x0c\x0b\x02\x02\x53\x54\x07\x1b\x02\x02\x54\x55\x05\x02\x02\x02\
	\x55\x56\x07\x1c\x02\x02\x56\x65\x03\x02\x02\x02\x57\x58\x0c\x0a\x02\x02\
	\x58\x61\x07\x17\x02\x02\x59\x5e\x05\x02\x02\x02\x5a\x5b\x07\x13\x02\x02\
	\x5b\x5d\x05\x02\x02\x02\x5c\x5a\x03\x02\x02\x02\x5d\x60\x03\x02\x02\x02\
	\x5e\x5c\x03\x02\x02\x02\x5e\x5f\x03\x02\x02\x02\x5f\x62\x03\x02\x02\x02\
	\x60\x5e\x03\x02\x02\x02\x61\x59\x03\x02\x02\x02\x61\x62\x03\x02\x02\x02\
	\x62\x63\x03\x02\x02\x02\x63\x65\x07\x18\x02\x02\x64\x40\x03\x02\x02\x02\
	\x64\x43\x03\x02\x02\x02\x64\x46\x03\x02\x02\x02\x64\x49\x03\x02\x02\x02\
	\x64\x4c\x03\x02\x02\x02\x64\x4f\x03\x02\x02\x02\x64\x52\x03\x02\x02\x02\
	\x64\x57\x03\x02\x02\x02\x65\x68\x03\x02\x02\x02\x66\x64\x03\x02\x02\x02\
	\x66\x67\x03\x02\x02\x02\x67\x03\x03\x02\x02\x02\x68\x66\x03\x02\x02\x02\
	\x69\x74\x07\x12\x02\x02\x6a\x74\x05\x0a\x06\x02\x6b\x74\x05\x0c\x07\x02\
	\x6c\x74\x05\x0e\x08\x02\x6d\x74\x05\x10\x09\x02\x6e\x74\x05\x12\x0a\x02\
	\x6f\x74\x05\x14\x0b\x02\x70\x74\x05\x16\x0c\x02\x71\x74\x05\x18\x0d\x02\
	\x72\x74\x05\x1a\x0e\x02\x73\x69\x03\x02\x02\x02\x73\x6a\x03\x02\x02\x02\
	\x73\x6b\x03\x02\x02\x02\x73\x6c\x03\x02\x02\x02\x73\x6d\x03\x02\x02\x02\
	\x73\x6e\x03\x02\x02\x02\x73\x6f\x03\x02\x02\x02\x73\x70\x03\x02\x02\x02\
	\x73\x71\x03\x02\x02\x02\x73\x72\x03\x02\x02\x02\x74\x05\x03\x02\x02\x02\
	\x75\x77\x05\x04\x03\x02\x76\x75\x03\x02\x02\x02\x77\x7a\x03\x02\x02\x02\
	\x78\x76\x03\x02\x02\x02\x78\x79\x03\x02\x02\x02\x79\x07\x03\x02\x02\x02\
	\x7a\x78\x03\x02\x02\x02\x7b\x7f\x07\x19\x02\x02\x7c\x7e\x05\x04\x03\x02\
	\x7d\x7c\x03\x02\x02\x02\x7e\u{81}\x03\x02\x02\x02\x7f\x7d\x03\x02\x02\x02\
	\x7f\u{80}\x03\x02\x02\x02\u{80}\u{82}\x03\x02\x02\x02\u{81}\x7f\x03\x02\
	\x02\x02\u{82}\u{83}\x07\x1a\x02\x02\u{83}\x09\x03\x02\x02\x02\u{84}\u{85}\
	\x07\x03\x02\x02\u{85}\u{86}\x07\x2d\x02\x02\u{86}\u{87}\x07\x14\x02\x02\
	\u{87}\u{88}\x05\x02\x02\x02\u{88}\u{89}\x07\x12\x02\x02\u{89}\x0b\x03\x02\
	\x02\x02\u{8a}\u{8b}\x07\x06\x02\x02\u{8b}\u{8c}\x07\x2d\x02\x02\u{8c}\u{8d}\
	\x07\x14\x02\x02\u{8d}\u{8e}\x05\x02\x02\x02\u{8e}\u{8f}\x07\x12\x02\x02\
	\u{8f}\x0d\x03\x02\x02\x02\u{90}\u{91}\x07\x06\x02\x02\u{91}\u{92}\x07\x2d\
	\x02\x02\u{92}\u{93}\x07\x14\x02\x02\u{93}\u{94}\x05\x02\x02\x02\u{94}\u{95}\
	\x07\x07\x02\x02\u{95}\u{96}\x05\x02\x02\x02\u{96}\u{97}\x07\x12\x02\x02\
	\u{97}\x0f\x03\x02\x02\x02\u{98}\u{9c}\x05\x24\x13\x02\u{99}\u{9b}\x05\x26\
	\x14\x02\u{9a}\u{99}\x03\x02\x02\x02\u{9b}\u{9e}\x03\x02\x02\x02\u{9c}\u{9a}\
	\x03\x02\x02\x02\u{9c}\u{9d}\x03\x02\x02\x02\u{9d}\u{a0}\x03\x02\x02\x02\
	\u{9e}\u{9c}\x03\x02\x02\x02\u{9f}\u{a1}\x05\x28\x15\x02\u{a0}\u{9f}\x03\
	\x02\x02\x02\u{a1}\u{a2}\x03\x02\x02\x02\u{a2}\u{a0}\x03\x02\x02\x02\u{a2}\
	\u{a3}\x03\x02\x02\x02\u{a3}\x11\x03\x02\x02\x02\u{a4}\u{a5}\x07\x0a\x02\
	\x02\u{a5}\u{a6}\x07\x17\x02\x02\u{a6}\u{a7}\x05\x02\x02\x02\u{a7}\u{a8}\
	\x07\x18\x02\x02\u{a8}\u{a9}\x05\x08\x05\x02\u{a9}\x13\x03\x02\x02\x02\u{aa}\
	\u{ab}\x07\x0d\x02\x02\u{ab}\u{ac}\x05\x08\x05\x02\u{ac}\u{ad}\x07\x0a\x02\
	\x02\u{ad}\u{ae}\x07\x17\x02\x02\u{ae}\u{af}\x05\x02\x02\x02\u{af}\u{b0}\
	\x07\x18\x02\x02\u{b0}\u{b1}\x07\x12\x02\x02\u{b1}\x15\x03\x02\x02\x02\u{b2}\
	\u{b3}\x07\x0b\x02\x02\u{b3}\u{b4}\x05\x2a\x16\x02\u{b4}\u{b5}\x07\x07\x02\
	\x02\u{b5}\u{b6}\x05\x02\x02\x02\u{b6}\u{b7}\x05\x08\x05\x02\u{b7}\x17\x03\
	\x02\x02\x02\u{b8}\u{b9}\x07\x0c\x02\x02\u{b9}\u{ba}\x07\x2d\x02\x02\u{ba}\
	\u{bb}\x07\x17\x02\x02\u{bb}\u{bc}\x05\x2a\x16\x02\u{bc}\u{bd}\x07\x18\x02\
	\x02\u{bd}\u{be}\x05\x08\x05\x02\u{be}\x19\x03\x02\x02\x02\u{bf}\u{c0}\x07\
	\x0e\x02\x02\u{c0}\u{c1}\x05\x02\x02\x02\u{c1}\u{c2}\x07\x12\x02\x02\u{c2}\
	\x1b\x03\x02\x02\x02\u{c3}\u{c4}\x09\x06\x02\x02\u{c4}\x1d\x03\x02\x02\x02\
	\u{c5}\u{c6}\x09\x07\x02\x02\u{c6}\x1f\x03\x02\x02\x02\u{c7}\u{c8}\x09\x08\
	\x02\x02\u{c8}\x21\x03\x02\x02\x02\u{c9}\u{ca}\x07\x2e\x02\x02\u{ca}\x23\
	\x03\x02\x02\x02\u{cb}\u{cc}\x07\x08\x02\x02\u{cc}\u{cd}\x07\x17\x02\x02\
	\u{cd}\u{ce}\x05\x02\x02\x02\u{ce}\u{cf}\x07\x18\x02\x02\u{cf}\u{d0}\x05\
	\x08\x05\x02\u{d0}\x25\x03\x02\x02\x02\u{d1}\u{d2}\x07\x09\x02\x02\u{d2}\
	\u{d3}\x07\x08\x02\x02\u{d3}\u{d4}\x07\x17\x02\x02\u{d4}\u{d5}\x05\x02\x02\
	\x02\u{d5}\u{d6}\x07\x18\x02\x02\u{d6}\u{d7}\x05\x08\x05\x02\u{d7}\x27\x03\
	\x02\x02\x02\u{d8}\u{d9}\x07\x09\x02\x02\u{d9}\u{da}\x05\x08\x05\x02\u{da}\
	\x29\x03\x02\x02\x02\u{db}\u{e0}\x07\x2d\x02\x02\u{dc}\u{dd}\x07\x13\x02\
	\x02\u{dd}\u{df}\x07\x2d\x02\x02\u{de}\u{dc}\x03\x02\x02\x02\u{df}\u{e2}\
	\x03\x02\x02\x02\u{e0}\u{de}\x03\x02\x02\x02\u{e0}\u{e1}\x03\x02\x02\x02\
	\u{e1}\u{e4}\x03\x02\x02\x02\u{e2}\u{e0}\x03\x02\x02\x02\u{e3}\u{db}\x03\
	\x02\x02\x02\u{e3}\u{e4}\x03\x02\x02\x02\u{e4}\x2b\x03\x02\x02\x02\u{e5}\
	\u{f1}\x07\x19\x02\x02\u{e6}\u{eb}\x05\x02\x02\x02\u{e7}\u{e8}\x07\x13\x02\
	\x02\u{e8}\u{ea}\x05\x02\x02\x02\u{e9}\u{e7}\x03\x02\x02\x02\u{ea}\u{ed}\
	\x03\x02\x02\x02\u{eb}\u{e9}\x03\x02\x02\x02\u{eb}\u{ec}\x03\x02\x02\x02\
	\u{ec}\u{ef}\x03\x02\x02\x02\u{ed}\u{eb}\x03\x02\x02\x02\u{ee}\u{f0}\x07\
	\x13\x02\x02\u{ef}\u{ee}\x03\x02\x02\x02\u{ef}\u{f0}\x03\x02\x02\x02\u{f0}\
	\u{f2}\x03\x02\x02\x02\u{f1}\u{e6}\x03\x02\x02\x02\u{f1}\u{f2}\x03\x02\x02\
	\x02\u{f2}\u{f3}\x03\x02\x02\x02\u{f3}\u{f4}\x07\x1a\x02\x02\u{f4}\x2d\x03\
	\x02\x02\x02\u{f5}\u{f6}\x07\x1b\x02\x02\u{f6}\u{f7}\x05\x22\x12\x02\u{f7}\
	\u{f8}\x07\x1c\x02\x02\u{f8}\u{f9}\x07\x14\x02\x02\u{f9}\u{fa}\x05\x02\x02\
	\x02\u{fa}\x2f\x03\x02\x02\x02\u{fb}\u{107}\x07\x19\x02\x02\u{fc}\u{101}\
	\x05\x2e\x18\x02\u{fd}\u{fe}\x07\x13\x02\x02\u{fe}\u{100}\x05\x2e\x18\x02\
	\u{ff}\u{fd}\x03\x02\x02\x02\u{100}\u{103}\x03\x02\x02\x02\u{101}\u{ff}\
	\x03\x02\x02\x02\u{101}\u{102}\x03\x02\x02\x02\u{102}\u{105}\x03\x02\x02\
	\x02\u{103}\u{101}\x03\x02\x02\x02\u{104}\u{106}\x07\x13\x02\x02\u{105}\
	\u{104}\x03\x02\x02\x02\u{105}\u{106}\x03\x02\x02\x02\u{106}\u{108}\x03\
	\x02\x02\x02\u{107}\u{fc}\x03\x02\x02\x02\u{107}\u{108}\x03\x02\x02\x02\
	\u{108}\u{109}\x03\x02\x02\x02\u{109}\u{10a}\x07\x1a\x02\x02\u{10a}\x31\
	\x03\x02\x02\x02\u{10b}\u{10c}\x07\x0c\x02\x02\u{10c}\u{10d}\x07\x17\x02\
	\x02\u{10d}\u{10e}\x05\x2a\x16\x02\u{10e}\u{10f}\x07\x18\x02\x02\u{10f}\
	\u{110}\x05\x08\x05\x02\u{110}\u{111}\x07\x12\x02\x02\u{111}\x33\x03\x02\
	\x02\x02\x14\x3e\x5e\x61\x64\x66\x73\x78\x7f\u{9c}\u{a2}\u{e0}\u{e3}\u{eb}\
	\u{ef}\u{f1}\u{101}\u{105}\u{107}";

