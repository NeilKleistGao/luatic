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

#include "literal.h"

namespace chunk {
  std::variant<Literal, std::string> ReadLiteral(FILE* p_fp) {
    byte tag = fgetc(p_fp);
    if (tag == TAG_NIL) {
      return std::variant<Literal, std::string>{Literal{nullptr}};
    } else if (tag == TAG_BOOLEAN) {
      const BooleanLiteral value = fgetc(p_fp);
      return value;
    } else if (tag == TAG_NUMBER) {
      NumberLiteral value;
      if (fread(&value, sizeof(NumberLiteral), 1, p_fp) != 1) {
        return "can't read number literal.";
      } else {
        return value;
      }
    } else if (tag == TAG_INTEGER) {
      IntLiteral value;
      if (fread(&value, sizeof(IntLiteral), 1, p_fp) != 1) {
        return "can't read integer literal.";
      } else {
        return value;
      }
    } else if (tag == TAG_SHORT_STR || tag == TAG_LONG_STR) {
      const auto value = ReadLString(p_fp);
      if (value.index() == 1) {
        return std::get<1>(value);
      } else {
        const LString str = std::get<0>(value);
        if ((str.index() == 2 && tag == TAG_LONG_STR) ||
            (str.index() < 2 && tag == TAG_SHORT_STR)) {
          return str;
        } else {
          return "wrong string literal tag.";
        }
      }
    } else {
      return "wrong literal tag.";
    }
  }
} // namespace chunk
