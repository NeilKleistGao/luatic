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

#include "instructions_pretty_printer.h"

namespace instructions {
  void PrintInstruction(FILE* p_fp,
                        const Instruction& p_ins,
                        const std::string& p_indent) {
    const auto index = p_ins.index();
    if (index == 0) {
      const auto& ins = std::get<0>(p_ins);
      switch (ins.code) {
        case InstABC::OpCode::GET_TAB_UP:
          fprintf(p_fp, "%s GET TAB UP\n", p_indent.c_str());
          break;
        case InstABC::OpCode::CALL:
          fprintf(p_fp, "%s CALL\n", p_indent.c_str());
          break;
        case InstABC::OpCode::RETURN:
          fprintf(p_fp, "%s RETURN\n", p_indent.c_str());
          break;
        case InstABC::OpCode::VAR_ARG_PREP:
          fprintf(p_fp, "%s VAR ARG PREP\n", p_indent.c_str());
          break;
        // TODO: finish pretty printer
        default:
          fprintf(p_fp, "%s %d\n", p_indent.c_str(), ins.code);
      }
    } else if (index == 1) {
      const auto& ins = std::get<1>(p_ins);
      switch (ins.code) {
        case InstABx::OpCode::LOAD_K:
          fprintf(p_fp, "%s LOAD K\n", p_indent.c_str());
          break;
        // TODO: finish pretty printer
        default:
          fprintf(p_fp, "%s %d\n", p_indent.c_str(), ins.code);
      }
    } else if (index == 2) {
      const auto& ins = std::get<2>(p_ins);
      switch (ins.code) {
        // TODO: finish pretty printer
        default:
          fprintf(p_fp, "%s %d\n", p_indent.c_str(), ins.code);
      }
    } else if (index == 3) {
      const auto& ins = std::get<3>(p_ins);
      switch (ins.code) {
        // TODO: finish pretty printer
        default:
          fprintf(p_fp, "%s %d\n", p_indent.c_str(), ins.code);
      }
    } else {
      const auto& ins = std::get<4>(p_ins);
      switch (ins.code) {
        // TODO: finish pretty printer
        default:
          fprintf(p_fp, "%s %d\n", p_indent.c_str(), ins.code);
      }
    }
  }
} // namespace instructions
