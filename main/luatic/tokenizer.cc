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

#include "helper.hpp"

Tokenizer::Tokenizer(std::optional<std::string> p_filename, std::string p_code):
  m_filename{std::move(p_filename)}, m_code{std::move(p_code)},
  m_length{m_code.length()}, m_pos{0}, m_line{1}, m_current_line_start{0} {}

std::variant<Tokenizer::TokenStream, Tokenizer::DiagnosticList>
  Tokenizer::Parse() noexcept {
  while (m_pos < m_length) {
    ParseOne();
  }

  if (m_diags.empty()) {
    return m_tokens;
  } else {
    return m_diags;
  }
}

void Tokenizer::ParseOne() noexcept {
  const char head = m_code[m_pos];
  if (std::isspace(head)) {
    const int line = m_line;
    if (head == '\n') {
      ++m_line;
      m_current_line_start = m_pos + 1;
    }
    ++m_pos;
  } else if (head == '(') {
    m_tokens.push_back(
      std::make_shared<LeftParenthesisToken>(Position{m_line, m_pos++}));
  } else if (head == ')') {
    m_tokens.push_back(
      std::make_shared<RightParenthesisToken>(Position{m_line, m_pos++}));
  } else if (std::isdigit(head) ||
             ((head == '.' || head == '+' || head == '-') &&
              m_pos + 1 < m_length && std::isdigit(m_code[m_pos + 1]))) {
    ParseNumber();
  } else if (head == '"') {
    const int start = m_pos++;
    while (m_pos < m_length && m_code[m_pos] != '"') {
      ++m_pos;
    }

    if (m_pos == m_length) {
      RaiseError(Position{m_line, start - m_current_line_start},
                 Position{m_line, m_pos - m_current_line_start},
                 "unexpected end of string.");
    } else {
      ++m_pos;
      m_tokens.push_back(std::make_shared<LiteralToken>(
        m_code.substr(start, m_pos - start),
        Position{m_line, start - m_current_line_start},
        Position{m_line, m_pos - m_current_line_start}));
    }
  } else if (head == ';') {
    while (m_pos < m_length && m_code[m_pos] != '\n') {
      ++m_pos;
    }
  } else {
    int start = m_pos++;
    while (m_pos < m_length && m_code[m_pos] != ';' &&
           !std::isspace(m_code[m_pos])) {
      ++m_pos;
    }

    m_tokens.push_back(std::make_shared<SymbolToken>(
      m_code.substr(start, m_pos - start),
      Position{m_line, start - m_current_line_start},
      Position{m_line, m_pos - m_current_line_start}));
  }
}

void Tokenizer::ParseNumber() noexcept {
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
    } else if (std::isspace(c) || c == ';' || c == '(' || c == ')') {
      break;
    } else {
      ++m_pos;
      RaiseError(Position{m_line, m_pos - m_current_line_start - 1},
                 Position{m_line, m_pos - m_current_line_start},
                 std::string{"unexpected character "} + c +
                   " in number literal.");
    }

    prev = c;
  }

  if (science || point) {
    m_tokens.push_back(std::make_shared<LiteralToken>(
      helper::String2<LuaNum>(m_code.substr(start, m_pos - start)),
      Position{m_line, start},
      Position{m_line, m_pos}));
  } else {
    m_tokens.push_back(std::make_shared<LiteralToken>(
      helper::String2<LuaInt>(m_code.substr(start, m_pos - start)),
      Position{m_line, start},
      Position{m_line, m_pos}));
  }
}
