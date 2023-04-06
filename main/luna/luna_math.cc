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

#include <array>
#include <cmath>
#include <functional>

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

#define RETURN_IF_DIFF_TYPES(__P1__, __P2__, __T1__, __T2__)                   \
const auto __T1__ = __P1__.index();                                            \
const auto __T2__ = __P2__.index();                                            \
if (__T1__ != __T2__) return false // TODO: add type checking

#define EXTRACT_VALUES(__P1__, __P2__, __V1__, __V2__, __TYPE__)               \
const auto __V1__ = std::get<__TYPE__>(__P1__);                                \
const auto __V2__ = std::get<__TYPE__>(__P2__);                                \
if (true)

namespace math {
  LunaNumber Neg(LunaNumber p) {
    if (p.index() == LUNA_INT) {
      return -std::get<LUNA_INT>(p);
    } else {
      return -std::get<LUNA_FLOAT>(p);
    }
  }

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

  LunaNumber CalcArith(ArithOperator p_ao, LunaNumber p1, LunaNumber p2) {
    static const auto lams =
      std::array<std::function<LunaNumber(LunaNumber, LunaNumber)>, 6>{
        [](LunaNumber x, LunaNumber y) { return Add(x, y); },
        [](LunaNumber x, LunaNumber y) { return Sub(x, y); },
        [](LunaNumber x, LunaNumber y) { return Multiply(x, y); },
        [](LunaNumber x, LunaNumber y) { return Power(x, y); },
        [](LunaNumber x, LunaNumber y) { return Divide(x, y); },
        [](LunaNumber x, LunaNumber y) {
          return DivideFloor(x, y);
        }};

    return lams[static_cast<int>(p_ao)](p1, p2);
  }

  LunaInt CalcBit(IntOperator p_io, LunaInt p1, LunaInt p2) {
    static const auto lams =
      std::array<std::function<LunaInt(LunaInt, LunaInt)>, 6>{
        [](LunaInt x, LunaInt y) { return x % y; },
        [](LunaInt x, LunaInt y) { return x & y; },
        [](LunaInt x, LunaInt y) { return x | y; },
        [](LunaInt x, LunaInt y) { return x ^ y; },
        [](LunaInt x, LunaInt y) { return ShiftLeft(x, y); },
        [](LunaInt x, LunaInt y) {
          return ShiftRight(x, y);
        }};

    return lams[static_cast<int>(p_io)](p1, p2);
  }

  LunaBoolean
    Compare(ComOperator p_co, const LunaValue& p1, const LunaValue& p2) {
    static const auto lams =
      std::array<std::function<LunaBoolean(const LunaValue&, const LunaValue&)>,
                 3>{
        [](const LunaValue& x, const LunaValue& y) { return Equal(x, y); },
        [](const LunaValue& x, const LunaValue& y) { return LessThan(x, y); },
        [](const LunaValue& x, const LunaValue& y) {
          return LessEqual(x, y);
        }};

    return lams[static_cast<int>(p_co)](p1, p2);
  }

  LunaBoolean Equal(const LunaValue& p1, const LunaValue& p2) {
    RETURN_IF_DIFF_TYPES(p1, p2, t1, t2);
    switch (t1) {
      case LunaType::LUNA_NONE:
      case LunaType::LUNA_NIL:
        return true;
      case LunaType::LUNA_BOOLEAN: {
        EXTRACT_VALUES(p1, p2, v1, v2, LunaType::LUNA_BOOLEAN) {
          return v1 == v2;
        }
      }
      case LunaType::LUNA_LIGHT_USERDATA:
        return false; // TODO: implement type
      case LunaType::LUNA_NUMBER: {
        EXTRACT_VALUES(p1, p2, v1, v2, LunaType::LUNA_NUMBER) {
          const auto sub = Sub(v1, v2);
          if (sub.index() == LUNA_INT) {
            return std::get<LUNA_INT>(sub) == 0;
          } else {
            return std::get<LUNA_FLOAT>(sub) == 0.0;
          }
        }
      }
      case LunaType::LUNA_STRING: {
        EXTRACT_VALUES(p1, p2, v1, v2, LunaType::LUNA_STRING) {
          return v1 == v2;
        }
      }
      case LunaType::LUNA_TABLE: {
        EXTRACT_VALUES(p1, p2, v1, v2, LunaType::LUNA_TABLE) {
          return v1.get() == v2.get();
        }
      }
      case LunaType::LUNA_FUNCTION:
      case LunaType::LUNA_USERDATA:
      case LunaType::LUNA_THREAD:
      default:
        return false; // TODO: implement type
    }
  }

  LunaBoolean LessThan(const LunaValue& p1, const LunaValue& p2) {
    RETURN_IF_DIFF_TYPES(p1, p2, t1, t2);
    switch (t1) {
      case LunaType::LUNA_NONE:
      case LunaType::LUNA_NIL:
      case LunaType::LUNA_BOOLEAN:
      case LunaType::LUNA_LIGHT_USERDATA:
        return false; // TODO: implement type
      case LunaType::LUNA_NUMBER: {
        EXTRACT_VALUES(p1, p2, v1, v2, LunaType::LUNA_NUMBER) {
          const auto sub = Sub(v1, v2);
          if (sub.index() == LUNA_INT) {
            return std::get<LUNA_INT>(sub) < 0;
          } else {
            return std::get<LUNA_FLOAT>(sub) < 0.0;
          }
        }
      }
      case LunaType::LUNA_STRING: {
        EXTRACT_VALUES(p1, p2, v1, v2, LunaType::LUNA_STRING) {
          return v1 < v2;
        }
      }
      case LunaType::LUNA_TABLE:
      case LunaType::LUNA_FUNCTION:
      case LunaType::LUNA_USERDATA:
      case LunaType::LUNA_THREAD:
      default:
        return false; // TODO: implement type
    }
  }

  LunaBoolean LessEqual(const LunaValue& p1, const LunaValue& p2) {
    RETURN_IF_DIFF_TYPES(p1, p2, t1, t2);
    switch (t1) {
      case LunaType::LUNA_NONE:
      case LunaType::LUNA_NIL:
      case LunaType::LUNA_BOOLEAN:
      case LunaType::LUNA_LIGHT_USERDATA:
        return false; // TODO: implement type
      case LunaType::LUNA_NUMBER: {
        EXTRACT_VALUES(p1, p2, v1, v2, LunaType::LUNA_NUMBER) {
          const auto sub = Sub(v1, v2);
          if (sub.index() == LUNA_INT) {
            return std::get<LUNA_INT>(sub) <= 0;
          } else {
            return std::get<LUNA_FLOAT>(sub) <= 0.0;
          }
        }
      }
      case LunaType::LUNA_STRING: {
        EXTRACT_VALUES(p1, p2, v1, v2, LunaType::LUNA_STRING) {
          return v1 <= v2;
        }
      }
      case LunaType::LUNA_TABLE:
      case LunaType::LUNA_FUNCTION:
      case LunaType::LUNA_USERDATA:
      case LunaType::LUNA_THREAD:
      default:
        return false; // TODO: implement type
    }
  }

} // namespace math
