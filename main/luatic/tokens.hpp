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

#include <string>
#include <utility>
#include <variant>
#include <memory>

#include "diagnostic.hpp"
#include "data_types.hpp"

class Token: public Location {
public:
  Token(Position p_begin, Position p_end): Location(std::move(p_begin), std::move(p_begin)) {}
  virtual bool IsLeftParenthesis() const noexcept = 0;
  virtual bool IsRightParenthesis() const noexcept = 0;
};

class LeftParenthesisToken: public Token {
public:
  bool IsLeftParenthesis() const noexcept {
    return true;
  }

  bool IsRightParenthesis() const noexcept {
    return false;
  }

  explicit LeftParenthesisToken(const Position& p_pos): Token(p_pos, Position{p_pos.line, p_pos.column + 1}) {}
};

class RightParenthesisToken: public Token {
public:
  bool IsLeftParenthesis() const noexcept {
    return false;
  }

  bool IsRightParenthesis() const noexcept {
    return true;
  }

  explicit RightParenthesisToken(const Position& p_pos): Token(p_pos, Position{p_pos.line, p_pos.column + 1}) {}
};

class SymbolToken: public Token {
private:
  std::string m_name;
public:
  bool IsLeftParenthesis() const noexcept {
    return false;
  }

  bool IsRightParenthesis() const noexcept {
    return false;
  }

  SymbolToken(std::string&& p_name, Position p_begin, Position p_end): Token(std::move(p_begin), std::move(p_end)), m_name{p_name} {}
};

class LiteralToken: public Token {
private:
  std::variant<LuaInt, LuaNum, LuaStr> m_value;
public:
  bool IsLeftParenthesis() const noexcept {
    return false;
  }

  bool IsRightParenthesis() const noexcept {
    return false;
  }

  LiteralToken(LuaInt p_value, Position p_begin, Position p_end): Token(std::move(p_begin), std::move(p_end)), m_value{std::move(p_value)} {}
  LiteralToken(LuaNum p_value, Position p_begin, Position p_end): Token(std::move(p_begin), std::move(p_end)), m_value{std::move(p_value)} {}
  LiteralToken(LuaStr p_value, Position p_begin, Position p_end): Token(std::move(p_begin), std::move(p_end)), m_value{std::move(p_value)} {}
};


#endif //LUATIC_TOKENS_H
