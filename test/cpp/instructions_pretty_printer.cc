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
        case InstABC::OpCode::MOVE:
          fprintf(p_fp, "%s MOVE\n", p_indent.c_str());
          break;
        case InstABC::OpCode::LOAD_FALSE:
          fprintf(p_fp, "%s LOAD FALSE\n", p_indent.c_str());
          break;
        case InstABC::OpCode::L_FALSE_SKIP:
          fprintf(p_fp, "%s L FALSE SKIP\n", p_indent.c_str());
          break;
        case InstABC::OpCode::LOAD_TRUE:
          fprintf(p_fp, "%s LOAD TRUE\n", p_indent.c_str());
          break;
        case InstABC::OpCode::LOAD_NIL:
          fprintf(p_fp, "%s LOAD NIL\n", p_indent.c_str());
          break;
        case InstABC::OpCode::GET_UP_VAL:
          fprintf(p_fp, "%s GET UP VAL\n", p_indent.c_str());
          break;
        case InstABC::OpCode::GET_TAB_UP:
          fprintf(p_fp, "%s GET TAB UP\n", p_indent.c_str());
          break;
        case InstABC::OpCode::SET_TAB_UP:
          fprintf(p_fp, "%s SET TAB UP\n", p_indent.c_str());
          break;
        case InstABC::OpCode::SET_FIELD:
          fprintf(p_fp, "%s SET FIELD\n", p_indent.c_str());
          break;
        case InstABC::OpCode::NEW_TABLE:
          fprintf(p_fp, "%s NEW TABLE\n", p_indent.c_str());
          break;
        case InstABC::OpCode::ADD_I:
          fprintf(p_fp, "%s ADD I\n", p_indent.c_str());
          break;
        case InstABC::OpCode::SUB_K:
          fprintf(p_fp, "%s SUB K\n", p_indent.c_str());
          break;
        case InstABC::OpCode::MUL_K:
          fprintf(p_fp, "%s MUL K\n", p_indent.c_str());
          break;
        case InstABC::OpCode::POW_K:
          fprintf(p_fp, "%s POW K\n", p_indent.c_str());
          break;
        case InstABC::OpCode::DIV_K:
          fprintf(p_fp, "%s DIV K\n", p_indent.c_str());
          break;
        case InstABC::OpCode::B_AND_K:
          fprintf(p_fp, "%s B AND K\n", p_indent.c_str());
          break;
        case InstABC::OpCode::B_OR_K:
          fprintf(p_fp, "%s B OR K\n", p_indent.c_str());
          break;
        case InstABC::OpCode::ADD:
          fprintf(p_fp, "%s ADD\n", p_indent.c_str());
          break;
        case InstABC::OpCode::MUL:
          fprintf(p_fp, "%s MUL\n", p_indent.c_str());
          break;
        case InstABC::OpCode::MM_BIN:
          fprintf(p_fp, "%s MM BIN\n", p_indent.c_str());
          break;
        case InstABC::OpCode::MM_BIN_I:
          fprintf(p_fp, "%s MM BIN I\n", p_indent.c_str());
          break;
        case InstABC::OpCode::MM_BIN_K:
          fprintf(p_fp, "%s MM BIN K\n", p_indent.c_str());
          break;
        case InstABC::OpCode::NOT:
          fprintf(p_fp, "%s NOT\n", p_indent.c_str());
          break;
        case InstABC::OpCode::LEN:
          fprintf(p_fp, "%s LEN\n", p_indent.c_str());
          break;
        case InstABC::OpCode::CONCAT:
          fprintf(p_fp, "%s CONCAT\n", p_indent.c_str());
          break;
        case InstABC::OpCode::EQ_I:
          fprintf(p_fp, "%s EQ I\n", p_indent.c_str());
          break;
        case InstABC::OpCode::LT_I:
          fprintf(p_fp, "%s LT I\n", p_indent.c_str());
          break;
        case InstABC::OpCode::LE_I:
          fprintf(p_fp, "%s LE I\n", p_indent.c_str());
          break;
        case InstABC::OpCode::TEST:
          fprintf(p_fp, "%s TEST\n", p_indent.c_str());
          break;
        case InstABC::OpCode::CALL:
          fprintf(p_fp, "%s CALL\n", p_indent.c_str());
          break;
        case InstABC::OpCode::TAIL_CALL:
          fprintf(p_fp, "%s TAIL CALL\n", p_indent.c_str());
          break;
        case InstABC::OpCode::RETURN:
          fprintf(p_fp, "%s RETURN\n", p_indent.c_str());
          break;
        case InstABC::OpCode::RETURN_0:
          fprintf(p_fp, "%s RETURN 0\n", p_indent.c_str());
          break;
        case InstABC::OpCode::RETURN_1:
          fprintf(p_fp, "%s RETURN 1\n", p_indent.c_str());
          break;
        case InstABC::OpCode::SET_LIST:
          fprintf(p_fp, "%s SET LIST\n", p_indent.c_str());
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
        case InstABx::OpCode::FOR_LOOP:
          fprintf(p_fp, "%s FOR LOOP\n", p_indent.c_str());
          break;
        case InstABx::OpCode::FOR_PREP:
          fprintf(p_fp, "%s FOR PREP\n", p_indent.c_str());
          break;
        case InstABx::OpCode::CLOSURE:
          fprintf(p_fp, "%s CLOSURE\n", p_indent.c_str());
          break;
        // TODO: finish pretty printer
        default:
          fprintf(p_fp, "%s %d\n", p_indent.c_str(), ins.code);
      }
    } else if (index == 2) {
      const auto& ins = std::get<2>(p_ins);
      switch (ins.code) {
        case InstAsBx::OpCode::LOAD_I:
          fprintf(p_fp, "%s LOAD I\n", p_indent.c_str());
          break;
        default:
          fprintf(p_fp, "%s %d\n", p_indent.c_str(), ins.code);
      }
    } else if (index == 3) {
      const auto& ins = std::get<3>(p_ins);
      switch (ins.code) {
        case InstAx::OpCode::EXTRA_ARG:
          fprintf(p_fp, "%s EXTRA ARG\n", p_indent.c_str());
          break;
      }
    } else {
      const auto& ins = std::get<4>(p_ins);
      switch (ins.code) {
        case InstsJ::OpCode::JMP:
          fprintf(p_fp, "%s JMP\n", p_indent.c_str());
          break;
      }
    }
  }
} // namespace instructions
