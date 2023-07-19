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

#ifndef LUATIC_PARSER_H
#define LUATIC_PARSER_H

#include <optional>

#include "ast.h"
#include "lexer.h"

class Parser {
public:
  using DiagnosticList = std::vector<Diagnostic>;

  Parser(std::optional<std::string> p_filename, Lexer::TokenStream p_tokens);
  [[nodiscard]] std::variant<Block, DiagnosticList> Parse() noexcept;

  ~Parser() = default;
  Parser(const Parser&) = delete;
  Parser& operator=(const Parser&) = delete;
  Parser(Parser&&) = delete;
  Parser& operator=(Parser&&) = delete;

private:
  const std::optional<std::string> m_filename;

  using TokenPointer = Lexer::TokenStream::const_iterator;

  const Lexer::TokenStream m_stream;
  const TokenPointer m_ending;
  DiagnosticList m_diags{};

  [[nodiscard]] std::optional<Stmt>
    ParseStatement(TokenPointer& p_cur) noexcept;

  [[nodiscard]] inline std::optional<Expr>
    ParseExpr(TokenPointer& p_cur) noexcept {
    return ParseExpr0(p_cur);
  }

  [[nodiscard]] std::optional<Expr>
    ParseExpr0(TokenPointer& p_cur) noexcept; // or
  [[nodiscard]] std::optional<Expr>
    ParseExpr1(TokenPointer& p_cur) noexcept; // and
  [[nodiscard]] std::optional<Expr>
    ParseExpr2(TokenPointer& p_cur) noexcept; // < > <= >= == ~=
  [[nodiscard]] std::optional<Expr>
    ParseExpr3(TokenPointer& p_cur) noexcept; // |
  [[nodiscard]] std::optional<Expr>
    ParseExpr4(TokenPointer& p_cur) noexcept; // ~
  [[nodiscard]] std::optional<Expr>
    ParseExpr5(TokenPointer& p_cur) noexcept; //&
  [[nodiscard]] std::optional<Expr>
    ParseExpr6(TokenPointer& p_cur) noexcept; // << >>
  [[nodiscard]] std::optional<Expr>
    ParseExpr7(TokenPointer& p_cur) noexcept; // ..
  [[nodiscard]] std::optional<Expr>
    ParseExpr8(TokenPointer& p_cur) noexcept; // + -
  [[nodiscard]] std::optional<Expr>
    ParseExpr9(TokenPointer& p_cur) noexcept; // * / // %
  [[nodiscard]] std::optional<Expr>
    ParseExpr10(TokenPointer& p_cur) noexcept; // unary
  [[nodiscard]] std::optional<Expr>
    ParseExpr11(TokenPointer& p_cur) noexcept; // ^
  [[nodiscard]] std::optional<Expr>
    ParseExpr12(TokenPointer& p_cur) noexcept; // lit/fun/table/...

  [[nodiscard]] std::optional<Expr> MakeUnary(std::optional<Expr>&& p_expr,
                                              const Position& p_begin,
                                              UnaryOperator uop);
  [[nodiscard]] std::optional<Expr> MakeBinary(std::optional<Expr>&& p_lhs,
                                               std::optional<Expr>&& p_rhs,
                                               BinaryOperator bop);

  [[nodiscard]] TokenPointer Skip(TokenPointer p_cur) const noexcept;

  [[nodiscard]] std::optional<GotoStmt> ParseGoto(TokenPointer& p_cur) noexcept;
  [[nodiscard]] std::optional<DoStmt> ParseDo(TokenPointer& p_cur) noexcept;

  [[nodiscard]] std::optional<Block> ParseBlock(TokenPointer& p_cur) noexcept;

  [[nodiscard]] std::optional<FunParams>
    ParseParamsList(TokenPointer& p_cur) noexcept;

  [[nodiscard]] std::optional<FunctionExpr>
    ParseFuncExpression(TokenPointer& p_cur, Position&& p_start) noexcept;

  [[nodiscard]] std::optional<Expr> TryToParseAccess(TokenPointer& p_cur,
                                                     Expr&& p_prev) noexcept;

  template<typename T>
  [[nodiscard]] inline std::optional<T>
    RaiseError(Location p_loc, std::string p_info) noexcept {
    m_diags.push_back(RaiseErrorByType(DiagnosticType::DIAG_PARSE,
                                       p_loc,
                                       std::move(p_info),
                                       this->m_filename));
    return std::nullopt;
  }
};

#endif //LUATIC_PARSER_H
