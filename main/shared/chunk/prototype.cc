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

#include "prototype.h"
#include "helper.hpp"

#define READ_ARRAY(__PROTO__, __FIELD__, __FP__, __TYPE__)                     \
auto __FIELD__ = ReadArray<__TYPE__>(__FP__);                                  \
if (__FIELD__.index() == 1) {                                                  \
return std::get<1>(__FIELD__);                                                 \
}                                                                              \
__PROTO__.__FIELD__ = std::get<0>(__FIELD__)

#define READ_PROTOTYPE_ARRAY(__PARENT__, __PARENT_SRC__, __FP__)               \
auto __PROTO__ = ReadPrototypes(__FP__, __PARENT_SRC__);                       \
if (__PROTO__.index() == 1) {                                                  \
return std::get<1>(__PROTO__);                                                 \
}                                                                              \
__PARENT__.proto = std::get<0>(__PROTO__)

namespace chunk {
  std::variant<LocalVar, Error> ReadLocalVar(FILE* p_fp) {
    auto var = LocalVar{};
    const auto name = ReadString(p_fp);
    if (name.index() == 1) {
      return "can't read local variable's name.";
    }

    var.var_name = std::get<0>(name);
    if (fread(&var.start_pc, sizeof(var.start_pc), 1, p_fp) != 1) {
      return "can't read local variable's start pc.";
    }
    if (fread(&var.end_pc, sizeof(var.end_pc), 1, p_fp) != 1) {
      return "can't read local variable's end pc.";
    }

    return var;
  }

  std::variant<Prototype, Error>
    ReadPrototype(FILE* p_fp, const std::string& p_parent_source) {
    auto res = Prototype{};

    auto src = (p_parent_source.empty()) ? ReadString(p_fp) : p_parent_source;
    if (src.index() == 1) {
      return std::get<1>(src);
    }
    res.source = std::get<0>(src);

    res.line_defined = ReadVarInt(p_fp);
    res.last_line_defined = ReadVarInt(p_fp);
    res.num_params = fgetc(p_fp);
    res.is_vararg = fgetc(p_fp);
    res.max_stack_size = fgetc(p_fp);

    READ_ARRAY(res, code, p_fp, instructions::Instruction);
    READ_ARRAY(res, constants, p_fp, Literal);
    READ_ARRAY(res, up_values, p_fp, UpValue);
    READ_PROTOTYPE_ARRAY(res, res.source, p_fp);
    READ_ARRAY(res, line_info, p_fp, byte);
    READ_ARRAY(res, abs_line_info, p_fp, AbsLineInfo);
    READ_ARRAY(res, local_var, p_fp, LocalVar);
    READ_ARRAY(res, up_value_names, p_fp, std::string);

    return res;
  }

  std::variant<std::vector<Prototype>, Error>
    ReadPrototypes(FILE* p_fp, const std::string& p_parent_source) {
    size_t length = ReadVarInt(p_fp);
    auto res = std::vector<Prototype>{};
    res.resize(length);

    for (int i = 0; i < length; ++i) {
      auto proto = ReadPrototype(p_fp, p_parent_source);
      if (proto.index() == 1) {
        return std::get<1>(proto);
      }
      res[i] = std::get<0>(proto);
    }

    return res;
  }
} // namespace chunk
