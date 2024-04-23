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
	 * Visit a parse tree produced by {@link LuaticParser#global_stat}.
	 * @param ctx the parse tree
	 */
	fn visit_global_stat(&mut self, ctx: &Global_statContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#local_stat}.
	 * @param ctx the parse tree
	 */
	fn visit_local_stat(&mut self, ctx: &Local_statContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#local_in_stat}.
	 * @param ctx the parse tree
	 */
	fn visit_local_in_stat(&mut self, ctx: &Local_in_statContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#full_if_stat}.
	 * @param ctx the parse tree
	 */
	fn visit_full_if_stat(&mut self, ctx: &Full_if_statContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#while_stat}.
	 * @param ctx the parse tree
	 */
	fn visit_while_stat(&mut self, ctx: &While_statContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#do_stat}.
	 * @param ctx the parse tree
	 */
	fn visit_do_stat(&mut self, ctx: &Do_statContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#for_in_stat}.
	 * @param ctx the parse tree
	 */
	fn visit_for_in_stat(&mut self, ctx: &For_in_statContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#func_stat}.
	 * @param ctx the parse tree
	 */
	fn visit_func_stat(&mut self, ctx: &Func_statContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#ret_stat}.
	 * @param ctx the parse tree
	 */
	fn visit_ret_stat(&mut self, ctx: &Ret_statContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#boolean}.
	 * @param ctx the parse tree
	 */
	fn visit_boolean(&mut self, ctx: &BooleanContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#integer}.
	 * @param ctx the parse tree
	 */
	fn visit_integer(&mut self, ctx: &IntegerContext<'input>) { self.visit_children(ctx) }

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

	/**
	 * Visit a parse tree produced by {@link LuaticParser#if_stat}.
	 * @param ctx the parse tree
	 */
	fn visit_if_stat(&mut self, ctx: &If_statContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#elif_stat}.
	 * @param ctx the parse tree
	 */
	fn visit_elif_stat(&mut self, ctx: &Elif_statContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#else_state}.
	 * @param ctx the parse tree
	 */
	fn visit_else_state(&mut self, ctx: &Else_stateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#param_list}.
	 * @param ctx the parse tree
	 */
	fn visit_param_list(&mut self, ctx: &Param_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#array}.
	 * @param ctx the parse tree
	 */
	fn visit_array(&mut self, ctx: &ArrayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#kv_pair}.
	 * @param ctx the parse tree
	 */
	fn visit_kv_pair(&mut self, ctx: &Kv_pairContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#table}.
	 * @param ctx the parse tree
	 */
	fn visit_table(&mut self, ctx: &TableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LuaticParser#lambda}.
	 * @param ctx the parse tree
	 */
	fn visit_lambda(&mut self, ctx: &LambdaContext<'input>) { self.visit_children(ctx) }

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
	 * Visit a parse tree produced by {@link LuaticParser#global_stat}.
	 * @param ctx the parse tree
	 */
		fn visit_global_stat(&mut self, ctx: &Global_statContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#local_stat}.
	 * @param ctx the parse tree
	 */
		fn visit_local_stat(&mut self, ctx: &Local_statContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#local_in_stat}.
	 * @param ctx the parse tree
	 */
		fn visit_local_in_stat(&mut self, ctx: &Local_in_statContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#full_if_stat}.
	 * @param ctx the parse tree
	 */
		fn visit_full_if_stat(&mut self, ctx: &Full_if_statContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#while_stat}.
	 * @param ctx the parse tree
	 */
		fn visit_while_stat(&mut self, ctx: &While_statContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#do_stat}.
	 * @param ctx the parse tree
	 */
		fn visit_do_stat(&mut self, ctx: &Do_statContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#for_in_stat}.
	 * @param ctx the parse tree
	 */
		fn visit_for_in_stat(&mut self, ctx: &For_in_statContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#func_stat}.
	 * @param ctx the parse tree
	 */
		fn visit_func_stat(&mut self, ctx: &Func_statContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#ret_stat}.
	 * @param ctx the parse tree
	 */
		fn visit_ret_stat(&mut self, ctx: &Ret_statContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#boolean}.
	 * @param ctx the parse tree
	 */
		fn visit_boolean(&mut self, ctx: &BooleanContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#integer}.
	 * @param ctx the parse tree
	 */
		fn visit_integer(&mut self, ctx: &IntegerContext<'input>) -> Self::Return {
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

	/**
	 * Visit a parse tree produced by {@link LuaticParser#if_stat}.
	 * @param ctx the parse tree
	 */
		fn visit_if_stat(&mut self, ctx: &If_statContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#elif_stat}.
	 * @param ctx the parse tree
	 */
		fn visit_elif_stat(&mut self, ctx: &Elif_statContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#else_state}.
	 * @param ctx the parse tree
	 */
		fn visit_else_state(&mut self, ctx: &Else_stateContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#param_list}.
	 * @param ctx the parse tree
	 */
		fn visit_param_list(&mut self, ctx: &Param_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#array}.
	 * @param ctx the parse tree
	 */
		fn visit_array(&mut self, ctx: &ArrayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#kv_pair}.
	 * @param ctx the parse tree
	 */
		fn visit_kv_pair(&mut self, ctx: &Kv_pairContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#table}.
	 * @param ctx the parse tree
	 */
		fn visit_table(&mut self, ctx: &TableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LuaticParser#lambda}.
	 * @param ctx the parse tree
	 */
		fn visit_lambda(&mut self, ctx: &LambdaContext<'input>) -> Self::Return {
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

	fn visit_prgm(&mut self, ctx: &PrgmContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_prgm(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_block(&mut self, ctx: &BlockContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_global_stat(&mut self, ctx: &Global_statContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_global_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_local_stat(&mut self, ctx: &Local_statContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_local_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_local_in_stat(&mut self, ctx: &Local_in_statContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_local_in_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_full_if_stat(&mut self, ctx: &Full_if_statContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_full_if_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_while_stat(&mut self, ctx: &While_statContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_while_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_do_stat(&mut self, ctx: &Do_statContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_do_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_for_in_stat(&mut self, ctx: &For_in_statContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_for_in_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_func_stat(&mut self, ctx: &Func_statContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_func_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ret_stat(&mut self, ctx: &Ret_statContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_ret_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_boolean(&mut self, ctx: &BooleanContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_boolean(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_integer(&mut self, ctx: &IntegerContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_integer(self, ctx);
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

	fn visit_if_stat(&mut self, ctx: &If_statContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_if_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_elif_stat(&mut self, ctx: &Elif_statContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_elif_stat(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_else_state(&mut self, ctx: &Else_stateContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_else_state(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_param_list(&mut self, ctx: &Param_listContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_param_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_array(&mut self, ctx: &ArrayContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_array(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_kv_pair(&mut self, ctx: &Kv_pairContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_kv_pair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_table(&mut self, ctx: &TableContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_table(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_lambda(&mut self, ctx: &LambdaContext<'input>){
		let result = <Self as LuaticVisitorCompat>::visit_lambda(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}