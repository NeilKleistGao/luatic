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

#include "instructions.h"

namespace instructions {
  bool InstABC::Is(uint p_code) {
    switch (p_code & OP_CODE_MASK) {
      case static_cast<int>(OpCode::MOVE):
      case static_cast<int>(OpCode::LOAD_FALSE):
      case static_cast<int>(OpCode::L_FALSE_SKIP):
      case static_cast<int>(OpCode::LOAD_TRUE):
      case static_cast<int>(OpCode::LOAD_NIL):
      case static_cast<int>(OpCode::GET_UP_VAL):
      case static_cast<int>(OpCode::SET_UP_VAL):
      case static_cast<int>(OpCode::GET_TAB_UP):
      case static_cast<int>(OpCode::GET_TABLE):
      case static_cast<int>(OpCode::GET_I):
      case static_cast<int>(OpCode::GET_FIELD):
      case static_cast<int>(OpCode::SET_TAB_UP):
      case static_cast<int>(OpCode::SET_TABLE):
      case static_cast<int>(OpCode::SET_I):
      case static_cast<int>(OpCode::SET_FIELD):
      case static_cast<int>(OpCode::NEW_TABLE):
      case static_cast<int>(OpCode::SELF):
      case static_cast<int>(OpCode::ADD_I):
      case static_cast<int>(OpCode::ADD_K):
      case static_cast<int>(OpCode::SUB_K):
      case static_cast<int>(OpCode::MUL_K):
      case static_cast<int>(OpCode::MOD_K):
      case static_cast<int>(OpCode::POW_K):
      case static_cast<int>(OpCode::DIV_K):
      case static_cast<int>(OpCode::I_DIV_K):
      case static_cast<int>(OpCode::B_AND_K):
      case static_cast<int>(OpCode::B_OR_K):
      case static_cast<int>(OpCode::B_XOR_K):
      case static_cast<int>(OpCode::SHR_I):
      case static_cast<int>(OpCode::SHL_I):
      case static_cast<int>(OpCode::ADD):
      case static_cast<int>(OpCode::SUB):
      case static_cast<int>(OpCode::MUL):
      case static_cast<int>(OpCode::MOD):
      case static_cast<int>(OpCode::POW):
      case static_cast<int>(OpCode::DIV):
      case static_cast<int>(OpCode::I_DIV):
      case static_cast<int>(OpCode::B_AND):
      case static_cast<int>(OpCode::B_OR):
      case static_cast<int>(OpCode::B_XOR):
      case static_cast<int>(OpCode::SHL):
      case static_cast<int>(OpCode::SHR):
      case static_cast<int>(OpCode::MM_BIN):
      case static_cast<int>(OpCode::MM_BIN_I):
      case static_cast<int>(OpCode::MM_BIN_K):
      case static_cast<int>(OpCode::UNM):
      case static_cast<int>(OpCode::B_NOT):
      case static_cast<int>(OpCode::NOT):
      case static_cast<int>(OpCode::LEN):
      case static_cast<int>(OpCode::CONCAT):
      case static_cast<int>(OpCode::CLOSE):
      case static_cast<int>(OpCode::TBC):
      case static_cast<int>(OpCode::EQ):
      case static_cast<int>(OpCode::LT):
      case static_cast<int>(OpCode::LE):
      case static_cast<int>(OpCode::EQ_K):
      case static_cast<int>(OpCode::EQ_I):
      case static_cast<int>(OpCode::LT_I):
      case static_cast<int>(OpCode::LE_I):
      case static_cast<int>(OpCode::GT_I):
      case static_cast<int>(OpCode::GE_I):
      case static_cast<int>(OpCode::TEST):
      case static_cast<int>(OpCode::TEST_SET):
      case static_cast<int>(OpCode::CALL):
      case static_cast<int>(OpCode::TAIL_CALL):
      case static_cast<int>(OpCode::RETURN):
      case static_cast<int>(OpCode::RETURN_0):
      case static_cast<int>(OpCode::RETURN_1):
      case static_cast<int>(OpCode::T_FOR_CALL):
      case static_cast<int>(OpCode::SET_LIST):
      case static_cast<int>(OpCode::VAR_ARG):
      case static_cast<int>(OpCode::VAR_ARG_PREP):
        return true;
      default:
        return false;
    }
  }

  InstABC::InstABC(uint p_code):
    code(static_cast<OpCode>(p_code & OP_CODE_MASK)),
    a(static_cast<uchar>((p_code >> 7) & 0xFF)), k(((p_code >> 15) & 1) > 0),
    b(static_cast<uchar>((p_code >> 16) & 0xFF)),
    c(static_cast<uchar>((p_code >> 24) & 0xFF)) {}

  bool InstABx::Is(uint p_code) {
    switch (p_code & OP_CODE_MASK) {
      case static_cast<int>(OpCode::LOAD_K):
      case static_cast<int>(OpCode::LOAD_KX):
      case static_cast<int>(OpCode::FOR_LOOP):
      case static_cast<int>(OpCode::FOR_PREP):
      case static_cast<int>(OpCode::T_FOR_PREP):
      case static_cast<int>(OpCode::T_FOR_LOOP):
      case static_cast<int>(OpCode::CLOSURE):
        return true;
      default:
        return false;
    }
  }

  InstABx::InstABx(uint p_code):
    code(static_cast<OpCode>(p_code & OP_CODE_MASK)),
    a(static_cast<uchar>((p_code >> 7) & 0xFF)),
    bx(static_cast<ushort>((p_code >> 15) & 0x1FFFF)) {}

  bool InstAsBx::Is(uint p_code) {
    switch (p_code & OP_CODE_MASK) {
      case static_cast<int>(OpCode::LOAD_I):
      case static_cast<int>(OpCode::LOAD_F):
        return true;
      default:
        return false;
    }
  }

  InstAsBx::InstAsBx(uint p_code):
    code(static_cast<OpCode>(p_code & OP_CODE_MASK)),
    a(static_cast<uchar>((p_code >> 7) & 0xFF)),
    sbx(static_cast<short>((p_code >> 15) & 0x1FFFF)) {}

  bool InstAx::Is(uint p_code) {
    return (p_code & OP_CODE_MASK) == static_cast<int>(OpCode::EXTRA_ARG);
  }

  InstAx::InstAx(uint p_code):
    code(static_cast<OpCode>(p_code & OP_CODE_MASK)),
    ax(static_cast<uint>((p_code >> 15) & 0x1FFFFFF)) {}

  bool InstsJ::Is(uint p_code) {
    return (p_code & OP_CODE_MASK) == static_cast<int>(OpCode::JMP);
  }

  InstsJ::InstsJ(uint p_code):
    code(static_cast<OpCode>(p_code & OP_CODE_MASK)),
    sj(static_cast<uint>((p_code >> 15) & 0x1FFFFFF)) {}

  std::variant<Instruction, Error> Parse(uint p_code) {
    if (InstABC::Is(p_code)) {
      return InstABC{p_code};
    } else if (InstABx::Is(p_code)) {
      return InstABx{p_code};
    } else if (InstAsBx::Is(p_code)) {
      return InstAsBx{p_code};
    } else if (InstAx::Is(p_code)) {
      return InstAx{p_code};
    } else if (InstsJ::Is(p_code)) {
      return InstsJ{p_code};
    } else {
      return "wrong operation code.";
    }
  }
} // namespace instructions
