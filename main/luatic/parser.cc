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

Parser::Parser(std::optional<std::string> p_filename,
               Lexer::TokenStream p_tokens):
  m_filename(std::move(p_filename)),
  m_stream(std::move(p_tokens)), m_ending(m_stream.cend()) {}

std::variant<Block, Parser::DiagnosticList> Parser::Parse() noexcept {
  Block block{Location::Begin()};

  for (auto p = m_stream.cbegin(); p != m_ending; ++p) {
    auto res = ParseStatement(p);
    if (res.has_value()) {
      block.stmts.push_back(res.value());
    }
  }

  if (!m_stream.empty()) {
    block.loc.end = m_stream.back().location.end;
  }

  if (m_diags.empty()) {
    return block;
  } else {
    return m_diags;
  }
}

#define CASE_KEY(__CUR__) if (__CUR__->token.index() == 0)
#define CASE_IDENT(__CUR__) if (__CUR__->token.index() == 1)
#define CASE_LIT(__CUR__) if (__CUR__->token.index() == 2)
#define CASE_PUNC(__CUR__) if (__CUR__->token.index() == 3)

#define GET_KEY(__CUR__, __RES__)                                              \
const auto __RES__ = std::get<Keyword>(__CUR__->token)
#define GET_IDENT(__CUR__, __RES__)                                            \
const auto __RES__ = std::get<Identifier>(__CUR__->token)
#define GET_LIT(__CUR__, __RES__)                                              \
const auto __RES__ = std::get<Literal>(__CUR__->token)
#define GET_PUNC(__CUR__, __RES__)                                             \
const auto __RES__ = std::get<Punctuation>(__CUR__->token)

std::optional<Stmt> Parser::ParseStatement(TokenPointer& p_cur) noexcept {
  CASE_KEY(p_cur) {
    GET_KEY(p_cur, kw);
    if (kw == Keyword::KW_BREAK) {
      return Stmt{BreakStmt{p_cur->location.begin}};
    } else if (kw == Keyword::KW_GOTO) {
      return std::optional<Stmt>{ParseGoto(++p_cur)};
    } else if (kw == Keyword::KW_DO) {
      return std::optional<Stmt>{ParseDo(++p_cur)};
    } else if (kw == Keyword::KW_WHILE) {
    } else if (kw == Keyword::KW_REPEAT) {
    } else if (kw == Keyword::KW_IF) {
    } else if (kw == Keyword::KW_FOR) {
    } else if (kw == Keyword::KW_FUN) {
    } else if (kw == Keyword::KW_LOCAL) {
    }
    return RaiseError<Stmt>(p_cur->location,
                            "unexpected " + std::to_string(kw) + ".");
  }
  CASE_IDENT(p_cur) {
    GET_IDENT(p_cur, id);
    return Stmt{EmptyStmt{p_cur->location.begin}};
  }
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    if (punc == Punctuation::PUN_SEMI || punc == Punctuation::PUN_SPACE) {
      return Stmt{EmptyStmt{p_cur->location.begin}};
    }

    return RaiseError<Stmt>(p_cur->location, "unexpected punctuation.");
  }

  return RaiseError<Stmt>(p_cur->location, "unexpected literal symbol.");
}

std::optional<Expr> Parser::ParseExpr(TokenPointer& p_cur) noexcept {
  CASE_KEY(p_cur) {
    GET_KEY(p_cur, kw);
    if (kw == Keyword::KW_NIL) {
      auto expr = NilExpr{p_cur->location.begin};
      expr.loc.end = p_cur->location.end;
      return Expr{expr};
    } else if (kw == Keyword::KW_TRUE || kw == Keyword::KW_FALSE) {
      auto expr = BoolExpr{p_cur->location.begin};
      expr.loc.end = p_cur->location.end;
      expr.value = kw == Keyword::KW_TRUE;
      return Expr{expr};
    }
  }
  CASE_LIT(p_cur) {
    GET_LIT(p_cur, lit);
    if (lit.index() == 0) {
      auto res = StringExpr{p_cur->location.begin};
      res.loc.end = p_cur->location.end;
      res.value = std::get<std::string>(lit);
      return Expr{res};
    } else if (lit.index() == 1) {
      auto res = IntExpr{p_cur->location.begin};
      res.loc.end = p_cur->location.end;
      res.value = std::get<long long>(lit);
      return Expr{res};
    } else {
      auto res = FloatExpr{p_cur->location.begin};
      res.loc.end = p_cur->location.end;
      res.value = std::get<double>(lit);
      return Expr{res};
    }
  }

  return RaiseError<Expr>(p_cur->location,
                          "wrong expression."); // TODO: improve
}

Parser::TokenPointer Parser::Skip(TokenPointer p_cur) const noexcept {
  while (p_cur != m_ending) {
    CASE_PUNC(p_cur) {
      GET_PUNC(p_cur, punc);
      if (punc == Punctuation::PUN_SPACE) {
        ++p_cur;
        continue;
      }
    }

    break;
  }

  return p_cur;
}

std::optional<GotoStmt> Parser::ParseGoto(TokenPointer& p_cur) noexcept {
  if (p_cur == m_ending) {
    return RaiseError<GotoStmt>(p_cur->location, "goto label is missing.");
  }

  auto stmt = GotoStmt{p_cur->location.begin};
  p_cur = Skip(p_cur);

  CASE_IDENT(p_cur) {
    GET_IDENT(p_cur, id);
    stmt.name = id.name;
    stmt.loc.end = p_cur->location.end;
    return stmt;
  }

  return RaiseError<GotoStmt>(p_cur->location, "unexpected goto label.");
}

std::optional<DoStmt> Parser::ParseDo(TokenPointer& p_cur) noexcept {
  if (p_cur == m_ending) {
    return RaiseError<DoStmt>(p_cur->location, "unexpected keyword do.");
  }
  auto stmt = DoStmt{p_cur->location.begin};
  p_cur = Skip(p_cur);
  stmt.block = std::make_shared<Block>(p_cur->location.begin);
  while (p_cur != m_ending) {
    p_cur = Skip(p_cur);
    CASE_KEY(p_cur) {
      GET_KEY(p_cur, kw);
      if (kw == Keyword::KW_END) {
        stmt.loc.end = p_cur->location.end;
        return stmt;
      }
    }

    auto s = ParseStatement(p_cur);
    if (s.has_value()) {
      stmt.block->stmts.push_back(s.value());
    }
    stmt.block->loc.end = p_cur->location.end;

    if (p_cur == m_ending) {
      break;
    }
    ++p_cur;
  }

  return RaiseError<DoStmt>(p_cur->location, "the do block is not finished.");
}

#undef GET_KEY
#undef GET_IDENT
#undef GET_LIT
#undef GET_PUNC
#undef CASE_PUNC
#undef CASE_LIT
#undef CASE_IDENT
#undef CASE_KEY
