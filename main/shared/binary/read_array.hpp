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

#ifndef LUATIC_READ_ARRAY_HPP
#define LUATIC_READ_ARRAY_HPP

#include "prototype.h"

namespace chunk {
  template<typename T> std::variant<T*, std::string> ReadArray(FILE* p_fp) {
    uint32 length;
    if (fread(&length, sizeof(uint32), 1, p_fp) != 1) {
      return "can't read array's length.";
    }

    auto res = new(std::nothrow) T[length];
    if (res == nullptr) {
      return "out of memory.";
    }

    for (int i = 0; i < length; ++i) {
      if (fread(res + i, sizeof(T), 1, p_fp) != 1) {
        return "can't read array's elements.";
      }
    }

    return res;
  }

  template<> std::variant<LocalVar*, std::string> ReadArray(FILE* p_fp) {
    uint32 length;
    if (fread(&length, sizeof(uint32), 1, p_fp) != 1) {
      return "can't read array's length.";
    }

    auto res = new(std::nothrow) LocalVar[length];
    if (res == nullptr) {
      return "out of memory.";
    }

    for (int i = 0; i < length; ++i) {
      auto lv = ReadLocalVar(p_fp);
      if (lv.index() == 1) {
        return std::get<1>(lv);
      }
      res[i] = std::get<0>(lv);
    }

    return res;
  }

  template<> std::variant<LString*, std::string> ReadArray(FILE* p_fp) {
    uint32 length;
    if (fread(&length, sizeof(uint32), 1, p_fp) != 1) {
      return "can't read array's length.";
    }

    auto res = new(std::nothrow) LString[length];
    if (res == nullptr) {
      return "out of memory.";
    }

    for (int i = 0; i < length; ++i) {
      auto name = ReadLString(p_fp);
      if (name.index() == 1) {
        return std::get<1>(name);
      }
      res[i] = std::get<0>(name);
    }

    return res;
  }

  template<> std::variant<Literal*, std::string> ReadArray(FILE* p_fp) {
    uint32 length;
    if (fread(&length, sizeof(uint32), 1, p_fp) != 1) {
      return "can't read array's length.";
    }

    auto res = new(std::nothrow) Literal[length];
    if (res == nullptr) {
      return "out of memory.";
    }

    for (int i = 0; i < length; ++i) {
      auto lit = ReadLiteral(p_fp);
      if (lit.index() == 1) {
        return std::get<1>(lit);
      }
      res[i] = std::get<0>(lit);
    }

    return res;
  }
} // namespace chunk

#endif //LUATIC_READ_ARRAY_HPP
