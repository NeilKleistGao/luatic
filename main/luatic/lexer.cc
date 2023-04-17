/*
 * MIT License
 *
 * Copyright (c) 2023 Cunyuan(Holden) Gao
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#include "lexer.h"

#include <utility>

const std::unordered_map<std::string, Keyword> Lexer::m_keywords = {
  {     "and",    Keyword::KW_AND},
  {   "break",  Keyword::KW_BREAK},
  {      "do",     Keyword::KW_DO},
  {    "else",   Keyword::KW_ELSE},
  {  "elseif", Keyword::KW_ELSEIF},
  {     "end",    Keyword::KW_END},
  {   "false",  Keyword::KW_FALSE},
  {     "for",    Keyword::KW_FOR},
  {"function",    Keyword::KW_FUN},
  {    "goto",   Keyword::KW_GOTO},
  {      "if",     Keyword::KW_IF},
  {      "in",     Keyword::KW_IN},
  {   "local",  Keyword::KW_LOCAL},
  {     "nil",    Keyword::KW_NIL},
  {     "not",    Keyword::KW_NOT},
  {      "or",     Keyword::KW_OR},
  {  "repeat", Keyword::LW_REPEAT},
  {  "return", Keyword::KW_RETURN},
  {    "then",   Keyword::KW_THEN},
  {    "true",   Keyword::KW_TRUE},
  {   "until",  Keyword::KW_UNTIL},
  {   "while",  Keyword::KW_WHILE}
};

const std::unordered_map<std::string, Operator> Lexer::m_operators = {
  {  "+",        Operator::OP_PLUS},
  {  "-",       Operator::OP_MINUS},
  {  "*",         Operator::OP_MUL},
  {  "/",         Operator::OP_DIV},
  {  "%",         Operator::OP_MOD},
  {  "^",         Operator::OP_POW},
  {  "#",         Operator::OP_LEN},
  {  "&",         Operator::OP_AND},
  {  "~",         Operator::OP_XOR},
  {  "|",          Operator::OP_OR},
  { "<<",  Operator::OP_LEFT_SHIFT},
  { ">>", Operator::OP_RIGHT_SHIFT},
  { "//",          Operator::OP_FD},
  { "==",          Operator::OP_EQ},
  { "~=",          Operator::OP_NE},
  { "<=",          Operator::OP_LE},
  { ">=",          Operator::OP_GE},
  {  "<",        Operator::OP_LESS},
  {  ">",       Operator::OP_GREAT},
  {  "=",      Operator::OP_ASSIGN},
  {  "(",    Operator::OP_LEFT_PAR},
  {  ")",   Operator::OP_RIGHT_PAR},
  {  "{",    Operator::OP_LEFT_BRA},
  {  "}",   Operator::OP_RIGHT_BRA},
  {  "[",    Operator::OP_LEFT_SQR},
  {  "]",   Operator::OP_RIGHT_SQR},
  {  ";",        Operator::OP_SEMI},
  {  ":",       Operator::OP_COLON},
  {  ",",       Operator::OP_COMMA},
  {  ".",         Operator::OP_DOT},
  { "..",        Operator::OP_DOT2},
  {"...",        Operator::OP_DOT3}
};

Lexer::Lexer(std::optional<std::string> p_filename):
  m_filename(std::move(p_filename)) {}

std::variant<Lexer::TokenStream, Lexer::DiagnosticList>
  Lexer::Parse(const std::string& p_code) const noexcept {}
