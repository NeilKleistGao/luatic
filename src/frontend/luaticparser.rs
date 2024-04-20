// Generated from luatic.g4 by ANTLR 4.8
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
		pub const PT_SEMI:isize=4; 
		pub const PT_EQL:isize=5; 
		pub const PT_LT:isize=6; 
		pub const PT_RT:isize=7; 
		pub const PT_LP:isize=8; 
		pub const PT_RP:isize=9; 
		pub const PT_LB:isize=10; 
		pub const PT_RB:isize=11; 
		pub const PT_LS:isize=12; 
		pub const PT_RS:isize=13; 
		pub const IDENT:isize=14; 
		pub const NORMALSTRING:isize=15; 
		pub const INT:isize=16; 
		pub const HEX:isize=17; 
		pub const FLOAT:isize=18; 
		pub const HEX_FLOAT:isize=19; 
		pub const WS:isize=20;
	pub const RULE_expr:usize = 0; 
	pub const RULE_stat:usize = 1; 
	pub const RULE_global_stat:usize = 2; 
	pub const RULE_number:usize = 3; 
	pub const RULE_string:usize = 4;
	pub const ruleNames: [&'static str; 5] =  [
		"expr", "stat", "global_stat", "number", "string"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;14] = [
		None, Some("'global'"), Some("'true'"), Some("'false'"), Some("';'"), 
		Some("'='"), Some("'<'"), Some("'>'"), Some("'('"), Some("')'"), Some("'{'"), 
		Some("'}'"), Some("'['"), Some("']'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;21]  = [
		None, Some("KW_GLOBAL"), Some("KW_TRUE"), Some("KW_FALSE"), Some("PT_SEMI"), 
		Some("PT_EQL"), Some("PT_LT"), Some("PT_RT"), Some("PT_LP"), Some("PT_RP"), 
		Some("PT_LB"), Some("PT_RB"), Some("PT_LS"), Some("PT_RS"), Some("IDENT"), 
		Some("NORMALSTRING"), Some("INT"), Some("HEX"), Some("FLOAT"), Some("HEX_FLOAT"), 
		Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,luaticParserExt<'input>, I, luaticParserContextType , dyn luaticListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type luaticTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, luaticParserContextType , dyn luaticListener<'input> + 'a>;

/// Parser for luatic grammar
pub struct luaticParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> luaticParser<'input, I, H>
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
				luaticParserExt{
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

impl<'input, I> luaticParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> luaticParser<'input, I, DefaultErrorStrategy<'input,luaticParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for luaticParser
pub trait luaticParserContext<'input>:
	for<'x> Listenable<dyn luaticListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=luaticParserContextType>
{}

antlr_rust::coerce_from!{ 'input : luaticParserContext<'input> }

impl<'input> luaticParserContext<'input> for TerminalNode<'input,luaticParserContextType> {}
impl<'input> luaticParserContext<'input> for ErrorNode<'input,luaticParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn luaticParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn luaticListener<'input> + 'input }

pub struct luaticParserContextType;
antlr_rust::tid!{luaticParserContextType}

impl<'input> ParserNodeType<'input> for luaticParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn luaticParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for luaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for luaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct luaticParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> luaticParserExt<'input>{
}
antlr_rust::tid! { luaticParserExt<'a> }

impl<'input> TokenAware<'input> for luaticParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for luaticParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for luaticParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "luatic.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- expr ----------------
pub type ExprContextAll<'input> = ExprContext<'input>;


pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> luaticParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn luaticListener<'input> + 'a> for ExprContext<'input>{
		fn enter(&self,listener: &mut (dyn luaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expr(self);
		}fn exit(&self,listener: &mut (dyn luaticListener<'input> + 'a)) {
			listener.exit_expr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = luaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn luaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExprContextAttrs<'input>: luaticParserContext<'input> + BorrowMut<ExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_TRUE
/// Returns `None` if there is no child corresponding to token KW_TRUE
fn KW_TRUE(&self) -> Option<Rc<TerminalNode<'input,luaticParserContextType>>> where Self:Sized{
	self.get_token(KW_TRUE, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_FALSE
/// Returns `None` if there is no child corresponding to token KW_FALSE
fn KW_FALSE(&self) -> Option<Rc<TerminalNode<'input,luaticParserContextType>>> where Self:Sized{
	self.get_token(KW_FALSE, 0)
}

}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

impl<'input, I, H> luaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_expr);
        let mut _localctx: Rc<ExprContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(10);
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
//------------------- stat ----------------
pub type StatContextAll<'input> = StatContext<'input>;


pub type StatContext<'input> = BaseParserRuleContext<'input,StatContextExt<'input>>;

#[derive(Clone)]
pub struct StatContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> luaticParserContext<'input> for StatContext<'input>{}

impl<'input,'a> Listenable<dyn luaticListener<'input> + 'a> for StatContext<'input>{
		fn enter(&self,listener: &mut (dyn luaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stat(self);
		}fn exit(&self,listener: &mut (dyn luaticListener<'input> + 'a)) {
			listener.exit_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for StatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = luaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stat }
}
antlr_rust::tid!{StatContextExt<'a>}

impl<'input> StatContextExt<'input>{
	fn new(parent: Option<Rc<dyn luaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatContextAttrs<'input>: luaticParserContext<'input> + BorrowMut<StatContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PT_SEMI
/// Returns `None` if there is no child corresponding to token PT_SEMI
fn PT_SEMI(&self) -> Option<Rc<TerminalNode<'input,luaticParserContextType>>> where Self:Sized{
	self.get_token(PT_SEMI, 0)
}
fn global_stat(&self) -> Option<Rc<Global_statContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatContextAttrs<'input> for StatContext<'input>{}

impl<'input, I, H> luaticParser<'input, I, H>
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

			recog.base.set_state(14);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 PT_SEMI 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(12);
					recog.base.match_token(PT_SEMI,&mut recog.err_handler)?;

					}
				}

			 KW_GLOBAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule global_stat*/
					recog.base.set_state(13);
					recog.global_stat()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
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

impl<'input> luaticParserContext<'input> for Global_statContext<'input>{}

impl<'input,'a> Listenable<dyn luaticListener<'input> + 'a> for Global_statContext<'input>{
		fn enter(&self,listener: &mut (dyn luaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_global_stat(self);
		}fn exit(&self,listener: &mut (dyn luaticListener<'input> + 'a)) {
			listener.exit_global_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Global_statContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = luaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_global_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_global_stat }
}
antlr_rust::tid!{Global_statContextExt<'a>}

impl<'input> Global_statContextExt<'input>{
	fn new(parent: Option<Rc<dyn luaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Global_statContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Global_statContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Global_statContextAttrs<'input>: luaticParserContext<'input> + BorrowMut<Global_statContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token KW_GLOBAL
/// Returns `None` if there is no child corresponding to token KW_GLOBAL
fn KW_GLOBAL(&self) -> Option<Rc<TerminalNode<'input,luaticParserContextType>>> where Self:Sized{
	self.get_token(KW_GLOBAL, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,luaticParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_EQL
/// Returns `None` if there is no child corresponding to token PT_EQL
fn PT_EQL(&self) -> Option<Rc<TerminalNode<'input,luaticParserContextType>>> where Self:Sized{
	self.get_token(PT_EQL, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Global_statContextAttrs<'input> for Global_statContext<'input>{}

impl<'input, I, H> luaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn global_stat(&mut self,)
	-> Result<Rc<Global_statContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Global_statContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_global_stat);
        let mut _localctx: Rc<Global_statContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(16);
			recog.base.match_token(KW_GLOBAL,&mut recog.err_handler)?;

			recog.base.set_state(17);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(18);
			recog.base.match_token(PT_EQL,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(19);
			recog.expr()?;

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

impl<'input> luaticParserContext<'input> for NumberContext<'input>{}

impl<'input,'a> Listenable<dyn luaticListener<'input> + 'a> for NumberContext<'input>{
		fn enter(&self,listener: &mut (dyn luaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_number(self);
		}fn exit(&self,listener: &mut (dyn luaticListener<'input> + 'a)) {
			listener.exit_number(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for NumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = luaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_number }
	//fn type_rule_index() -> usize where Self: Sized { RULE_number }
}
antlr_rust::tid!{NumberContextExt<'a>}

impl<'input> NumberContextExt<'input>{
	fn new(parent: Option<Rc<dyn luaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumberContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumberContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NumberContextAttrs<'input>: luaticParserContext<'input> + BorrowMut<NumberContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INT
/// Returns `None` if there is no child corresponding to token INT
fn INT(&self) -> Option<Rc<TerminalNode<'input,luaticParserContextType>>> where Self:Sized{
	self.get_token(INT, 0)
}
/// Retrieves first TerminalNode corresponding to token HEX
/// Returns `None` if there is no child corresponding to token HEX
fn HEX(&self) -> Option<Rc<TerminalNode<'input,luaticParserContextType>>> where Self:Sized{
	self.get_token(HEX, 0)
}
/// Retrieves first TerminalNode corresponding to token FLOAT
/// Returns `None` if there is no child corresponding to token FLOAT
fn FLOAT(&self) -> Option<Rc<TerminalNode<'input,luaticParserContextType>>> where Self:Sized{
	self.get_token(FLOAT, 0)
}
/// Retrieves first TerminalNode corresponding to token HEX_FLOAT
/// Returns `None` if there is no child corresponding to token HEX_FLOAT
fn HEX_FLOAT(&self) -> Option<Rc<TerminalNode<'input,luaticParserContextType>>> where Self:Sized{
	self.get_token(HEX_FLOAT, 0)
}

}

impl<'input> NumberContextAttrs<'input> for NumberContext<'input>{}

impl<'input, I, H> luaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn number(&mut self,)
	-> Result<Rc<NumberContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_number);
        let mut _localctx: Rc<NumberContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(21);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << INT) | (1usize << HEX) | (1usize << FLOAT) | (1usize << HEX_FLOAT))) != 0)) } {
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

impl<'input> luaticParserContext<'input> for StringContext<'input>{}

impl<'input,'a> Listenable<dyn luaticListener<'input> + 'a> for StringContext<'input>{
		fn enter(&self,listener: &mut (dyn luaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_string(self);
		}fn exit(&self,listener: &mut (dyn luaticListener<'input> + 'a)) {
			listener.exit_string(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for StringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = luaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_string }
	//fn type_rule_index() -> usize where Self: Sized { RULE_string }
}
antlr_rust::tid!{StringContextExt<'a>}

impl<'input> StringContextExt<'input>{
	fn new(parent: Option<Rc<dyn luaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StringContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StringContextAttrs<'input>: luaticParserContext<'input> + BorrowMut<StringContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NORMALSTRING
/// Returns `None` if there is no child corresponding to token NORMALSTRING
fn NORMALSTRING(&self) -> Option<Rc<TerminalNode<'input,luaticParserContextType>>> where Self:Sized{
	self.get_token(NORMALSTRING, 0)
}

}

impl<'input> StringContextAttrs<'input> for StringContext<'input>{}

impl<'input, I, H> luaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn string(&mut self,)
	-> Result<Rc<StringContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_string);
        let mut _localctx: Rc<StringContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(23);
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
	\x16\x1c\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x03\x02\x03\x02\x03\x03\x03\x03\x05\x03\x11\x0a\x03\x03\
	\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x05\x03\x05\x03\x06\x03\x06\x03\
	\x06\x02\x02\x07\x02\x04\x06\x08\x0a\x02\x04\x03\x02\x04\x05\x03\x02\x12\
	\x15\x02\x17\x02\x0c\x03\x02\x02\x02\x04\x10\x03\x02\x02\x02\x06\x12\x03\
	\x02\x02\x02\x08\x17\x03\x02\x02\x02\x0a\x19\x03\x02\x02\x02\x0c\x0d\x09\
	\x02\x02\x02\x0d\x03\x03\x02\x02\x02\x0e\x11\x07\x06\x02\x02\x0f\x11\x05\
	\x06\x04\x02\x10\x0e\x03\x02\x02\x02\x10\x0f\x03\x02\x02\x02\x11\x05\x03\
	\x02\x02\x02\x12\x13\x07\x03\x02\x02\x13\x14\x07\x10\x02\x02\x14\x15\x07\
	\x07\x02\x02\x15\x16\x05\x02\x02\x02\x16\x07\x03\x02\x02\x02\x17\x18\x09\
	\x03\x02\x02\x18\x09\x03\x02\x02\x02\x19\x1a\x07\x11\x02\x02\x1a\x0b\x03\
	\x02\x02\x02\x03\x10";

