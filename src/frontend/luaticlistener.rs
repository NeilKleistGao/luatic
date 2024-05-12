#![allow(nonstandard_style)]
// Generated from Luatic.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::luaticparser::*;

pub trait LuaticListener<'input> : ParseTreeListener<'input,LuaticParserContextType>{
/**
 * Enter a parse tree produced by {@link LuaticParser#dialog_block}.
 * @param ctx the parse tree
 */
fn enter_dialog_block(&mut self, _ctx: &Dialog_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#dialog_block}.
 * @param ctx the parse tree
 */
fn exit_dialog_block(&mut self, _ctx: &Dialog_blockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LuaticParser#say_stat}.
 * @param ctx the parse tree
 */
fn enter_say_stat(&mut self, _ctx: &Say_statContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#say_stat}.
 * @param ctx the parse tree
 */
fn exit_say_stat(&mut self, _ctx: &Say_statContext<'input>) { }
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
 * Enter a parse tree produced by {@link LuaticParser#say_block}.
 * @param ctx the parse tree
 */
fn enter_say_block(&mut self, _ctx: &Say_blockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#say_block}.
 * @param ctx the parse tree
 */
fn exit_say_block(&mut self, _ctx: &Say_blockContext<'input>) { }
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
 * Enter a parse tree produced by {@link LuaticParser#character}.
 * @param ctx the parse tree
 */
fn enter_character(&mut self, _ctx: &CharacterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LuaticParser#character}.
 * @param ctx the parse tree
 */
fn exit_character(&mut self, _ctx: &CharacterContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : LuaticListener<'input> }


