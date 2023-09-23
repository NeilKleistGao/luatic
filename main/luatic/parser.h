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

#include "codegen.h"
#include "tokenizer.h"

class Parser {
public:
  using TokenStream = Tokenizer::TokenStream;
  using DiagnosticList = Tokenizer::DiagnosticList;

  Parser(std::optional<std::string> p_filename, TokenStream&& p_tokens);
  [[nodiscard]] std::variant<Program, DiagnosticList> Parse() noexcept;

  ~Parser() = default;
  Parser(const Parser&) = delete;
  Parser(Parser&&) = delete;
  Parser& operator=(const Parser&) = delete;
  Parser& operator=(Parser&&) = delete;

private:
  using Iterator = TokenStream::iterator;

  const std::optional<std::string> m_filename;
  TokenStream m_tokens;
  Iterator m_cur;
  Program m_prgm{};
  DiagnosticList m_diags{};

  std::shared_ptr<Expression> ParseExpression() noexcept;
  std::shared_ptr<Call> ParseCall() noexcept;
  std::shared_ptr<Definition> ParseDefinition() noexcept;

  inline bool End() const noexcept { return m_cur == m_tokens.end(); }

  inline void
    RaiseError(Position p_begin, Position p_end, std::string p_info) noexcept {
    m_diags.emplace_back(DiagnosticType::DIAG_PARSE,
                         Location{std::move(p_begin), std::move(p_end)},
                         std::move(p_info),
                         this->m_filename);
  }
};

#endif // LUATIC_PARSER_H
