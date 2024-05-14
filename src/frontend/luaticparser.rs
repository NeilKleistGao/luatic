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

		pub const KW_LANG:isize=1; 
		pub const PT_SEMI:isize=2; 
		pub const PT_COLON:isize=3; 
		pub const PT_COMMA:isize=4; 
		pub const PT_ASGN:isize=5; 
		pub const PT_LT:isize=6; 
		pub const PT_RT:isize=7; 
		pub const PT_LP:isize=8; 
		pub const PT_RP:isize=9; 
		pub const PT_LB:isize=10; 
		pub const PT_RB:isize=11; 
		pub const PT_LS:isize=12; 
		pub const PT_RS:isize=13; 
		pub const PT_PLUS:isize=14; 
		pub const PT_MINUS:isize=15; 
		pub const PT_MUL:isize=16; 
		pub const PT_DIV:isize=17; 
		pub const PT_IDIV:isize=18; 
		pub const PT_MOD:isize=19; 
		pub const PT_LE:isize=20; 
		pub const PT_GE:isize=21; 
		pub const PT_EQ:isize=22; 
		pub const PT_NE:isize=23; 
		pub const PT_AND:isize=24; 
		pub const PT_OR:isize=25; 
		pub const PT_XN:isize=26; 
		pub const PT_LSF:isize=27; 
		pub const PT_RSF:isize=28; 
		pub const PT_POW:isize=29; 
		pub const PT_DDASH:isize=30; 
		pub const IDENT:isize=31; 
		pub const NORMALSTRING:isize=32; 
		pub const CHARASTRING:isize=33; 
		pub const INT:isize=34; 
		pub const HEX:isize=35; 
		pub const FLOAT:isize=36; 
		pub const HEX_FLOAT:isize=37; 
		pub const COMMENT:isize=38; 
		pub const LINE_COMMENT:isize=39; 
		pub const WS:isize=40;
	pub const RULE_dialog_block:usize = 0; 
	pub const RULE_say_stat:usize = 1; 
	pub const RULE_stat:usize = 2; 
	pub const RULE_lang_annotation:usize = 3; 
	pub const RULE_prgm:usize = 4; 
	pub const RULE_block:usize = 5; 
	pub const RULE_say_block:usize = 6; 
	pub const RULE_string:usize = 7; 
	pub const RULE_character:usize = 8;
	pub const ruleNames: [&'static str; 9] =  [
		"dialog_block", "say_stat", "stat", "lang_annotation", "prgm", "block", 
		"say_block", "string", "character"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;31] = [
		None, Some("'lang'"), Some("';'"), Some("':'"), Some("','"), Some("'='"), 
		Some("'<'"), Some("'>'"), Some("'('"), Some("')'"), Some("'{'"), Some("'}'"), 
		Some("'['"), Some("']'"), Some("'+'"), Some("'-'"), Some("'*'"), Some("'/'"), 
		Some("'//'"), Some("'%'"), Some("'<='"), Some("'>='"), Some("'=='"), Some("'~='"), 
		Some("'&'"), Some("'|'"), Some("'~'"), Some("'<<'"), Some("'>>'"), Some("'^'"), 
		Some("'--'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;41]  = [
		None, Some("KW_LANG"), Some("PT_SEMI"), Some("PT_COLON"), Some("PT_COMMA"), 
		Some("PT_ASGN"), Some("PT_LT"), Some("PT_RT"), Some("PT_LP"), Some("PT_RP"), 
		Some("PT_LB"), Some("PT_RB"), Some("PT_LS"), Some("PT_RS"), Some("PT_PLUS"), 
		Some("PT_MINUS"), Some("PT_MUL"), Some("PT_DIV"), Some("PT_IDIV"), Some("PT_MOD"), 
		Some("PT_LE"), Some("PT_GE"), Some("PT_EQ"), Some("PT_NE"), Some("PT_AND"), 
		Some("PT_OR"), Some("PT_XN"), Some("PT_LSF"), Some("PT_RSF"), Some("PT_POW"), 
		Some("PT_DDASH"), Some("IDENT"), Some("NORMALSTRING"), Some("CHARASTRING"), 
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
}
//------------------- dialog_block ----------------
pub type Dialog_blockContextAll<'input> = Dialog_blockContext<'input>;


pub type Dialog_blockContext<'input> = BaseParserRuleContext<'input,Dialog_blockContextExt<'input>>;

#[derive(Clone)]
pub struct Dialog_blockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Dialog_blockContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Dialog_blockContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_dialog_block(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_dialog_block(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Dialog_blockContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_dialog_block(self);
	}
}

impl<'input> CustomRuleContext<'input> for Dialog_blockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dialog_block }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dialog_block }
}
antlr_rust::tid!{Dialog_blockContextExt<'a>}

impl<'input> Dialog_blockContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Dialog_blockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Dialog_blockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Dialog_blockContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Dialog_blockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PT_LSF
/// Returns `None` if there is no child corresponding to token PT_LSF
fn PT_LSF(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_LSF, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_RSF
/// Returns `None` if there is no child corresponding to token PT_RSF
fn PT_RSF(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_RSF, 0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Dialog_blockContextAttrs<'input> for Dialog_blockContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn dialog_block(&mut self,)
	-> Result<Rc<Dialog_blockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Dialog_blockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_dialog_block);
        let mut _localctx: Rc<Dialog_blockContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(18);
			recog.base.match_token(PT_LSF,&mut recog.err_handler)?;

			recog.base.set_state(19);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(20);
			recog.base.match_token(PT_RSF,&mut recog.err_handler)?;

			/*InvokeRule block*/
			recog.base.set_state(21);
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
//------------------- say_stat ----------------
pub type Say_statContextAll<'input> = Say_statContext<'input>;


pub type Say_statContext<'input> = BaseParserRuleContext<'input,Say_statContextExt<'input>>;

#[derive(Clone)]
pub struct Say_statContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Say_statContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Say_statContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_say_stat(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_say_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Say_statContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_say_stat(self);
	}
}

impl<'input> CustomRuleContext<'input> for Say_statContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_say_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_say_stat }
}
antlr_rust::tid!{Say_statContextExt<'a>}

impl<'input> Say_statContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Say_statContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Say_statContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Say_statContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Say_statContextExt<'input>>{

fn character(&self) -> Option<Rc<CharacterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PT_COLON
/// Returns `None` if there is no child corresponding to token PT_COLON
fn PT_COLON(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_COLON, 0)
}
fn say_block(&self) -> Option<Rc<Say_blockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn string(&self) -> Option<Rc<StringContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PT_SEMI
/// Returns `None` if there is no child corresponding to token PT_SEMI
fn PT_SEMI(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_SEMI, 0)
}

}

impl<'input> Say_statContextAttrs<'input> for Say_statContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn say_stat(&mut self,)
	-> Result<Rc<Say_statContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Say_statContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_say_stat);
        let mut _localctx: Rc<Say_statContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule character*/
			recog.base.set_state(23);
			recog.character()?;

			recog.base.set_state(24);
			recog.base.match_token(PT_COLON,&mut recog.err_handler)?;

			recog.base.set_state(29);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 NORMALSTRING 
				=> {
					{
					{
					/*InvokeRule string*/
					recog.base.set_state(25);
					recog.string()?;

					recog.base.set_state(26);
					recog.base.match_token(PT_SEMI,&mut recog.err_handler)?;

					}
					}
				}

			 PT_LB 
				=> {
					{
					/*InvokeRule say_block*/
					recog.base.set_state(28);
					recog.say_block()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
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

fn say_stat(&self) -> Option<Rc<Say_statContextAll<'input>>> where Self:Sized{
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
        recog.base.enter_rule(_localctx.clone(), 4, RULE_stat);
        let mut _localctx: Rc<StatContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule say_stat*/
			recog.base.set_state(31);
			recog.say_stat()?;

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
//------------------- lang_annotation ----------------
pub type Lang_annotationContextAll<'input> = Lang_annotationContext<'input>;


pub type Lang_annotationContext<'input> = BaseParserRuleContext<'input,Lang_annotationContextExt<'input>>;

#[derive(Clone)]
pub struct Lang_annotationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Lang_annotationContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Lang_annotationContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lang_annotation(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_lang_annotation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Lang_annotationContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_lang_annotation(self);
	}
}

impl<'input> CustomRuleContext<'input> for Lang_annotationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lang_annotation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lang_annotation }
}
antlr_rust::tid!{Lang_annotationContextExt<'a>}

impl<'input> Lang_annotationContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Lang_annotationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Lang_annotationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Lang_annotationContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Lang_annotationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PT_DDASH
/// Returns `None` if there is no child corresponding to token PT_DDASH
fn PT_DDASH(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_DDASH, 0)
}
/// Retrieves first TerminalNode corresponding to token KW_LANG
/// Returns `None` if there is no child corresponding to token KW_LANG
fn KW_LANG(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(KW_LANG, 0)
}
/// Retrieves first TerminalNode corresponding to token PT_COLON
/// Returns `None` if there is no child corresponding to token PT_COLON
fn PT_COLON(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(PT_COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}

}

impl<'input> Lang_annotationContextAttrs<'input> for Lang_annotationContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lang_annotation(&mut self,)
	-> Result<Rc<Lang_annotationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Lang_annotationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_lang_annotation);
        let mut _localctx: Rc<Lang_annotationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(33);
			recog.base.match_token(PT_DDASH,&mut recog.err_handler)?;

			recog.base.set_state(34);
			recog.base.match_token(KW_LANG,&mut recog.err_handler)?;

			recog.base.set_state(35);
			recog.base.match_token(PT_COLON,&mut recog.err_handler)?;

			recog.base.set_state(36);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

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

fn lang_annotation(&self) -> Option<Rc<Lang_annotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn dialog_block_all(&self) ->  Vec<Rc<Dialog_blockContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn dialog_block(&self, i: usize) -> Option<Rc<Dialog_blockContextAll<'input>>> where Self:Sized{
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
        recog.base.enter_rule(_localctx.clone(), 8, RULE_prgm);
        let mut _localctx: Rc<PrgmContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(39);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==PT_DDASH {
				{
				/*InvokeRule lang_annotation*/
				recog.base.set_state(38);
				recog.lang_annotation()?;

				}
			}

			recog.base.set_state(44);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==PT_LSF {
				{
				{
				/*InvokeRule dialog_block*/
				recog.base.set_state(41);
				recog.dialog_block()?;

				}
				}
				recog.base.set_state(46);
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
        recog.base.enter_rule(_localctx.clone(), 10, RULE_block);
        let mut _localctx: Rc<BlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(47);
			recog.base.match_token(PT_LB,&mut recog.err_handler)?;

			recog.base.set_state(51);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==CHARASTRING {
				{
				{
				/*InvokeRule stat*/
				recog.base.set_state(48);
				recog.stat()?;

				}
				}
				recog.base.set_state(53);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(54);
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
//------------------- say_block ----------------
pub type Say_blockContextAll<'input> = Say_blockContext<'input>;


pub type Say_blockContext<'input> = BaseParserRuleContext<'input,Say_blockContextExt<'input>>;

#[derive(Clone)]
pub struct Say_blockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for Say_blockContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for Say_blockContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_say_block(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_say_block(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for Say_blockContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_say_block(self);
	}
}

impl<'input> CustomRuleContext<'input> for Say_blockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_say_block }
	//fn type_rule_index() -> usize where Self: Sized { RULE_say_block }
}
antlr_rust::tid!{Say_blockContextExt<'a>}

impl<'input> Say_blockContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Say_blockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Say_blockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Say_blockContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<Say_blockContextExt<'input>>{

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
fn string_all(&self) ->  Vec<Rc<StringContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn string(&self, i: usize) -> Option<Rc<StringContextAll<'input>>> where Self:Sized{
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

impl<'input> Say_blockContextAttrs<'input> for Say_blockContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn say_block(&mut self,)
	-> Result<Rc<Say_blockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Say_blockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_say_block);
        let mut _localctx: Rc<Say_blockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(56);
			recog.base.match_token(PT_LB,&mut recog.err_handler)?;

			{
			recog.base.set_state(62);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule string*/
					recog.base.set_state(57);
					recog.string()?;

					recog.base.set_state(58);
					recog.base.match_token(PT_COMMA,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(64);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
			}
			/*InvokeRule string*/
			recog.base.set_state(65);
			recog.string()?;

			recog.base.set_state(67);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==PT_COMMA {
				{
				recog.base.set_state(66);
				recog.base.match_token(PT_COMMA,&mut recog.err_handler)?;

				}
			}

			}
			recog.base.set_state(69);
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
        recog.base.enter_rule(_localctx.clone(), 14, RULE_string);
        let mut _localctx: Rc<StringContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(71);
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
//------------------- character ----------------
pub type CharacterContextAll<'input> = CharacterContext<'input>;


pub type CharacterContext<'input> = BaseParserRuleContext<'input,CharacterContextExt<'input>>;

#[derive(Clone)]
pub struct CharacterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LuaticParserContext<'input> for CharacterContext<'input>{}

impl<'input,'a> Listenable<dyn LuaticListener<'input> + 'a> for CharacterContext<'input>{
		fn enter(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_character(self);
		}
		fn exit(&self,listener: &mut (dyn LuaticListener<'input> + 'a)) {
			listener.exit_character(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LuaticVisitor<'input> + 'a> for CharacterContext<'input>{
	fn accept(&self,visitor: &mut (dyn LuaticVisitor<'input> + 'a)) {
		visitor.visit_character(self);
	}
}

impl<'input> CustomRuleContext<'input> for CharacterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LuaticParserContextType;
	fn get_rule_index(&self) -> usize { RULE_character }
	//fn type_rule_index() -> usize where Self: Sized { RULE_character }
}
antlr_rust::tid!{CharacterContextExt<'a>}

impl<'input> CharacterContextExt<'input>{
	fn new(parent: Option<Rc<dyn LuaticParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CharacterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CharacterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CharacterContextAttrs<'input>: LuaticParserContext<'input> + BorrowMut<CharacterContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CHARASTRING
/// Returns `None` if there is no child corresponding to token CHARASTRING
fn CHARASTRING(&self) -> Option<Rc<TerminalNode<'input,LuaticParserContextType>>> where Self:Sized{
	self.get_token(CHARASTRING, 0)
}

}

impl<'input> CharacterContextAttrs<'input> for CharacterContext<'input>{}

impl<'input, I, H> LuaticParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn character(&mut self,)
	-> Result<Rc<CharacterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CharacterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_character);
        let mut _localctx: Rc<CharacterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(73);
			recog.base.match_token(CHARASTRING,&mut recog.err_handler)?;

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
	\x2a\x4e\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x05\x03\x20\x0a\x03\x03\x04\x03\x04\x03\x05\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x06\x05\x06\x2a\x0a\x06\x03\x06\x07\x06\
	\x2d\x0a\x06\x0c\x06\x0e\x06\x30\x0b\x06\x03\x07\x03\x07\x07\x07\x34\x0a\
	\x07\x0c\x07\x0e\x07\x37\x0b\x07\x03\x07\x03\x07\x03\x08\x03\x08\x03\x08\
	\x03\x08\x07\x08\x3f\x0a\x08\x0c\x08\x0e\x08\x42\x0b\x08\x03\x08\x03\x08\
	\x05\x08\x46\x0a\x08\x03\x08\x03\x08\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\
	\x0a\x02\x02\x0b\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x02\x02\x02\x4a\x02\
	\x14\x03\x02\x02\x02\x04\x19\x03\x02\x02\x02\x06\x21\x03\x02\x02\x02\x08\
	\x23\x03\x02\x02\x02\x0a\x29\x03\x02\x02\x02\x0c\x31\x03\x02\x02\x02\x0e\
	\x3a\x03\x02\x02\x02\x10\x49\x03\x02\x02\x02\x12\x4b\x03\x02\x02\x02\x14\
	\x15\x07\x1d\x02\x02\x15\x16\x07\x21\x02\x02\x16\x17\x07\x1e\x02\x02\x17\
	\x18\x05\x0c\x07\x02\x18\x03\x03\x02\x02\x02\x19\x1a\x05\x12\x0a\x02\x1a\
	\x1f\x07\x05\x02\x02\x1b\x1c\x05\x10\x09\x02\x1c\x1d\x07\x04\x02\x02\x1d\
	\x20\x03\x02\x02\x02\x1e\x20\x05\x0e\x08\x02\x1f\x1b\x03\x02\x02\x02\x1f\
	\x1e\x03\x02\x02\x02\x20\x05\x03\x02\x02\x02\x21\x22\x05\x04\x03\x02\x22\
	\x07\x03\x02\x02\x02\x23\x24\x07\x20\x02\x02\x24\x25\x07\x03\x02\x02\x25\
	\x26\x07\x05\x02\x02\x26\x27\x07\x21\x02\x02\x27\x09\x03\x02\x02\x02\x28\
	\x2a\x05\x08\x05\x02\x29\x28\x03\x02\x02\x02\x29\x2a\x03\x02\x02\x02\x2a\
	\x2e\x03\x02\x02\x02\x2b\x2d\x05\x02\x02\x02\x2c\x2b\x03\x02\x02\x02\x2d\
	\x30\x03\x02\x02\x02\x2e\x2c\x03\x02\x02\x02\x2e\x2f\x03\x02\x02\x02\x2f\
	\x0b\x03\x02\x02\x02\x30\x2e\x03\x02\x02\x02\x31\x35\x07\x0c\x02\x02\x32\
	\x34\x05\x06\x04\x02\x33\x32\x03\x02\x02\x02\x34\x37\x03\x02\x02\x02\x35\
	\x33\x03\x02\x02\x02\x35\x36\x03\x02\x02\x02\x36\x38\x03\x02\x02\x02\x37\
	\x35\x03\x02\x02\x02\x38\x39\x07\x0d\x02\x02\x39\x0d\x03\x02\x02\x02\x3a\
	\x40\x07\x0c\x02\x02\x3b\x3c\x05\x10\x09\x02\x3c\x3d\x07\x06\x02\x02\x3d\
	\x3f\x03\x02\x02\x02\x3e\x3b\x03\x02\x02\x02\x3f\x42\x03\x02\x02\x02\x40\
	\x3e\x03\x02\x02\x02\x40\x41\x03\x02\x02\x02\x41\x43\x03\x02\x02\x02\x42\
	\x40\x03\x02\x02\x02\x43\x45\x05\x10\x09\x02\x44\x46\x07\x06\x02\x02\x45\
	\x44\x03\x02\x02\x02\x45\x46\x03\x02\x02\x02\x46\x47\x03\x02\x02\x02\x47\
	\x48\x07\x0d\x02\x02\x48\x0f\x03\x02\x02\x02\x49\x4a\x07\x22\x02\x02\x4a\
	\x11\x03\x02\x02\x02\x4b\x4c\x07\x23\x02\x02\x4c\x13\x03\x02\x02\x02\x08\
	\x1f\x29\x2e\x35\x40\x45";

