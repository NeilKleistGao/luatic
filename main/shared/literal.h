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

#ifndef LUATIC_LITERAL_H
#define LUATIC_LITERAL_H

#include <variant>
#include <string>
#include <cstdio>

#include "l_string.h"

namespace chunk {
  constexpr byte TAG_NIL = 0u;
  constexpr byte TAG_BOOLEAN = 1u;
  constexpr byte TAG_NUMBER = 3u;
  constexpr byte TAG_INTEGER = 19u;
  constexpr byte TAG_SHORT_STR = 4u;
  constexpr byte TAG_LONG_STR = 20u;

  using NilLiteral = std::nullptr_t;
  using BooleanLiteral = byte;
  using NumberLiteral = double;
  using IntLiteral = long long;
  using StrLiteral = LString;

  using Literal = std::variant<NilLiteral, BooleanLiteral, NumberLiteral, IntLiteral, StrLiteral>;
  std::variant<Literal, std::string> ReadLiteral(FILE* p_fp);
} // namespace chunk

#endif //LUATIC_LITERAL_H
