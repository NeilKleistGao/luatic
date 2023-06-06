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

const std::unordered_map<std::string, Punctuation> Lexer::m_punctuations = {
  {  "+",        Punctuation::PUN_PLUS},
  {  "-",       Punctuation::PUN_MINUS},
  {  "*",         Punctuation::PUN_MUL},
  {  "/",         Punctuation::PUN_DIV},
  {  "%",         Punctuation::PUN_MOD},
  {  "^",         Punctuation::PUN_POW},
  {  "#",         Punctuation::PUN_LEN},
  {  "&",         Punctuation::PUN_AND},
  {  "~",         Punctuation::PUN_XOR},
  {  "|",          Punctuation::PUN_OR},
  { "<<",  Punctuation::PUN_LEFT_SHIFT},
  { ">>", Punctuation::PUN_RIGHT_SHIFT},
  { "//",          Punctuation::PUN_FD},
  { "==",          Punctuation::PUN_EQ},
  { "~=",          Punctuation::PUN_NE},
  { "<=",          Punctuation::PUN_LE},
  { ">=",          Punctuation::PUN_GE},
  {  "<",        Punctuation::PUN_LESS},
  {  ">",       Punctuation::PUN_GREAT},
  {  "=",      Punctuation::PUN_ASSIGN},
  {  "(",    Punctuation::PUN_LEFT_PAR},
  {  ")",   Punctuation::PUN_RIGHT_PAR},
  {  "{",    Punctuation::PUN_LEFT_BRA},
  {  "}",   Punctuation::PUN_RIGHT_BRA},
  {  "[",    Punctuation::PUN_LEFT_SQR},
  {  "]",   Punctuation::PUN_RIGHT_SQR},
  {  ";",        Punctuation::PUN_SEMI},
  {  ":",       Punctuation::PUN_COLON},
  {  ",",       Punctuation::PUN_COMMA},
  {  ".",         Punctuation::PUN_DOT},
  { "..",        Punctuation::PUN_DOT2},
  {"...",        Punctuation::PUN_DOT3}
};

Lexer::Lexer(std::optional<std::string> p_filename):
  m_filename(std::move(p_filename)) {}

std::variant<Lexer::TokenStream, Lexer::DiagnosticList>
  Lexer::Parse(const std::string& p_code) const noexcept {
  TokenStream tokens{};
  DiagnosticList diags{};

  int pos = 0;
  int line = 1;
  int start = 0;
  const auto length = p_code.length();
  while (pos < length) {
    auto res = Parse(p_code, pos, line, start);
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
                                             int& p_line,
                                             int& p_start) const noexcept {
  const auto length = p_code.length();
  const char head = p_code[p_pos];
  if (std::isspace(head)) {
    int line = p_line;
    if (head == '\n') {
      ++p_line;
      p_start = p_pos + 1;
    }
    return Token(Punctuation::PUN_SPACE, Location(line, p_pos++, m_filename));
  } else if (std::isdigit(head) ||
             (head == '.' && p_pos + 1 < length &&
              std::isdigit(p_code[p_pos + 1]))) {
    int start = p_pos;
    auto res = ParseNumber(p_code, p_start, p_pos, p_line);
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
  } else if (head == '"' || head == '\'') {
    int start = p_pos++;
    char prev[2] = {-1, head};
    while (p_pos < length && p_code[p_pos] != head) {
      if (p_code[p_pos] == '\n' && !(prev[0] == '\\' && prev[1] == 'z')) {
        const int start_backup = p_start;
        p_start = p_pos + 1;
        return RaiseError(p_line,
                          p_pos - start_backup,
                          "unexpected end of string.");
      }
      prev[0] = prev[1];
      prev[1] = p_code[p_pos];
      ++p_pos;
    }

    if (p_pos == length) {
      return RaiseError(p_line, p_pos - p_start, "unexpected end of string.");
    }

    ++p_pos;
    return Token(Literal(p_code.substr(start, p_pos - start)),
                 Location(p_line, start, m_filename));
  } else if (head == '[' && p_pos + 1 < length &&
             (p_code[p_pos + 1] == '[' || p_code[p_pos + 1] == '=')) {
    int start = p_pos;
    const auto res = ParseMultipleLineBlock(p_code, p_pos, p_line, p_start);
    if (res.index() == 0) {
      return Token(
        Literal(std::string{"\""} + std::get<0>(res) + std::string{"\""}),
        Location(p_line, start, m_filename));
    } else {
      return std::get<1>(res);
    }
  } else if (head == '-' && p_pos + 1 < length && p_code[p_pos + 1] == '-') {
    return ParseComment(p_code, p_pos, p_line, p_start);
  } else {
    int start = p_pos;
    auto op = std::string{head};
    if (m_punctuations.find(op) != m_punctuations.end()) {
      ++p_pos;
      while (p_pos < length) {
        const auto next_op = op + p_code[p_pos];
        if (m_punctuations.find(next_op) != m_punctuations.end()) {
          op = next_op;
          ++p_pos;
        } else {
          return Token(m_punctuations.at(op),
                       Location(p_line, start, m_filename));
        }
      }
    } else {
      ++p_pos;
      return RaiseError(p_line,
                        p_pos - p_start,
                        std::string{"unexpected character "} + head + ".");
    }
  }
}

std::variant<Literal, Diagnostic>
  Lexer::ParseNumber(const std::string& p_code,
                     const int& p_start,
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
    } else if (std::isspace(c) ||
               m_punctuations.find(std::string{c}) != m_punctuations.end()) {
      break;
    } else {
      ++p_pos;
      return RaiseError(p_line,
                        p_pos - 1 - p_start,
                        std::string{"unexpected character "} + c +
                          " in number literal.");
    }

    prev = c;
  }

  return Literal(p_code.substr(start, p_pos - start));
}

std::variant<std::string, Diagnostic>
  Lexer::ParseMultipleLineBlock(const std::string& p_code,
                                int& p_pos,
                                int& p_line,
                                int& p_start) const noexcept {
  std::string res;
  const auto length = p_code.length();
  int eq_num = 0, process = 0;
  bool start_flag = false, end_flag = false;
  ++p_pos;

  while (p_pos < length) {
    const char c = p_code[p_pos++];
    if (c == '[' && !start_flag) {
      start_flag = true;
    } else if (c == '=' && !start_flag) {
      ++eq_num;
    } else if (c == '=' && end_flag) {
      ++process;
    } else if (c == ']') {
      res += c;
      if (!end_flag) {
        end_flag = true;
      } else if (end_flag && process == eq_num) {
        break;
      } else {
        process = 0;
        end_flag = false;
      }
    } else {
      if (c == '\n') {
        ++p_line;
        p_start = p_pos;
      }
      process = 0;
      end_flag = false;
      res += c;
    }
  }

  if (eq_num == process) {
    return res.substr(0, res.length() - eq_num - 2);
  } else {
    return RaiseError(p_line,
                      p_pos - p_start,
                      "unfinished multiple-line block.");
  }
}

std::variant<Token, Diagnostic>
  Lexer::ParseComment(const std::string& p_code,
                      int& p_pos,
                      int& p_line,
                      int& p_start) const noexcept {
  int start = p_pos;
  const auto length = p_code.length();
  p_pos += 2;
  if (p_pos < length) {
    bool multiple_line = false;
    if (p_code[p_pos] == '[') {
      for (int i = p_pos + 1; i < length; ++i) {
        if (p_code[i] == '=') {
          continue;
        } else if (p_code[i] == '[') {
          multiple_line = true;
          break;
        } else {
          break;
        }
      }
    }

    if (multiple_line) {
      const auto res = ParseMultipleLineBlock(p_code, p_pos, p_line, p_start);
      if (res.index() == 0) {
        return Token(Punctuation::PUN_SPACE,
                     Location(p_line, start, m_filename));
      } else {
        return std::get<1>(res);
      }
    } else {
      while (p_pos < length) {
        if (p_code[p_pos] == '\n') {
          p_start = p_pos + 1;
          break;
        }
      }

      ++p_pos;
      return Token(Punctuation::PUN_SPACE, Location(p_line, start, m_filename));
    }
  } else {
    return Token(Punctuation::PUN_SPACE, Location(p_line, start, m_filename));
  }
}
