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

#ifndef LUATIC_LUNA_MATH_H
#define LUATIC_LUNA_MATH_H

#include "luna_values.h"

/**
 * 1. We contain some implicit conversion(e.g. int to float) in the math module;
 * 2. We will remove other implicit conversion to make code safer;
 * @see Parser
 */
namespace math {
  /**
   * We only store binary operators.
   */
  enum class ArithOperator { ADD, SUB, MUL, POW, DIV, I_DIV };

  /**
   * We only store binary operators.
   */
  enum class IntOperator { MOD, AND, OR, XOR, SHL, SHR };

  enum class ComOperator { EQ, LT, LE };

  LunaNumber CalcArith(ArithOperator p_ao, LunaNumber p1, LunaNumber p2);
  LunaInt CalcBit(IntOperator p_io, LunaInt p1, LunaInt p2);

  LunaNumber Neg(LunaNumber p);
  LunaNumber Add(LunaNumber p1, LunaNumber p2);
  LunaNumber Sub(LunaNumber p1, LunaNumber p2);
  LunaNumber Multiply(LunaNumber p1, LunaNumber p2);
  LunaNumber Divide(LunaNumber p1, LunaNumber p2);
  LunaNumber Power(LunaNumber p1, LunaNumber p2);
  LunaInt DivideFloor(LunaNumber p1, LunaNumber p2);
  LunaInt ShiftLeft(LunaInt p1, LunaInt p2);
  LunaInt ShiftRight(LunaInt p1, LunaInt p2);
  LunaInt GetLength(LunaString p); // TODO: overload for tables
} // namespace math

#endif //LUATIC_LUNA_MATH_H
