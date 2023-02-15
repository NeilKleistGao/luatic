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

#ifndef LUATIC_PROTOTYPE_H
#define LUATIC_PROTOTYPE_H

#include <vector>

#include "literal.h"

namespace chunk {
  struct AbsLineInfo {
    size_t pc;
    size_t line;
  };

  struct UpValue {
    byte in_stack;
    byte index;
    byte kind;
  };

  struct LocalVar {
    std::string var_name;
    size_t start_pc;
    size_t end_pc;
  };

  struct Prototype {
    std::string source;
    size_t line_defined;
    size_t last_line_defined;
    byte num_params;
    byte is_vararg;
    byte max_stack_size;
    std::vector<int> code;
    std::vector<Literal> constants;
    std::vector<UpValue> up_values;
    std::vector<Prototype> proto;
    std::vector<byte> line_info;
    std::vector<AbsLineInfo> abs_line_info;
    std::vector<LocalVar> local_var;
    std::vector<std::string> up_value_names;
  };

  std::variant<LocalVar, Error> ReadLocalVar(FILE* p_fp);
  std::variant<Prototype, Error> ReadPrototype(FILE* p_fp,
                                                     const std::string& p_parent_source);
  std::variant<std::vector<Prototype>, Error> ReadPrototypes(FILE* p_fp,
                                                       const std::string& p_parent_source);
} // namespace chunk

#endif //LUATIC_PROTOTYPE_H
