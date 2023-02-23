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

#include "luna_math.h"

#include <cmath>

// Convert int to float if necessary.
#define CALC_WITH_IMPLICIT_CONVERT(__P1__, __P2__, __OP__)                     \
const auto __T1__ = __P1__.index();                                            \
const auto __T2__ = __P2__.index();                                            \
if (__T1__ == LUNA_INT && __T2__ == LUNA_INT)                                  \
return std::get<LUNA_INT>(__P1__) __OP__ std::get<LUNA_INT>(__P2__);           \
else if (__T1__ == LUNA_FLOAT && __T2__ == LUNA_INT)                           \
return std::get<LUNA_FLOAT>(__P1__)                                            \
  __OP__ static_cast<LunaFloat>(std::get<LUNA_INT>(__P2__));                   \
else if (__T2__ == LUNA_FLOAT && __T1__ == LUNA_INT)                           \
return static_cast<LunaFloat>(std::get<LUNA_INT>(__P1__))                      \
  __OP__ std::get<LUNA_FLOAT>(__P2__);                                         \
else                                                                           \
return std::get<LUNA_FLOAT>(__P1__) __OP__ std::get<LUNA_FLOAT>(__P2__)

namespace math {
  LunaNumber Add(LunaNumber p1, LunaNumber p2) {
    CALC_WITH_IMPLICIT_CONVERT(p1, p2, +);
  }

  LunaNumber Sub(LunaNumber p1, LunaNumber p2) {
    CALC_WITH_IMPLICIT_CONVERT(p1, p2, -);
  }

  LunaNumber Multiply(LunaNumber p1, LunaNumber p2) {
    CALC_WITH_IMPLICIT_CONVERT(p1, p2, *);
  }

  LunaNumber Divide(LunaNumber p1, LunaNumber p2) {
    CALC_WITH_IMPLICIT_CONVERT(p1, p2, /);
  }

  LunaNumber Power(LunaNumber p1, LunaNumber p2) {
    const auto t1 = p1.index();
    const auto t2 = p2.index();
    if (t1 == LUNA_INT && t2 == LUNA_INT) {
      LunaInt res = 1;
      auto pow = std::get<LUNA_INT>(p2);
      auto base = std::get<LUNA_INT>(p1);
      while (pow > 0) {
        if (pow & 1) {
          res *= base;
        }
        base <<= 1;
        pow >>= 1;
      }

      return res;
    } else if (t1 == LUNA_INT && t2 == LUNA_FLOAT) {
      return std::pow(std::get<LUNA_INT>(p1), std::get<LUNA_FLOAT>(p2));
    } else if (t1 == LUNA_FLOAT && t2 == LUNA_INT) {
      LunaFloat res = 1.0;
      auto pow = std::get<LUNA_INT>(p2);
      auto base = std::get<LUNA_FLOAT>(p1);
      while (pow > 0) {
        if (pow & 1) {
          res *= base;
        }
        base *= 2.0;
        pow >>= 1;
      }

      return res;
    } else {
      return std::pow(std::get<LUNA_FLOAT>(p1), std::get<LUNA_FLOAT>(p2));
    }
  }

  LunaInt DivideFloor(LunaNumber p1, LunaNumber p2) {
    const auto t1 = p1.index();
    const auto t2 = p2.index();
    LunaNumber res;
    if (t1 == LUNA_INT && t2 == LUNA_INT) {
      const auto n1 = std::get<LUNA_INT>(p1);
      const auto n2 = std::get<LUNA_INT>(p2);
      res = (n1 * n2 > 0 || n1 % n2 == 0) ? n1 / n2 : n1 / n2 - 1;
    } else if (t1 == LUNA_INT && t2 == LUNA_FLOAT) {
      const auto n1 = std::get<LUNA_INT>(p1);
      const auto n2 = std::get<LUNA_FLOAT>(p2);
      res = static_cast<LunaFloat>(n1) / n2;
    } else if (t1 == LUNA_FLOAT && t2 == LUNA_INT) {
      const auto n1 = std::get<LUNA_FLOAT>(p1);
      const auto n2 = std::get<LUNA_INT>(p2);
      res = n1 / static_cast<LunaFloat>(n2);
    } else {
      const auto n1 = std::get<LUNA_FLOAT>(p1);
      const auto n2 = std::get<LUNA_FLOAT>(p2);
      res = n1 / n2;
    }

    return (res.index() == LUNA_INT)
      ? std::get<LUNA_INT>(res)
      : static_cast<LunaInt>(std::floor(std::get<LUNA_FLOAT>(res)));
  }

  LunaInt ShiftLeft(LunaInt p1, LunaInt p2) {
    return (p2 >= 0) ? p1 << p2 : ShiftRight(p1, -p2);
  }

  LunaInt ShiftRight(LunaInt p1, LunaInt p2) {
    return (p2 >= 0) ? p1 >> p2 : ShiftLeft(p1, -p2);
  }

  LunaInt GetLength(LunaString p) { return p.size(); }

} // namespace math
