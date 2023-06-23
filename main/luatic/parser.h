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

#include "ast.hpp"
#include "lexer.h"

class Parser {
public:
  using DiagnosticList = std::vector<Diagnostic>;

  Parser(std::optional<std::string> p_filename, Lexer::TokenStream p_tokens);
  [[nodiscard]] std::variant<Block, DiagnosticList> Parse() const noexcept;

  ~Parser() = default;
  Parser(const Parser&) = delete;
  Parser& operator=(const Parser&) = delete;
  Parser(Parser&&) = delete;
  Parser& operator=(Parser&&) = delete;

private:
  std::optional<std::string> m_filename;

  using TokenPointer = Lexer::TokenStream::const_iterator;

  Lexer::TokenStream m_stream;
  TokenPointer m_ending;

  [[nodiscard]] std::variant<Stmt, Diagnostic>
    ParseStatement(TokenPointer p_cur) const noexcept;

  [[nodiscard]] TokenPointer Skip(TokenPointer p_cur) const noexcept;

  [[nodiscard]] std::variant<GotoStmt, Diagnostic>
    ParseGoto(TokenPointer p_cur) const noexcept;
  // TODO:
  // [[nodiscard]] std::variant<DoStmt, Diagnostic> ParseDo(TokenPointer p_cur) const noexcept;

  [[nodiscard]] inline Diagnostic
    RaiseError(Location p_loc, std::string p_info) const noexcept {
    return RaiseErrorByType(DiagnosticType::DIAG_PARSE,
                            p_loc,
                            std::move(p_info),
                            this->m_filename);
  }
};

#endif //LUATIC_PARSER_H
