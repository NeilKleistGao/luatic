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

#include "instructions_impl.h"

#include "abc_impl.h"

namespace instructions {
  static int Execute(const InstABC& p_inst,
                     const std::shared_ptr<LunaStack>& p_stack) {
    switch (p_inst.code) {
      case InstABC::OpCode::MOVE:
        return Move(p_inst, p_stack);
      case InstABC::OpCode::RETURN:
        return Return(p_inst, p_stack);
      case InstABC::OpCode::VAR_ARG_PREP:
        return PrepareVarArgs(p_inst, p_stack);
      default:
        return 0;
    }
  }

  static int Execute(const InstABx& p_inst,
                     const std::shared_ptr<LunaStack>& p_stack) {
    switch (p_inst.code) {
      default:
        return 0;
    }
  }

  static int Execute(const InstAsBx& p_inst,
                     const std::shared_ptr<LunaStack>& p_stack) {
    switch (p_inst.code) {
      default:
        return 0;
    }
  }

  static int Execute(const InstAx& p_inst,
                     const std::shared_ptr<LunaStack>& p_stack) {
    switch (p_inst.code) {
      default:
        return 0;
    }
  }

  static int Execute(const InstsJ& p_inst,
                     const std::shared_ptr<LunaStack>& p_stack) {
    switch (p_inst.code) {
      default:
        return 0;
    }
  }

  int Execute(const Instruction& p_inst,
              const std::shared_ptr<LunaStack>& p_stack) {
    switch (p_inst.index()) {
      case 0:
        return Execute(std::get<0>(p_inst), p_stack);
      case 1:
        return Execute(std::get<1>(p_inst), p_stack);
      case 2:
        return Execute(std::get<2>(p_inst), p_stack);
      case 3:
        return Execute(std::get<3>(p_inst), p_stack);
      default:
        return Execute(std::get<4>(p_inst), p_stack);
    }
  }
} // namespace instructions
