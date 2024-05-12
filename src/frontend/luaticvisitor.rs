#![allow(nonstandard_style)]
// Generated from Luatic.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::luaticparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link LuaticParser}.
 */
pub trait LuaticVisitor<'input>: ParseTreeVisitor<'input,LuaticParserContextType>{
	/**
	 * Visit a parse tree produced by {@link LuaticParser#dialog_block}.
	 * @param ctx the parse tree
	 */
	fn visit_dialog_block(&mut self, ctx: &Dialog_blockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#say_stat}.
	 * @param ctx the parse tree
	 */
	fn visit_say_stat(&mut self, ctx: &Say_statContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#stat}.
	 * @param ctx the parse tree
	 */
	fn visit_stat(&mut self, ctx: &StatContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#prgm}.
	 * @param ctx the parse tree
	 */
	fn visit_prgm(&mut self, ctx: &PrgmContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#block}.
	 * @param ctx the parse tree
	 */
	fn visit_block(&mut self, ctx: &BlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#say_block}.
	 * @param ctx the parse tree
	 */
	fn visit_say_block(&mut self, ctx: &Say_blockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#string}.
	 * @param ctx the parse tree
	 */
	fn visit_string(&mut self, ctx: &StringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#character}.
	 * @param ctx the parse tree
	 */
	fn visit_character(&mut self, ctx: &CharacterContext<'input>) { self.visit_children(ctx) }

}

pub trait LuaticVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= LuaticParserContextType>{
	/**
	 * Visit a parse tree produced by {@link LuaticParser#dialog_block}.
	 * @param ctx the parse tree
	 */
		fn visit_dialog_block(&mut self, ctx: &Dialog_blockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#say_stat}.
	 * @param ctx the parse tree
	 */
		fn visit_say_stat(&mut self, ctx: &Say_statContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#stat}.
	 * @param ctx the parse tree
	 */
		fn visit_stat(&mut self, ctx: &StatContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#prgm}.
	 * @param ctx the parse tree
	 */
		fn visit_prgm(&mut self, ctx: &PrgmContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#block}.
	 * @param ctx the parse tree
	 */
		fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#say_block}.
	 * @param ctx the parse tree
	 */
		fn visit_say_block(&mut self, ctx: &Say_blockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#string}.
	 * @param ctx the parse tree
	 */
		fn visit_string(&mut self, ctx: &StringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#character}.
	 * @param ctx the parse tree
	 */
		fn visit_character(&mut self, ctx: &CharacterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> LuaticVisitor<'input> for T
where
	T: LuaticVisitorCompat<'input>
{
	fn visit_dialog_block(&mut self, ctx: &Dialog_blockContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_dialog_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_say_stat(&mut self, ctx: &Say_statContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_say_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stat(&mut self, ctx: &StatContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_prgm(&mut self, ctx: &PrgmContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_prgm(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_block(&mut self, ctx: &BlockContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_say_block(&mut self, ctx: &Say_blockContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_say_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_string(&mut self, ctx: &StringContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_string(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_character(&mut self, ctx: &CharacterContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_character(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}