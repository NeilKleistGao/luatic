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

#include "luna_values.h"

LunaType GetTypeOf(const LunaValue& p_value) {
  switch (p_value.index()) {
    case 1:
      return LunaType::LUNA_NIL;
    case 2:
      return LunaType::LUNA_BOOLEAN;
    case 3:
      return LunaType::LUNA_LIGHT_USERDATA;
    case 4:
      return LunaType::LUNA_NUMBER;
    case 5:
      return LunaType::LUNA_STRING;
    case 6:
      return LunaType::LUNA_TABLE;
    case 7:
      return LunaType::LUNA_FUNCTION;
    case 8:
      return LunaType::LUNA_USERDATA;
    case 9:
      return LunaType::LUNA_THREAD;
    default:
      return LunaType::LUNA_NONE;
  }
}
