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

#ifndef LUATIC_HELPER_HPP
#define LUATIC_HELPER_HPP

#include <vector>

#include "prototype.h"
#include "helper.h"
#include "shared/instructions/instructions.h"

namespace chunk {
  template<typename T> std::variant<std::vector<T>, Error> ReadArray(FILE* p_fp) {
    size_t length = ReadVarInt(p_fp);
    auto res = std::vector<T>{};
    res.resize(length);
    for (int i = 0; i < length; ++i) {
      if (fread(&res[i], sizeof(T), 1, p_fp) != 1) {
        return "can't read array's elements.";
      }
    }

    return res;
  }

  template<> std::variant<std::vector<LocalVar>, Error> ReadArray(FILE* p_fp) {
    size_t length = ReadVarInt(p_fp);
    auto res = std::vector<LocalVar>{};
    res.resize(length);

    for (int i = 0; i < length; ++i) {
      auto lv = ReadLocalVar(p_fp);
      if (lv.index() == 1) {
        return std::get<1>(lv);
      }
      res[i] = std::get<0>(lv);
    }

    return res;
  }

  template<> std::variant<std::vector<std::string>, Error> ReadArray(FILE* p_fp) {
    size_t length = ReadVarInt(p_fp);
    auto res = std::vector<std::string>{};
    res.resize(length);

    for (int i = 0; i < length; ++i) {
      auto name = ReadString(p_fp);
      if (name.index() == 1) {
        return std::get<1>(name);
      }
      res[i] = std::get<0>(name);
    }

    return res;
  }

  template<> std::variant<std::vector<Literal>, Error> ReadArray(FILE* p_fp) {
    size_t length = ReadVarInt(p_fp);
    auto res = std::vector<Literal>{};
    res.resize(length);

    for (int i = 0; i < length; ++i) {
      auto lit = ReadLiteral(p_fp);
      if (lit.index() == 1) {
        return std::get<1>(lit);
      }
      res[i] = std::get<0>(lit);
    }

    return res;
  }

  template<> std::variant<std::vector<instructions::Instruction>, Error> ReadArray(FILE* p_fp) {
    size_t length = ReadVarInt(p_fp);
    auto res = std::vector<instructions::Instruction>{};

    for (int i = 0; i < length; ++i) {
      unsigned int code;
      if (fread(&code, sizeof(code), 1, p_fp) != 1) {
        return "can't read instruction.";
      }

      const auto ins = instructions::Parse(code);
      if (ins.index() == 1) {
        return std::get<1>(ins);
      }

      res.push_back(std::get<0>(ins));
    }

    return res;
  }
} // namespace chunk

#endif //LUATIC_HELPER_HPP
