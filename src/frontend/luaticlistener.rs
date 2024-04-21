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

}

antlr_rust::coerce_from!{ 'input : LuaticListener<'input> }


