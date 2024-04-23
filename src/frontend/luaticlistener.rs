#![allow(nonstandard_style)]
// Generated from Luatic.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::luaticparser::*;

pub trait LuaticListener<'input> : ParseTreeListener<'input,LuaticParserContextType>{
/**
 * Enter a parse tree produced by {@link LuaticParser#expr}.
 * @param ctx the parse tree
 */
fn enter_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#expr}.
 * @param ctx the parse tree
 */
fn exit_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#stat}.
 * @param ctx the parse tree
 */
fn enter_stat(&mut self, _ctx: &StatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#stat}.
 * @param ctx the parse tree
 */
fn exit_stat(&mut self, _ctx: &StatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#prgm}.
 * @param ctx the parse tree
 */
fn enter_prgm(&mut self, _ctx: &PrgmContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#prgm}.
 * @param ctx the parse tree
 */
fn exit_prgm(&mut self, _ctx: &PrgmContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#block}.
 * @param ctx the parse tree
 */
fn enter_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#block}.
 * @param ctx the parse tree
 */
fn exit_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#global_stat}.
 * @param ctx the parse tree
 */
fn enter_global_stat(&mut self, _ctx: &Global_statContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#global_stat}.
 * @param ctx the parse tree
 */
fn exit_global_stat(&mut self, _ctx: &Global_statContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#local_stat}.
 * @param ctx the parse tree
 */
fn enter_local_stat(&mut self, _ctx: &Local_statContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#local_stat}.
 * @param ctx the parse tree
 */
fn exit_local_stat(&mut self, _ctx: &Local_statContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#local_in_stat}.
 * @param ctx the parse tree
 */
fn enter_local_in_stat(&mut self, _ctx: &Local_in_statContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#local_in_stat}.
 * @param ctx the parse tree
 */
fn exit_local_in_stat(&mut self, _ctx: &Local_in_statContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#full_if_stat}.
 * @param ctx the parse tree
 */
fn enter_full_if_stat(&mut self, _ctx: &Full_if_statContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#full_if_stat}.
 * @param ctx the parse tree
 */
fn exit_full_if_stat(&mut self, _ctx: &Full_if_statContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#while_stat}.
 * @param ctx the parse tree
 */
fn enter_while_stat(&mut self, _ctx: &While_statContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#while_stat}.
 * @param ctx the parse tree
 */
fn exit_while_stat(&mut self, _ctx: &While_statContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#do_stat}.
 * @param ctx the parse tree
 */
fn enter_do_stat(&mut self, _ctx: &Do_statContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#do_stat}.
 * @param ctx the parse tree
 */
fn exit_do_stat(&mut self, _ctx: &Do_statContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#for_in_stat}.
 * @param ctx the parse tree
 */
fn enter_for_in_stat(&mut self, _ctx: &For_in_statContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#for_in_stat}.
 * @param ctx the parse tree
 */
fn exit_for_in_stat(&mut self, _ctx: &For_in_statContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#func_stat}.
 * @param ctx the parse tree
 */
fn enter_func_stat(&mut self, _ctx: &Func_statContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#func_stat}.
 * @param ctx the parse tree
 */
fn exit_func_stat(&mut self, _ctx: &Func_statContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#ret_stat}.
 * @param ctx the parse tree
 */
fn enter_ret_stat(&mut self, _ctx: &Ret_statContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#ret_stat}.
 * @param ctx the parse tree
 */
fn exit_ret_stat(&mut self, _ctx: &Ret_statContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#boolean}.
 * @param ctx the parse tree
 */
fn enter_boolean(&mut self, _ctx: &BooleanContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#boolean}.
 * @param ctx the parse tree
 */
fn exit_boolean(&mut self, _ctx: &BooleanContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#integer}.
 * @param ctx the parse tree
 */
fn enter_integer(&mut self, _ctx: &IntegerContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#integer}.
 * @param ctx the parse tree
 */
fn exit_integer(&mut self, _ctx: &IntegerContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#number}.
 * @param ctx the parse tree
 */
fn enter_number(&mut self, _ctx: &NumberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#number}.
 * @param ctx the parse tree
 */
fn exit_number(&mut self, _ctx: &NumberContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#string}.
 * @param ctx the parse tree
 */
fn enter_string(&mut self, _ctx: &StringContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#string}.
 * @param ctx the parse tree
 */
fn exit_string(&mut self, _ctx: &StringContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#if_stat}.
 * @param ctx the parse tree
 */
fn enter_if_stat(&mut self, _ctx: &If_statContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#if_stat}.
 * @param ctx the parse tree
 */
fn exit_if_stat(&mut self, _ctx: &If_statContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#elif_stat}.
 * @param ctx the parse tree
 */
fn enter_elif_stat(&mut self, _ctx: &Elif_statContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#elif_stat}.
 * @param ctx the parse tree
 */
fn exit_elif_stat(&mut self, _ctx: &Elif_statContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#else_state}.
 * @param ctx the parse tree
 */
fn enter_else_state(&mut self, _ctx: &Else_stateContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#else_state}.
 * @param ctx the parse tree
 */
fn exit_else_state(&mut self, _ctx: &Else_stateContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#param_list}.
 * @param ctx the parse tree
 */
fn enter_param_list(&mut self, _ctx: &Param_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#param_list}.
 * @param ctx the parse tree
 */
fn exit_param_list(&mut self, _ctx: &Param_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#array}.
 * @param ctx the parse tree
 */
fn enter_array(&mut self, _ctx: &ArrayContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#array}.
 * @param ctx the parse tree
 */
fn exit_array(&mut self, _ctx: &ArrayContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#kv_pair}.
 * @param ctx the parse tree
 */
fn enter_kv_pair(&mut self, _ctx: &Kv_pairContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#kv_pair}.
 * @param ctx the parse tree
 */
fn exit_kv_pair(&mut self, _ctx: &Kv_pairContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#table}.
 * @param ctx the parse tree
 */
fn enter_table(&mut self, _ctx: &TableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#table}.
 * @param ctx the parse tree
 */
fn exit_table(&mut self, _ctx: &TableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#lambda}.
 * @param ctx the parse tree
 */
fn enter_lambda(&mut self, _ctx: &LambdaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#lambda}.
 * @param ctx the parse tree
 */
fn exit_lambda(&mut self, _ctx: &LambdaContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : LuaticListener<'input> }


