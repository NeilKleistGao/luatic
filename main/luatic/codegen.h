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

#ifndef LUATIC_CODEGEN_H
#define LUATIC_CODEGEN_H

#include <memory>
#include <string>
#include <vector>

#include "tokens.hpp"

class Expression {
public:
  virtual bool IsUnit() const noexcept = 0;
  virtual void ToBinary() const noexcept {} // TODO: remove void and {}
};

class Variable: public Expression {
private:
  const std::shared_ptr<SymbolToken> m_name;

public:
  bool IsUnit() const noexcept final { return false; }

  explicit Variable(std::shared_ptr<SymbolToken>&& p_sym):
    m_name{std::move(p_sym)} {}
};

class Literal: public Expression {
private:
  const std::shared_ptr<LiteralToken> m_value;

public:
  bool IsUnit() const noexcept final { return false; }

  explicit Literal(std::shared_ptr<LiteralToken>&& p_value):
    m_value{std::move(p_value)} {}
};

class Definition: public Expression {
private:
  const std::shared_ptr<SymbolToken> m_name;
  const std::shared_ptr<Expression> m_value;

public:
  bool IsUnit() const noexcept final { return true; }

  Definition(std::shared_ptr<SymbolToken>&& p_sym,
             std::shared_ptr<Expression>&& p_value):
    m_name{std::move(p_sym)},
    m_value{std::move(p_value)} {}
};

class Call: public Expression {
private:
  const std::shared_ptr<Expression> m_callee;
  std::vector<std::shared_ptr<Expression>> m_args{};

public:
  bool IsUnit() const noexcept final { return false; }

  explicit Call(std::shared_ptr<Expression>&& p_callee):
    m_callee{std::move(p_callee)} {}

  inline void Push(std::shared_ptr<Expression> p_arg) noexcept {
    m_args.push_back(p_arg);
  }
};

using Program = std::vector<std::shared_ptr<Expression>>;

#endif // LUATIC_CODEGEN_H
