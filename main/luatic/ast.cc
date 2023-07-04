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

#include "ast.h"

Location GetLocation(const Stmt& p_stmt) {
  switch (p_stmt.value.index()) {
    case 0:
      return std::get<0>(p_stmt.value).loc;
    case 1:
      return std::get<1>(p_stmt.value).loc;
    case 2:
      return std::get<2>(p_stmt.value).loc;
    case 3:
      return std::get<3>(p_stmt.value).loc;
    case 4:
      return std::get<4>(p_stmt.value).loc;
    case 5:
      return std::get<5>(p_stmt.value).loc;
    case 6:
      return std::get<6>(p_stmt.value).loc;
    case 7:
      return std::get<7>(p_stmt.value).loc;
    case 8:
      return std::get<8>(p_stmt.value).loc;
    case 9:
      return std::get<9>(p_stmt.value).loc;
    case 10:
      return std::get<10>(p_stmt.value).loc;
    case 11:
      return std::get<11>(p_stmt.value).loc;
    case 12:
      return std::get<12>(p_stmt.value).loc;
  }
}

Location GetLocation(const Expr& p_expr) {
  switch (p_expr.value.index()) {
    case 0:
      return std::get<0>(p_expr.value).loc;
    case 1:
      return std::get<1>(p_expr.value).loc;
    case 2:
      return std::get<2>(p_expr.value).loc;
    case 3:
      return std::get<3>(p_expr.value).loc;
    case 4:
      return std::get<4>(p_expr.value).loc;
    case 5:
      return std::get<5>(p_expr.value).loc;
    case 6:
      return std::get<6>(p_expr.value).loc;
    case 7:
      return std::get<7>(p_expr.value).loc;
    case 8:
      return std::get<8>(p_expr.value).loc;
    case 9:
      return std::get<9>(p_expr.value).loc;
    case 10:
      return std::get<10>(p_expr.value).loc;
    case 11:
      return std::get<11>(p_expr.value).loc;
    case 12:
      return std::get<12>(p_expr.value).loc;
  }
}
