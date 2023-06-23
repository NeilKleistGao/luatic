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

#ifndef LUATIC_TOKENS_H
#define LUATIC_TOKENS_H

#include <functional>
#include <optional>
#include <string>
#include <utility>
#include <variant>

#include "diagnostic.hpp"

enum class Keyword {
  KW_AND,
  KW_BREAK,
  KW_DO,
  KW_ELSE,
  KW_ELSEIF,
  KW_END,
  KW_FALSE,
  KW_FOR,
  KW_FUN,
  KW_GOTO,
  KW_IF,
  KW_IN,
  KW_LOCAL,
  KW_NIL,
  KW_NOT,
  KW_OR,
  KW_REPEAT,
  KW_RETURN,
  KW_THEN,
  KW_TRUE,
  KW_UNTIL,
  KW_WHILE
};

struct Identifier {
  std::string name;

  explicit Identifier(std::string p_n): name(std::move(p_n)) {}
};

struct Literal {
  std::string value;

  explicit Literal(std::string p_v): value(std::move(p_v)) {}
};

enum class Punctuation {
  PUN_PLUS,
  PUN_MINUS,
  PUN_MUL,
  PUN_DIV,
  PUN_MOD,
  PUN_POW,
  PUN_LEN,
  PUN_AND,
  PUN_XOR,
  PUN_OR,
  PUN_LEFT_SHIFT,
  PUN_RIGHT_SHIFT,
  PUN_FD,
  PUN_EQ,
  PUN_NE,
  PUN_LE,
  PUN_GE,
  PUN_LESS,
  PUN_GREAT,
  PUN_ASSIGN,
  PUN_LEFT_PAR,
  PUN_RIGHT_PAR,
  PUN_LEFT_BRA,
  PUN_RIGHT_BRA,
  PUN_LEFT_SQR,
  PUN_RIGHT_SQR,
  PUN_SEMI,
  PUN_COLON,
  PUN_COMMA,
  PUN_DOT,
  PUN_DOT2,
  PUN_DOT3,
  PUN_SPACE
};

struct Token {
  std::variant<Keyword, Identifier, Literal, Punctuation> token;
  Location location;

  Token(std::variant<Keyword, Identifier, Literal, Punctuation> p_tok,
        Location p_loc):
    token(std::move(p_tok)),
    location(p_loc) {}
};

namespace std {
  std::string to_string(Keyword p_kw);
  std::string to_string(Punctuation p_punc);

  inline std::string to_string(const Identifier& p_id) {
    return "symbol " + p_id.name;
  }

  inline std::string to_string(const Literal& p_lit) {
    return "literal " + p_lit.value;
  }
} // namespace std

#endif //LUATIC_TOKENS_H
