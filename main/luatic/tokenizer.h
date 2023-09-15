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

#ifndef LUATIC_TOKENIZER_H
#define LUATIC_TOKENIZER_H

#include <memory>
#include <unordered_map>
#include <vector>

#include "diagnostic.hpp"
#include "tokens.hpp"

class Tokenizer {
public:
  using TokenStream = std::vector<std::shared_ptr<Token>>;
  using DiagnosticList = std::vector<Diagnostic>;

  Tokenizer(std::optional<std::string> p_filename, std::string p_code);
  [[nodiscard]] std::variant<TokenStream, DiagnosticList> Parse() noexcept;

  ~Tokenizer() = default;
  Tokenizer(const Tokenizer&) = delete;
  Tokenizer& operator=(const Tokenizer&) = delete;
  Tokenizer(Tokenizer&&) = delete;
  Tokenizer& operator=(Tokenizer&&) = delete;

private:
  const std::optional<std::string> m_filename;
  const std::string m_code;
  const size_t m_length;
  int m_pos, m_line, m_current_line_start;

  TokenStream m_tokens{};
  DiagnosticList m_diags{};

  void ParseOne() noexcept;
  void ParseNumber() noexcept;

  inline void
    RaiseError(Position p_begin, Position p_end, std::string p_info) noexcept {
    m_diags.emplace_back(DiagnosticType::DIAG_LEX,
                         Location{std::move(p_begin), std::move(p_end)},
                         std::move(p_info),
                         this->m_filename);
  }
};

#endif //LUATIC_TOKENIZER_H
