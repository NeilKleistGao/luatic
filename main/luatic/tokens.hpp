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

#ifndef LUATIC_TOKENS_HPP
#define LUATIC_TOKENS_HPP

#include <utility>
#include <variant>
#include <string>
#include <functional>
#include <optional>

#include "diagnostic.hpp"

enum class Keyword{
  KW_AND, KW_BREAK, KW_DO, KW_ELSE, KW_ELSEIF, KW_END,
  KW_FALSE, KW_FOR, KW_FUN, KW_GOTO, KW_IF, KW_IN,
  KW_LOCAL, KW_NIL, KW_NOT, KW_OR, LW_REPEAT, KW_RETURN,
  KW_THEN, KW_TRUE, KW_UNTIL, KW_WHILE
};

struct Identifier {
  std::string name;

  explicit Identifier(std::string p_n): name(std::move(p_n)) {}
};

struct Literal {
  std::string value;

  explicit Literal(std::string p_v): value(std::move(p_v)) {}
};

enum class Operator {
  OP_PLUS, OP_MINUS, OP_MUL, OP_DIV, OP_MOD, OP_POW,
  OP_LEN, OP_AND, OP_XOR, OP_OR, OP_LEFT_SHIFT, OP_RIGHT_SHIFT,
  OP_FD, OP_EQ, OP_NE, OP_LE, OP_GE, OP_LESS, OP_GREAT, OP_ASSIGN,
  OP_LEFT_PAR, OP_RIGHT_PAR, OP_LEFT_BRA, OP_RIGHT_BRA,
  OP_LEFT_SQR, OP_RIGHT_SQR, OP_SEMI, OP_COLON, OP_COMMA, OP_DOT,
  OP_DOT2, OP_DOT3, OP_SPACE
};

struct Token {
  std::variant<Keyword, Identifier, Literal, Operator> token;
  Location location;

  Token(std::variant<Keyword, Identifier, Literal, Operator> p_tok, Location p_loc): token(std::move(p_tok)), location(std::move(p_loc)) {}
};

template <typename T>
T Match(const Token& p_t,
        const std::function<T(const Keyword&)>& p_kw,
        const std::function<T(const Identifier&)>& p_id,
        const std::function<T(const Literal&)>& p_lt,
        const std::function<T(const Operator&)>& p_op) {
  switch (p_t.token.index()) {
    case 0:
      return p_kw(std::get<0>(p_t.token));
    case 1:
      return p_id(std::get<1>(p_t.token));
    case 2:
      return p_lt(std::get<2>(p_t.token));
    case 3:
      return p_op(std::get<3>(p_t.token));
  }
}

#endif //LUATIC_TOKENS_HPP
