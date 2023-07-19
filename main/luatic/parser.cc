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
  auto p = m_stream.begin();
  const auto block = ParseBlock(p);

  if (block.has_value()) {
    return block.value();
  } else {
    return m_diags;
  }
}

#define CASE_KEY(__CUR__)                                                      \
if (__CUR__ != this->m_ending && __CUR__->token.index() == 0)
#define CASE_IDENT(__CUR__)                                                    \
if (__CUR__ != this->m_ending && __CUR__->token.index() == 1)
#define CASE_LIT(__CUR__)                                                      \
if (__CUR__ != this->m_ending && __CUR__->token.index() == 2)
#define CASE_PUNC(__CUR__)                                                     \
if (__CUR__ != this->m_ending && __CUR__->token.index() == 3)

#define GET_KEY(__CUR__, __RES__)                                              \
const auto __RES__ = std::get<Keyword>(__CUR__->token)
#define GET_IDENT(__CUR__, __RES__)                                            \
const auto __RES__ = std::get<Identifier>(__CUR__->token)
#define GET_LIT(__CUR__, __RES__)                                              \
const auto __RES__ = std::get<Literal>(__CUR__->token)
#define GET_PUNC(__CUR__, __RES__)                                             \
const auto __RES__ = std::get<Punctuation>(__CUR__->token)

#define CHECK_EOF(__CUR__, __TYPE__, __INFO__)                                 \
if (__CUR__ == m_ending)                                                       \
return RaiseError<__TYPE__>((__CUR__ - 1)->location, __INFO__)

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
      auto func = FuncStmt{p_cur->location.begin};
      p_cur = Skip(p_cur);
      CASE_IDENT(p_cur) {
        GET_IDENT(p_cur, f_name);
        func.name = f_name.name;
        p_cur = Skip(p_cur);
        auto p_start = p_cur->location.begin;
        const auto body = ParseFuncExpression(p_cur, std::move(p_start));
        if (body.has_value()) {
          func.func = std::make_shared<FunctionExpr>(body.value());
          func.loc.end = func.func->loc.end;
          return Stmt{func};
        } else {
          return std::nullopt;
        }
      }

      return RaiseError<Stmt>(p_cur->location, "expect function name.");
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

std::optional<Expr> Parser::ParseExpr0(TokenPointer& p_cur) noexcept {
  p_cur = Skip(p_cur);
  auto lhs = ParseExpr1(p_cur);
  CASE_KEY(p_cur) {
    GET_KEY(p_cur, kw);
    if (kw == Keyword::KW_OR) {
      p_cur = Skip(p_cur + 1);
      auto rhs = ParseExpr1(p_cur);
      return MakeBinary(std::move(lhs), std::move(rhs), BinaryOperator::OP_OR);
    }
  }

  return lhs;
}

std::optional<Expr> Parser::ParseExpr1(TokenPointer& p_cur) noexcept {
  p_cur = Skip(p_cur);
  auto lhs = ParseExpr2(p_cur);
  CASE_KEY(p_cur) {
    GET_KEY(p_cur, kw);
    if (kw == Keyword::KW_AND) {
      p_cur = Skip(p_cur + 1);
      auto rhs = ParseExpr2(p_cur);
      return MakeBinary(std::move(lhs), std::move(rhs), BinaryOperator::OP_AND);
    }
  }

  return lhs;
}

std::optional<Expr> Parser::ParseExpr2(TokenPointer& p_cur) noexcept {
  p_cur = Skip(p_cur);
  auto lhs = ParseExpr3(p_cur);
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    if (punc == Punctuation::PUN_LESS || punc == Punctuation::PUN_LE ||
        punc == Punctuation::PUN_GREAT || punc == Punctuation::PUN_GE ||
        punc == Punctuation::PUN_EQ || punc == Punctuation::PUN_NE) {
      p_cur = Skip(p_cur + 1);
      auto rhs = ParseExpr3(p_cur);
      if (punc == Punctuation::PUN_LESS) {
        return MakeBinary(std::move(lhs),
                          std::move(rhs),
                          BinaryOperator::OP_LESS);
      } else if (punc == Punctuation::PUN_LE) {
        return MakeBinary(std::move(lhs),
                          std::move(rhs),
                          BinaryOperator::OP_LE);
      } else if (punc == Punctuation::PUN_GREAT) {
        return MakeBinary(std::move(lhs),
                          std::move(rhs),
                          BinaryOperator::OP_GREAT);
      } else if (punc == Punctuation::PUN_GE) {
        return MakeBinary(std::move(lhs),
                          std::move(rhs),
                          BinaryOperator::OP_GE);
      } else if (punc == Punctuation::PUN_EQ) {
        return MakeBinary(std::move(lhs),
                          std::move(rhs),
                          BinaryOperator::OP_EQ);
      } else {
        return MakeBinary(std::move(lhs),
                          std::move(rhs),
                          BinaryOperator::OP_NE);
      }
    }
  }

  return lhs;
}

std::optional<Expr> Parser::ParseExpr3(TokenPointer& p_cur) noexcept {
  p_cur = Skip(p_cur);
  auto lhs = ParseExpr4(p_cur);
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    if (punc == Punctuation::PUN_OR) {
      p_cur = Skip(p_cur + 1);
      auto rhs = ParseExpr4(p_cur);
      return MakeBinary(std::move(lhs),
                        std::move(rhs),
                        BinaryOperator::OP_BIN_OR);
    }
  }

  return lhs;
}

std::optional<Expr> Parser::ParseExpr4(TokenPointer& p_cur) noexcept {
  p_cur = Skip(p_cur);
  auto lhs = ParseExpr5(p_cur);
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    if (punc == Punctuation::PUN_XOR) {
      p_cur = Skip(p_cur + 1);
      auto rhs = ParseExpr5(p_cur);
      return MakeBinary(std::move(lhs), std::move(rhs), BinaryOperator::OP_XOR);
    }
  }

  return lhs;
}

std::optional<Expr> Parser::ParseExpr5(TokenPointer& p_cur) noexcept {
  p_cur = Skip(p_cur);
  auto lhs = ParseExpr6(p_cur);
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    if (punc == Punctuation::PUN_AND) {
      p_cur = Skip(p_cur + 1);
      auto rhs = ParseExpr6(p_cur);
      return MakeBinary(std::move(lhs),
                        std::move(rhs),
                        BinaryOperator::OP_BIN_AND);
    }
  }

  return lhs;
}

std::optional<Expr> Parser::ParseExpr6(TokenPointer& p_cur) noexcept {
  p_cur = Skip(p_cur);
  auto lhs = ParseExpr7(p_cur);
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    if (punc == Punctuation::PUN_LEFT_SHIFT ||
        punc == Punctuation::PUN_RIGHT_SHIFT) {
      p_cur = Skip(p_cur + 1);
      auto rhs = ParseExpr7(p_cur);
      if (punc == Punctuation::PUN_LEFT_SHIFT) {
        return MakeBinary(std::move(lhs),
                          std::move(rhs),
                          BinaryOperator::OP_LEFT_SHIFT);
      } else {
        return MakeBinary(std::move(lhs),
                          std::move(rhs),
                          BinaryOperator::OP_RIGHT_SHIFT);
      }
    }
  }

  return lhs;
}

std::optional<Expr> Parser::ParseExpr7(TokenPointer& p_cur) noexcept {
  p_cur = Skip(p_cur);
  auto lhs = ParseExpr8(p_cur);
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    if (punc == Punctuation::PUN_DOT2) {
      p_cur = Skip(p_cur + 1);
      auto rhs = ParseExpr8(p_cur);
      return MakeBinary(std::move(lhs), std::move(rhs), BinaryOperator::OP_CAT);
    }
  }

  return lhs;
}

std::optional<Expr> Parser::ParseExpr8(TokenPointer& p_cur) noexcept {
  p_cur = Skip(p_cur);
  auto lhs = ParseExpr9(p_cur);
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    if (punc == Punctuation::PUN_PLUS || punc == Punctuation::PUN_MINUS) {
      p_cur = Skip(p_cur + 1);
      auto rhs = ParseExpr9(p_cur);
      if (punc == Punctuation::PUN_PLUS) {
        return MakeBinary(std::move(lhs),
                          std::move(rhs),
                          BinaryOperator::OP_ADD);
      } else {
        return MakeBinary(std::move(lhs),
                          std::move(rhs),
                          BinaryOperator::OP_SUB);
      }
    }
  }

  return lhs;
}

std::optional<Expr> Parser::ParseExpr9(TokenPointer& p_cur) noexcept {
  p_cur = Skip(p_cur);
  auto lhs = ParseExpr10(p_cur);
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    if (punc == Punctuation::PUN_MUL || punc == Punctuation::PUN_DIV ||
        punc == Punctuation::PUN_FD || punc == Punctuation::PUN_MOD) {
      p_cur = Skip(p_cur + 1);
      auto rhs = ParseExpr10(p_cur);
      if (punc == Punctuation::PUN_MUL) {
        return MakeBinary(std::move(lhs),
                          std::move(rhs),
                          BinaryOperator::OP_MUL);
      } else if (punc == Punctuation::PUN_DIV) {
        return MakeBinary(std::move(lhs),
                          std::move(rhs),
                          BinaryOperator::OP_DIV);
      } else if (punc == Punctuation::PUN_FD) {
        return MakeBinary(std::move(lhs),
                          std::move(rhs),
                          BinaryOperator::OP_FD);
      } else {
        return MakeBinary(std::move(lhs),
                          std::move(rhs),
                          BinaryOperator::OP_MOD);
      }
    }
  }

  return lhs;
}

std::optional<Expr> Parser::ParseExpr10(TokenPointer& p_cur) noexcept {
  p_cur = Skip(p_cur);
  const auto loc = p_cur->location;
  CASE_KEY(p_cur) {
    GET_KEY(p_cur, kw);
    if (kw == Keyword::KW_NOT) {
      p_cur = Skip(p_cur + 1);
      auto exp = ParseExpr11(p_cur);
      return MakeUnary(std::move(exp), loc.begin, UnaryOperator::OP_NOT);
    }
  }
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    if (punc == Punctuation::PUN_MINUS) {
      p_cur = Skip(p_cur + 1);
      auto exp = ParseExpr11(p_cur);
      return MakeUnary(std::move(exp), loc.begin, UnaryOperator::OP_NEG);
    } else if (punc == Punctuation::PUN_LEN) {
      p_cur = Skip(p_cur + 1);
      auto exp = ParseExpr11(p_cur);
      return MakeUnary(std::move(exp), loc.begin, UnaryOperator::OP_LEN);
    } else if (punc == Punctuation::PUN_XOR) {
      p_cur = Skip(p_cur + 1);
      auto exp = ParseExpr11(p_cur);
      return MakeUnary(std::move(exp), loc.begin, UnaryOperator::OP_BIN_NOT);
    }
  }

  return ParseExpr11(p_cur);
}

std::optional<Expr> Parser::ParseExpr11(TokenPointer& p_cur) noexcept {
  p_cur = Skip(p_cur);
  auto lhs = ParseExpr12(p_cur);
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    if (punc == Punctuation::PUN_POW) {
      p_cur = Skip(p_cur + 1);
      auto rhs = ParseExpr10(p_cur);
      return MakeBinary(std::move(lhs), std::move(rhs), BinaryOperator::OP_POW);
    } else if (lhs.has_value()) {
      auto temp = lhs.value();
      return TryToParseAccess(p_cur, std::move(temp));
    }
  }

  return lhs;
}

std::optional<Expr> Parser::ParseExpr12(TokenPointer& p_cur) noexcept {
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
    } else if (kw == Keyword::KW_FUN) {
      auto p_start = p_cur->location.begin;
      p_cur = Skip(p_cur);
      const auto res = ParseFuncExpression(p_cur, std::move(p_start));
      if (res.has_value()) {
        return Expr{res.value()};
      } else {
        return std::nullopt;
      }
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
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    if (punc == Punctuation::PUN_DOT3) {
      auto res = VarArgExpr{p_cur->location.begin};
      res.loc.end = p_cur->location.end;
      return Expr{res};
    } else if (punc == Punctuation::PUN_LEFT_PAR) {
      p_cur = Skip(p_cur);
      auto res = ParseExpr(p_cur);
      p_cur = Skip(p_cur);
      CASE_PUNC(p_cur) {
        GET_PUNC(p_cur, punc);
        if (punc == Punctuation::PUN_RIGHT_PAR) {
          return res;
        } else {
          return RaiseError<Expr>(p_cur->location, "expect ')'.");
        }
      }
    }
  }

  CASE_IDENT(p_cur) {
    GET_IDENT(p_cur, id);
    auto var = VarExpr{p_cur->location.begin};
    var.name = id.name;
    var.loc.end = p_cur->location.end;
    return Expr{var};
  }

  return RaiseError<Expr>(p_cur->location,
                          "wrong expression."); // TODO: improve
}

std::optional<Expr> Parser::TryToParseAccess(TokenPointer& p_cur,
                                             Expr&& p_prev) noexcept {
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    if (punc == Punctuation::PUN_DOT || punc == Punctuation::PUN_COLON ||
        punc == Punctuation::PUN_LEFT_SQR) {
      auto lhs = std::make_shared<Expr>(std::move(p_prev));
      auto acc = AccessExpr{GetLocation(*lhs).begin};

      if (punc == Punctuation::PUN_LEFT_SQR) {
        acc.type = AccessExpr::AccessType::ACC_IDX;
        p_cur = Skip(p_cur + 1);

        auto expr = ParseExpr(p_cur);
        if (expr.has_value()) {
          acc.rhs = std::make_shared<Expr>(expr.value());
          CASE_PUNC(p_cur) {
            GET_PUNC(p_cur, end);
            if (end == Punctuation::PUN_RIGHT_SQR) {
              acc.loc.end = p_cur->location.end;
              ++p_cur;
              return TryToParseAccess(p_cur, Expr{acc});
            }
          }

          return RaiseError<Expr>(p_cur->location, "expect ']'.");
        } else {
          return std::nullopt;
        }
      } else {
        if (punc == Punctuation::PUN_DOT) {
          acc.type = AccessExpr::AccessType::ACC_DOT;
        } else {
          acc.type = AccessExpr::AccessType::ACC_COL;
        }

        ++p_cur;
        CASE_IDENT(p_cur) {
          GET_IDENT(p_cur, field);
          auto var = VarExpr{p_cur->location.begin};
          var.loc.end = p_cur->location.end;
          var.name = field.name;
          acc.rhs = std::make_shared<Expr>(std::move(var));
          return TryToParseAccess(p_cur, Expr{acc});
        }

        return RaiseError<Expr>(p_cur->location,
                                "expect field name after " +
                                  std::to_string(punc) + ".");
      }
    }
  }

  return std::move(p_prev);
}

std::optional<FunctionExpr>
  Parser::ParseFuncExpression(TokenPointer& p_cur,
                              Position&& p_start) noexcept {
  auto expr = FunctionExpr{std::move(p_start)};
  auto params = ParseParamsList(p_cur);
  auto block = ParseBlock(p_cur);
  if (params.has_value() && block.has_value()) {
    expr.params = params.value();
    expr.body = std::make_shared<Block>(block.value());
    expr.loc.end = p_cur->location.end;

    return expr;
  }

  return std::nullopt;
}

std::optional<Expr> Parser::MakeUnary(std::optional<Expr>&& p_expr,
                                      const Position& p_begin,
                                      UnaryOperator uop) {
  if (p_expr.has_value()) {
    auto res = UnaryExpr{p_begin};
    res.expr = std::make_shared<Expr>(p_expr.value());
    res.op = uop;
    res.loc.end = GetLocation(p_expr.value()).end;
    return Expr{res};
  }

  return std::nullopt;
}

std::optional<Expr> Parser::MakeBinary(std::optional<Expr>&& p_lhs,
                                       std::optional<Expr>&& p_rhs,
                                       BinaryOperator bop) {
  if (p_lhs.has_value() && p_rhs.has_value()) {
    auto res = BinaryExpr{GetLocation(p_lhs.value()).begin};
    res.lhs = std::make_shared<Expr>(p_lhs.value());
    res.rhs = std::make_shared<Expr>(p_rhs.value());
    res.op = bop;
    res.loc.end = GetLocation(p_rhs.value()).end;
    return Expr{res};
  }

  return std::nullopt;
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

std::optional<FunParams> Parser::ParseParamsList(TokenPointer& p_cur) noexcept {
  CHECK_EOF(p_cur, FunParams, "parameter list expected.");

  p_cur = Skip(p_cur);
  CASE_PUNC(p_cur) {
    GET_PUNC(p_cur, punc);
    if (punc == Punctuation::PUN_LEFT_PAR) {
      p_cur = Skip(p_cur + 1);

      CASE_PUNC(p_cur) {
        GET_PUNC(p_cur, punc);
        if (punc == Punctuation::PUN_RIGHT_PAR) {
          ++p_cur;
          return FunParams{};
        }
      }

      std::vector<Identifier> list;
      bool is_var = false;

      while (p_cur != m_ending) {
        CASE_IDENT(p_cur) {
          GET_IDENT(p_cur, id);
          list.push_back(id);

          p_cur = Skip(p_cur + 1);
          CASE_PUNC(p_cur) {
            GET_PUNC(p_cur, punc);
            if (punc == Punctuation::PUN_COMMA) {
              p_cur = Skip(p_cur + 1);
            } else if (punc == Punctuation::PUN_RIGHT_PAR) {
              ++p_cur;
              break;
            }
          }
        }

        CASE_PUNC(p_cur) {
          GET_PUNC(p_cur, punc);
          if (punc == Punctuation::PUN_DOT3) {
            p_cur = Skip(p_cur + 1);
            CASE_PUNC(p_cur) {
              GET_PUNC(p_cur, punc);
              if (punc == Punctuation::PUN_RIGHT_PAR) {
                if (!is_var) {
                  is_var = true;
                } else {
                  return RaiseError<FunParams>(p_cur->location,
                                               "multiple `...` notations.");
                }
                ++p_cur;
                break;
              }
            }
          }
        }

        return RaiseError<FunParams>(p_cur->location,
                                     "expect parameter name or `...`.");
      }

      return FunParams{std::move(list), is_var};
    }

    return RaiseError<FunParams>(p_cur->location, "parameter list expected.");
  }

  return RaiseError<FunParams>(p_cur->location, "parameter list expected.");
}

std::optional<GotoStmt> Parser::ParseGoto(TokenPointer& p_cur) noexcept {
  CHECK_EOF(p_cur, GotoStmt, "goto label is missing.");

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
  CHECK_EOF(p_cur, DoStmt, "unexpected keyword do.");

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

std::optional<Block> Parser::ParseBlock(TokenPointer& p_cur) noexcept {
  Block block{p_cur->location.begin};
  bool err = false;

  while (p_cur != m_ending) {
    p_cur = Skip(p_cur + 1);
    auto res = ParseStatement(p_cur);
    if (res.has_value()) {
      block.stmts.push_back(res.value());
    } else {
      err = true;
    }
  }

  if (p_cur == m_ending) {
    block.loc.end = m_stream.back().location.end;
  } else {
    block.loc.end = p_cur->location.end;
  }

  if (err) {
    return std::nullopt;
  } else {
    return block;
  }
}

#undef CHECK_EOF
#undef GET_KEY
#undef GET_IDENT
#undef GET_LIT
#undef GET_PUNC
#undef CASE_PUNC
#undef CASE_LIT
#undef CASE_IDENT
#undef CASE_KEY
