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

#include "parser.h"

#include "context.h"

Parser::Parser(std::optional<std::string> p_filename, TokenStream&& p_tokens):
  m_filename{p_filename}, m_tokens{std::move(p_tokens)},
  m_cur{m_tokens.begin()} {}

std::variant<Program, Parser::DiagnosticList> Parser::Parse() noexcept {
  while (!End()) {
    m_prgm.push_back(ParseExpression());
  }

  if (m_diags.empty()) {
    return m_prgm;
  } else {
    return m_diags;
  }
}

std::shared_ptr<Expression> Parser::ParseExpression() noexcept {
  auto token = *m_cur;
  if (auto lit = std::dynamic_pointer_cast<LiteralToken>(token)) {
    ++m_cur;
    return std::make_shared<Literal>(std::move(lit));
  } else if (auto sym = std::dynamic_pointer_cast<SymbolToken>(token)) {
    ++m_cur;
    if (Context::s_keywords.find(sym->Name()) == Context::s_keywords.end()) {
      return std::make_shared<Variable>(std::move(sym));
    }

    RaiseError(sym->begin,
               sym->end,
               "unexpected keyword " + sym->ToString() + ".");
    return nullptr;
  } else if (const auto p =
               std::dynamic_pointer_cast<LeftParenthesisToken>(token)) {
    ++m_cur;
    std::shared_ptr<Expression> res = nullptr;

    if (!End()) {
      auto next = *m_cur;
      if (next->IsSymbol("define")) {
        ++m_cur;
        res = ParseDefinition();
      } else {
        res = ParseCall();
      }

      if (End() ||
          std::dynamic_pointer_cast<RightParenthesisToken>(*m_cur) == nullptr) {
        RaiseError((*m_cur)->begin, (*m_cur)->end, "expected ')'.");
      } else {
        ++m_cur;
      }
    } else {
      --m_cur;
      RaiseError((*m_cur)->begin, (*m_cur)->end, "unexpected '('.");
    }

    return res;
  }

  RaiseError(token->begin,
             token->end,
             "unexpected token " + token->ToString() + ".");
  return nullptr;
}

std::shared_ptr<Call> Parser::ParseCall() noexcept {
  auto callee = ParseExpression();
  auto call = std::make_shared<Call>(std::move(callee));

  while (!End()) {
    if (std::dynamic_pointer_cast<RightParenthesisToken>(*m_cur) != nullptr) {
      break;
    }

    auto arg = ParseExpression();
    call->Push(std::move(arg));
  }

  return call;
}

std::shared_ptr<Definition> Parser::ParseDefinition() noexcept {
  if (End()) {
    --m_cur;
    RaiseError((*m_cur)->begin, (*m_cur)->end, "expected a symbol here.");
    return nullptr;
  }

  if (auto name = std::dynamic_pointer_cast<SymbolToken>(*m_cur)) {
    ++m_cur;
    if (End()) {
      --m_cur;
      RaiseError((*m_cur)->begin,
                 (*m_cur)->end,
                 "expected an expression here.");
      return nullptr;
    } else {
      auto expr = ParseExpression();
      return std::make_shared<Definition>(std::move(name), std::move(expr));
    }
  }

  RaiseError((*m_cur)->begin,
             (*m_cur)->end,
             "expected a symbol here, but got " + (*m_cur)->ToString() + ".");
  ++m_cur;
  return nullptr;
}
