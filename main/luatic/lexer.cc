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

#include "lexer.h"

#include <cctype>
#include <utility>

const std::unordered_map<std::string, Keyword> Lexer::m_keywords = {
  {     "and",    Keyword::KW_AND},
  {   "break",  Keyword::KW_BREAK},
  {      "do",     Keyword::KW_DO},
  {    "else",   Keyword::KW_ELSE},
  {  "elseif", Keyword::KW_ELSEIF},
  {     "end",    Keyword::KW_END},
  {   "false",  Keyword::KW_FALSE},
  {     "for",    Keyword::KW_FOR},
  {"function",    Keyword::KW_FUN},
  {    "goto",   Keyword::KW_GOTO},
  {      "if",     Keyword::KW_IF},
  {      "in",     Keyword::KW_IN},
  {   "local",  Keyword::KW_LOCAL},
  {     "nil",    Keyword::KW_NIL},
  {     "not",    Keyword::KW_NOT},
  {      "or",     Keyword::KW_OR},
  {  "repeat", Keyword::LW_REPEAT},
  {  "return", Keyword::KW_RETURN},
  {    "then",   Keyword::KW_THEN},
  {    "true",   Keyword::KW_TRUE},
  {   "until",  Keyword::KW_UNTIL},
  {   "while",  Keyword::KW_WHILE}
};

const std::unordered_map<std::string, Operator> Lexer::m_operators = {
  {  "+",        Operator::OP_PLUS},
  {  "-",       Operator::OP_MINUS},
  {  "*",         Operator::OP_MUL},
  {  "/",         Operator::OP_DIV},
  {  "%",         Operator::OP_MOD},
  {  "^",         Operator::OP_POW},
  {  "#",         Operator::OP_LEN},
  {  "&",         Operator::OP_AND},
  {  "~",         Operator::OP_XOR},
  {  "|",          Operator::OP_OR},
  { "<<",  Operator::OP_LEFT_SHIFT},
  { ">>", Operator::OP_RIGHT_SHIFT},
  { "//",          Operator::OP_FD},
  { "==",          Operator::OP_EQ},
  { "~=",          Operator::OP_NE},
  { "<=",          Operator::OP_LE},
  { ">=",          Operator::OP_GE},
  {  "<",        Operator::OP_LESS},
  {  ">",       Operator::OP_GREAT},
  {  "=",      Operator::OP_ASSIGN},
  {  "(",    Operator::OP_LEFT_PAR},
  {  ")",   Operator::OP_RIGHT_PAR},
  {  "{",    Operator::OP_LEFT_BRA},
  {  "}",   Operator::OP_RIGHT_BRA},
  {  "[",    Operator::OP_LEFT_SQR},
  {  "]",   Operator::OP_RIGHT_SQR},
  {  ";",        Operator::OP_SEMI},
  {  ":",       Operator::OP_COLON},
  {  ",",       Operator::OP_COMMA},
  {  ".",         Operator::OP_DOT},
  { "..",        Operator::OP_DOT2},
  {"...",        Operator::OP_DOT3}
};

Lexer::Lexer(std::optional<std::string> p_filename):
  m_filename(std::move(p_filename)) {}

std::variant<Lexer::TokenStream, Lexer::DiagnosticList>
  Lexer::Parse(const std::string& p_code) const noexcept {
  TokenStream tokens{};
  DiagnosticList diags{};

  int pos = 0;
  int line = 1;
  const auto length = p_code.length();
  while (pos < length) {
    auto res = Parse(p_code, pos, line);
    if (res.index() == 0 && diags.empty()) {
      tokens.push_back(std::move(std::get<0>(res)));
    } else if (res.index() == 1) {
      diags.push_back(std::move(std::get<1>(res)));
    }
  }

  if (diags.empty()) {
    return tokens;
  } else {
    return diags;
  }
}

std::variant<Token, Diagnostic> Lexer::Parse(const std::string& p_code,
                                             int& p_pos,
                                             int& p_line) const noexcept {
  const auto length = p_code.length();
  const char head = p_code[p_pos];
  if (std::isblank(head)) {
    int line = p_line;
    if (head == '\n') {
      ++p_line;
    }
    return Token(Operator::OP_SPACE, Location(line, p_pos++, m_filename));
  } else if (std::isdigit(head) ||
             (head == '.' && p_pos + 1 < length &&
              std::isdigit(p_code[p_pos + 1]))) {
    int start = p_pos;
    auto res = ParseNumber(p_code, p_pos, p_line);
    if (res.index() == 0) {
      return Token(std::get<0>(res), Location(p_line, start, m_filename));
    } else {
      return std::get<1>(res);
    }
  } else if (std::isalpha(head) || head == '_') {
    int start = p_pos++;
    while (p_pos < length &&
           (std::isalnum(p_code[p_pos]) || p_code[p_pos] == '_')) {
      ++p_pos;
    }

    std::string res = p_code.substr(start, p_pos - start);
    if (m_keywords.find(res) != m_keywords.end()) {
      return Token(m_keywords.at(res), Location(p_line, start, m_filename));
    } else {
      return Token(Identifier(res), Location(p_line, start, m_filename));
    }
  } else if (head == '"') {
    // TODO: Parse string
    return RaiseError(p_line, p_pos, "not supported yet.");
  } else {
    // TODO: Parse operator/comment
    return RaiseError(p_line, p_pos, "not supported yet.");
  }
}

std::variant<Literal, Diagnostic>
  Lexer::ParseNumber(const std::string& p_code,
                     int& p_pos,
                     int p_line) const noexcept {
  bool science = false, hex = false, point = false;
  const auto length = p_code.length();

  int start = p_pos;
  char prev = 0;
  while (p_pos < length) {
    const char c = p_code[p_pos];
    if (std::isdigit(c)) {
      ++p_pos;
    } else if (((c >= 'A' && c <= 'F') || (c >= 'a' && c <= 'f')) && hex &&
               !science) {
      ++p_pos;
    } else if ((c == 'x' || c == 'X') && prev == '0' && p_pos - 1 == start) {
      ++p_pos;
      hex = true;
    } else if (c == '.' && !point) {
      ++p_pos;
      point = true;
    } else if (c == 'e' || c == 'E' && !science) {
      ++p_pos;
      science = true;
    } else if ((c == 'p' || c == 'P') && !science && hex) {
      ++p_pos;
      science = true;
    } else if ((c == '-' || c == '+') &&
               (prev == 'e' || prev == 'E' || prev == 'p' || prev == 'P')) {
      ++p_pos;
    } else if (std::isblank(c)) {
      break;
    } else {
      ++p_pos;
      return RaiseError(p_line,
                        p_pos - 1,
                        std::string{"unexpected character "} + c +
                          " in number literal.");
    }

    prev = c;
  }

  return Literal(p_code.substr(start, p_pos - start));
}
