// Generated from luatic.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::char_stream::CharStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr_rust::PredictionContextCache;
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::TokenSource;
use antlr_rust::token_factory::{TokenFactory,CommonTokenFactory,TokenAware};
use antlr_rust::token::*;
use antlr_rust::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyContext};
use antlr_rust::parser_rule_context::{ParserRuleContext,BaseParserRuleContext,cast};
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};

use antlr_rust::{lazy_static,Tid,TidAble,TidExt};

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


	pub const KW_GLOBAL:isize=1; 
	pub const KW_TRUE:isize=2; 
	pub const KW_FALSE:isize=3; 
	pub const PT_SEMI:isize=4; 
	pub const PT_EQL:isize=5; 
	pub const IDENT:isize=6; 
	pub const WS:isize=7;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;7] = [
		"KW_GLOBAL", "KW_TRUE", "KW_FALSE", "PT_SEMI", "PT_EQL", "IDENT", "WS"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;6] = [
		None, Some("'global'"), Some("'true'"), Some("'false'"), Some("';'"), 
		Some("'='")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;8]  = [
		None, Some("KW_GLOBAL"), Some("KW_TRUE"), Some("KW_FALSE"), Some("PT_SEMI"), 
		Some("PT_EQL"), Some("IDENT"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub type LexerContext<'input> = BaseRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

pub struct luaticLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,luaticLexerActions,Input,LocalTokenFactory<'input>>,
}

antlr_rust::tid! { impl<'input,Input> TidAble<'input> for luaticLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input:CharStream<From<'input> >> Deref for luaticLexer<'input,Input>{
	type Target = BaseLexer<'input,luaticLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for luaticLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> luaticLexer<'input,Input>{
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "luaticLexer.g4"
    }

	pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
		antlr_rust::recognizer::check_version("0","3");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				luaticLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> luaticLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Input) -> Self{
		luaticLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct luaticLexerActions {
}

impl luaticLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,luaticLexerActions,Input,LocalTokenFactory<'input>>> for luaticLexerActions{
	}

	impl<'input, Input:CharStream<From<'input> >> luaticLexer<'input,Input>{

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,luaticLexerActions,Input,LocalTokenFactory<'input>>> for luaticLexerActions{
}
impl<'input> TokenAware<'input> for luaticLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for luaticLexer<'input,Input>{
	type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

	fn get_source_name(&self) -> String {
		self.base.get_source_name()
	}

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
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
		"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x09\x35\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x03\x02\x03\x02\
		\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\x03\
		\x03\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x05\x03\x05\
		\x03\x06\x03\x06\x03\x07\x03\x07\x07\x07\x2a\x0a\x07\x0c\x07\x0e\x07\x2d\
		\x0b\x07\x03\x08\x06\x08\x30\x0a\x08\x0d\x08\x0e\x08\x31\x03\x08\x03\x08\
		\x02\x02\x09\x03\x03\x05\x04\x07\x05\x09\x06\x0b\x07\x0d\x08\x0f\x09\x03\
		\x02\x05\x05\x02\x43\x5c\x61\x61\x63\x7c\x06\x02\x32\x3b\x43\x5c\x61\x61\
		\x63\x7c\x05\x02\x0b\x0c\x0e\x0f\x22\x22\x02\x36\x02\x03\x03\x02\x02\x02\
		\x02\x05\x03\x02\x02\x02\x02\x07\x03\x02\x02\x02\x02\x09\x03\x02\x02\x02\
		\x02\x0b\x03\x02\x02\x02\x02\x0d\x03\x02\x02\x02\x02\x0f\x03\x02\x02\x02\
		\x03\x11\x03\x02\x02\x02\x05\x18\x03\x02\x02\x02\x07\x1d\x03\x02\x02\x02\
		\x09\x23\x03\x02\x02\x02\x0b\x25\x03\x02\x02\x02\x0d\x27\x03\x02\x02\x02\
		\x0f\x2f\x03\x02\x02\x02\x11\x12\x07\x69\x02\x02\x12\x13\x07\x6e\x02\x02\
		\x13\x14\x07\x71\x02\x02\x14\x15\x07\x64\x02\x02\x15\x16\x07\x63\x02\x02\
		\x16\x17\x07\x6e\x02\x02\x17\x04\x03\x02\x02\x02\x18\x19\x07\x76\x02\x02\
		\x19\x1a\x07\x74\x02\x02\x1a\x1b\x07\x77\x02\x02\x1b\x1c\x07\x67\x02\x02\
		\x1c\x06\x03\x02\x02\x02\x1d\x1e\x07\x68\x02\x02\x1e\x1f\x07\x63\x02\x02\
		\x1f\x20\x07\x6e\x02\x02\x20\x21\x07\x75\x02\x02\x21\x22\x07\x67\x02\x02\
		\x22\x08\x03\x02\x02\x02\x23\x24\x07\x3d\x02\x02\x24\x0a\x03\x02\x02\x02\
		\x25\x26\x07\x3f\x02\x02\x26\x0c\x03\x02\x02\x02\x27\x2b\x09\x02\x02\x02\
		\x28\x2a\x09\x03\x02\x02\x29\x28\x03\x02\x02\x02\x2a\x2d\x03\x02\x02\x02\
		\x2b\x29\x03\x02\x02\x02\x2b\x2c\x03\x02\x02\x02\x2c\x0e\x03\x02\x02\x02\
		\x2d\x2b\x03\x02\x02\x02\x2e\x30\x09\x04\x02\x02\x2f\x2e\x03\x02\x02\x02\
		\x30\x31\x03\x02\x02\x02\x31\x2f\x03\x02\x02\x02\x31\x32\x03\x02\x02\x02\
		\x32\x33\x03\x02\x02\x02\x33\x34\x08\x08\x02\x02\x34\x10\x03\x02\x02\x02\
		\x05\x02\x2b\x31\x03\x02\x03\x02";
