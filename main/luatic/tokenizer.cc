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

#include "tokenizer.h"

#include <cctype>
#include <utility>

const std::unordered_map<std::string, Keyword> Tokenizer::m_keywords = {
  // TODO:
};

const std::unordered_map<char, Punctuation> Tokenizer::m_punctuations = {
  {'(',  Punctuation::PUN_LEFT_PAR},
  {')', Punctuation::PUN_RIGHT_PAR},
};

Tokenizer::Tokenizer(std::optional<std::string> p_filename, std::string p_code):
  m_filename(std::move(p_filename)), m_code(std::move(p_code)),
  m_length(m_code.length()) {}

std::variant<Tokenizer::TokenStream, Tokenizer::DiagnosticList>
  Tokenizer::Parse() noexcept {
  TokenStream tokens{};
  DiagnosticList diags{};

  m_pos = 0;
  m_line = 1;
  m_current_line_start = 0;
  while (m_pos < m_length) {
    auto res = ParseOne();
    if (res.index() == 0 && diags.empty()) {
      tokens.push_back(std::move(std::get<Token>(res)));
    } else if (res.index() == 1) {
      diags.push_back(std::move(std::get<Diagnostic>(res)));
    }
  }

  if (diags.empty()) {
    return tokens;
  } else {
    return diags;
  }
}

std::variant<Token, Diagnostic> Tokenizer::ParseOne() noexcept {
  const char head = m_code[m_pos];
  if (std::isspace(head)) {
    const int line = m_line;
    if (head == '\n') {
      ++m_line;
      m_current_line_start = m_pos + 1;
    }
    return Token(Punctuation::PUN_SPACE, Locate(line, m_pos++));
  } else if (std::isdigit(head) ||
             (head == '.' && m_pos + 1 < m_length &&
              std::isdigit(m_code[m_pos + 1]))) {
    const int start = m_pos;
    auto res = ParseNumber();
    if (res.index() == 0) {
      return Token(std::get<Literal>(res),
                   Locate(m_line,
                          start - m_current_line_start,
                          m_pos - m_current_line_start));
    } else {
      return std::get<Diagnostic>(res);
    }
  } else if (head == '"') {
    const int start = m_pos++;
    while (m_pos < m_length && m_code[m_pos] != '"') {
      ++m_pos;
    }

    if (m_pos == m_length) {
      return RaiseError(Locate(m_line,
                               start - m_current_line_start,
                               m_pos - m_current_line_start),
                        "unexpected end of string.");
    }

    ++m_pos;
    return Token(Literal(m_code.substr(start, m_pos - start)),
                 Locate(m_line,
                        start - m_current_line_start,
                        m_pos - m_current_line_start));
  } else if (m_punctuations.find(head) != m_punctuations.end()) {
    return Token(m_punctuations.at(head), Locate(m_line, m_pos++));
  } else if (head == ';') {
    const int start = m_pos++;
    while (m_pos < m_length && m_code[m_pos] != '\n') {
      ++m_pos;
    }

    return Token(Punctuation::PUN_SPACE, Locate(m_line, start));
  } else {
    int start = m_pos++;
    while (m_pos < m_length && m_code[m_pos] != ';' &&
           !std::isspace(m_code[m_pos]) &&
           m_punctuations.find(m_code[m_pos]) == m_punctuations.end()) {
      ++m_pos;
    }

    if (m_pos != m_length) {
      ++m_pos;
    }

    const auto id = m_code.substr(start, m_pos - start);
    if (m_keywords.find(id) == m_keywords.end()) {
      return Token(Identifier(id),
                   Locate(m_line,
                          start - m_current_line_start,
                          m_pos - m_current_line_start));
    } else {
      return Token(Keyword(m_keywords.at(id)),
                   Locate(m_line,
                          start - m_current_line_start,
                          m_pos - m_current_line_start));
    }
  }

  return RaiseError(Locate(0, 0), std::string{"unexpected error."});
}

std::variant<Literal, Diagnostic> Tokenizer::ParseNumber() noexcept {
  bool science = false, hex = false, point = false;

  const int start = m_pos;
  char prev = 0;
  while (m_pos < m_length) {
    const char c = m_code[m_pos];
    if (std::isdigit(c)) {
      ++m_pos;
    } else if (((c >= 'A' && c <= 'F') || (c >= 'a' && c <= 'f')) && hex &&
               !science) {
      ++m_pos;
    } else if ((c == 'x' || c == 'X') && prev == '0' && m_pos - 1 == start) {
      ++m_pos;
      hex = true;
    } else if (c == '.' &&
               (!point &&
                (m_pos + 1 == m_length || m_code[m_pos + 1] != '.'))) {
      ++m_pos;
      point = true;
    } else if (c == 'e' || c == 'E' && !science) {
      ++m_pos;
      science = true;
    } else if ((c == 'p' || c == 'P') && !science && hex) {
      ++m_pos;
      science = true;
    } else if ((c == '-' || c == '+') &&
               (prev == 'e' || prev == 'E' || prev == 'p' || prev == 'P')) {
      ++m_pos;
    } else if (std::isspace(c) || c == ';' ||
               (m_punctuations.find(c) != m_punctuations.end() && c != '.')) {
      break;
    } else {
      ++m_pos;
      return RaiseError(Locate(m_line, m_pos - m_current_line_start - 1),
                        std::string{"unexpected character "} + c +
                          " in number literal.");
    }

    prev = c;
  }

  if (science || point) {
    return Literal(String2<double>(m_code.substr(start, m_pos - start)));
  } else {
    return Literal(String2<long long>(m_code.substr(start, m_pos - start)));
  }
}
