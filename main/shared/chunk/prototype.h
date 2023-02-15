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

#include "l_string.h"
#include "literal.h"

namespace chunk {
  using uint32 = unsigned int;

  struct UpValue {
    byte in_stack;
    byte index;
  };

  struct LocalVar {
    LString var_name;
    uint32 start_pc;
    uint32 end_pc;
  };

  struct Prototype {
    LString source;
    uint32 line_defined;
    uint32 last_line_defined;
    byte num_params;
    byte is_vararg;
    byte max_stack_size;
    long long* code;
    Literal* constants;
    UpValue* up_values;
    Prototype* proto;
    uint32* line_info;
    LocalVar* local_var;
    LString* up_value_names;
  };

  std::variant<LocalVar, std::string> ReadLocalVar(FILE* p_fp);
  std::variant<Prototype, std::string> ReadPrototype(FILE* p_fp,
                                                     LString p_parent_source);
  std::variant<Prototype*, std::string> ReadPrototypes(FILE* p_fp,
                                                       LString p_parent_source);
} // namespace chunk

#endif //LUATIC_PROTOTYPE_H
