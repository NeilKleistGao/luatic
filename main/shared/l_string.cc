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

#include "l_string.h"

#include <cstring>

namespace chunk {
  std::variant<LString, std::string> ReadLString(FILE* p_fp) {
    byte head = fgetc(p_fp);
    if (head == 0u) {
      return NullString{0u};
    } else if (head == 255u) {
      size_t length;
      if (fread(&length, sizeof(size_t), 1, p_fp) != 1) {
        return "can't read long string.";
      } else {
        auto ls = LongString{};
        ls.long_string_num = head;
        ls.length = length - 1;
        ls.buff = new (std::nothrow) char[ls.length + 1];
        ls.buff[ls.length] = 0;
        if (ls.buff == nullptr) {
          return "out of memory.";
        }

        if (fread(ls.buff, sizeof(byte), ls.length, p_fp) != ls.length) {
          return "can't read long string.";
        } else {
          return ls;
        }
      }
    } else {
      auto ls = ShortString{};
      ls.length = (head & 0b01111111) - 1;
      ls.buff = new (std::nothrow) char[ls.length + 1];
      ls.buff[ls.length] = 0;
      if (ls.buff == nullptr) {
        return "out of memory.";
      }

      if (fread(ls.buff, sizeof(byte), ls.length, p_fp) != ls.length) {
        return "can't read short string.";
      } else {
        return ls;
      }
    }
  }
} // namespace chunk
