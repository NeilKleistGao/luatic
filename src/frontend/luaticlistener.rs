#![allow(nonstandard_style)]
// Generated from luatic.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::luaticparser::*;

pub trait luaticListener<'input> : ParseTreeListener<'input,luaticParserContextType>{
/**
 * Enter a parse tree produced by {@link luaticParser#expr}.
 * @param ctx the parse tree
 */
fn enter_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link luaticParser#expr}.
 * @param ctx the parse tree
 */
fn exit_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link luaticParser#stat}.
 * @param ctx the parse tree
 */
fn enter_stat(&mut self, _ctx: &StatContext<'input>) { }
/**
 * Exit a parse tree produced by {@link luaticParser#stat}.
 * @param ctx the parse tree
 */
fn exit_stat(&mut self, _ctx: &StatContext<'input>) { }
/**
 * Enter a parse tree produced by {@link luaticParser#global_stat}.
 * @param ctx the parse tree
 */
fn enter_global_stat(&mut self, _ctx: &Global_statContext<'input>) { }
/**
 * Exit a parse tree produced by {@link luaticParser#global_stat}.
 * @param ctx the parse tree
 */
fn exit_global_stat(&mut self, _ctx: &Global_statContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : luaticListener<'input> }


