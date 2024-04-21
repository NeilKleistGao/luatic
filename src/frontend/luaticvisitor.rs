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
	 * Visit a parse tree produced by {@link LuaticParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_expr(&mut self, ctx: &ExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#stat}.
	 * @param ctx the parse tree
	 */
	fn visit_stat(&mut self, ctx: &StatContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#global_stat}.
	 * @param ctx the parse tree
	 */
	fn visit_global_stat(&mut self, ctx: &Global_statContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#number}.
	 * @param ctx the parse tree
	 */
	fn visit_number(&mut self, ctx: &NumberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#string}.
	 * @param ctx the parse tree
	 */
	fn visit_string(&mut self, ctx: &StringContext<'input>) { self.visit_children(ctx) }

}

pub trait LuaticVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= LuaticParserContextType>{
	/**
	 * Visit a parse tree produced by {@link LuaticParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_expr(&mut self, ctx: &ExprContext<'input>) -> Self::Return {
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
	 * Visit a parse tree produced by {@link LuaticParser#global_stat}.
	 * @param ctx the parse tree
	 */
		fn visit_global_stat(&mut self, ctx: &Global_statContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#number}.
	 * @param ctx the parse tree
	 */
		fn visit_number(&mut self, ctx: &NumberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#string}.
	 * @param ctx the parse tree
	 */
		fn visit_string(&mut self, ctx: &StringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> LuaticVisitor<'input> for T
where
	T: LuaticVisitorCompat<'input>
{
	fn visit_expr(&mut self, ctx: &ExprContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stat(&mut self, ctx: &StatContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_global_stat(&mut self, ctx: &Global_statContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_global_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_number(&mut self, ctx: &NumberContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_string(&mut self, ctx: &StringContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_string(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}