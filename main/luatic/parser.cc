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

#include "parser.h"

#include <utility>

Parser::Parser(std::optional<std::string> p_filename):
  m_filename(std::move(p_filename)) {}

std::variant<Block, Parser::DiagnosticList>
  Parser::Parse(const Lexer::TokenStream& p_tokens) const noexcept {
  DiagnosticList diags{};
  Block block{Location::Begin()};

  for (auto p = p_tokens.cbegin(); p != p_tokens.cend(); ++p) {
    const auto res = ParseStatement(p);
    if (res.index() == 0) {
      auto stmt = std::get<0>(res);
      block.stmts.push_back(std::move(stmt));
    } else {
      auto diag = std::get<1>(res);
      diags.push_back(std::move(diag));
    }
  }

  if (!p_tokens.empty()) {
    block.loc.end = p_tokens.back().location.end;
  }

  if (diags.empty()) {
    return block;
  } else {
    return diags;
  }
}

#define CASE_KEY(__CUR__) if (__CUR__->token.index() == 0)
#define CASE_IDENT(__CUR__) if (__CUR__->token.index() == 1)
#define CASE_LIT(__CUR__) if (__CUR__->token.index() == 2)
#define CASE_PUNC(__CUR__) if (__CUR__->token.index() == 3)

#define GET_KEY(__CUR__, __RES__)                                              \
const auto __RES__ = std::get<0>(__CUR__->token)
#define GET_IDENT(__CUR__, __RES__)                                            \
const auto __RES__ = std::get<1>(__CUR__->token)
#define GET_LIT(__CUR__, __RES__)                                              \
const auto __RES__ = std::get<2>(__CUR__->token)
#define GET_PUNC(__CUR__, __RES__)                                             \
const auto __RES__ = std::get<3>(__CUR__->token)

std::variant<Stmt, Diagnostic>
  Parser::ParseStatement(TokenPointer p_cur) const noexcept {
  CASE_KEY(p_cur) {
    GET_KEY(p_cur, kw);
    return Stmt{EmptyStmt{p_cur->location.begin}};
  }
  CASE_IDENT(p_cur) {
    GET_IDENT(p_cur, id);
    return Stmt{EmptyStmt{p_cur->location.begin}};
  }
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    return Stmt{EmptyStmt{p_cur->location.begin}};
  }

  return RaiseError(p_cur->location, "unexpected literal symbol.");
}

#undef GET_KEY
#undef GET_IDENT
#undef GET_LIT
#undef GET_PUNC
#undef CASE_PUNC
#undef CASE_LIT
#undef CASE_IDENT
#undef CASE_KEY
