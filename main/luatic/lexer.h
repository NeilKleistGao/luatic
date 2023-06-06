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

#ifndef LUATIC_LEXER_H
#define LUATIC_LEXER_H

#include <unordered_map>
#include <vector>

#include "diagnostic.hpp"
#include "tokens.hpp"

class Lexer {
public:
  using TokenStream = std::vector<Token>;
  using DiagnosticList = std::vector<Diagnostic>;

  explicit Lexer(std::optional<std::string> p_filename);
  [[nodiscard]] std::variant<TokenStream, DiagnosticList>
    Parse(const std::string& p_code) const noexcept;

private:
  static const std::unordered_map<std::string, Keyword> m_keywords;
  static const std::unordered_map<std::string, Punctuation> m_punctuations;
  const std::optional<std::string> m_filename;

  std::variant<Token, Diagnostic> Parse(const std::string& p_code,
                                        int& p_pos,
                                        int& p_line,
                                        int& p_start) const noexcept;
  std::variant<Literal, Diagnostic> ParseNumber(const std::string& p_code,
                                                const int& p_start,
                                                int& p_pos,
                                                int p_line) const noexcept;

  [[nodiscard]] inline Diagnostic
    RaiseError(int p_line, int p_col, std::string p_info) const noexcept {
    return RaiseErrorByType(DiagnosticType::DIAG_LEX,
                            Location(p_line, p_col, m_filename),
                            std::move(p_info));
  }

  std::variant<std::string, Diagnostic>
    ParseMultipleLineBlock(const std::string& p_code,
                           int& p_pos,
                           int& p_line,
                           int& p_start) const noexcept;

  std::variant<Token, Diagnostic> ParseComment(const std::string& p_code,
                                               int& p_pos,
                                               int& p_line,
                                               int& p_start) const noexcept;
};

#endif //LUATIC_LEXER_H
