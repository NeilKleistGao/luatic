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

#ifndef LUATIC_INSTRUCTIONS_H
#define LUATIC_INSTRUCTIONS_H

namespace instructions {
  using uint = unsigned int;
  using ushort = unsigned short;
  using uchar = unsigned char;

  struct Instruction {
    static constexpr uint OP_CODE_MASK = 0b01111111;
    virtual bool operator==(uint p_code) = 0;
  };

  struct InstABC: public Instruction {
    enum class OpCode {
      MOVE = 0,
      LOAD_FALSE = 5,
      LFALSE_SKIP = 6,
      LOAD_TRUE = 7,
      LOAD_NIL = 8,
      GET_UP_VAL = 9,
      SET_UP_VAL = 10,
      GET_TAB_UP = 11,
      GET_TABLE = 12,
      GET_I = 13,
      GET_FIELD = 14,
      SET_TAB_UP = 15,
      SET_TABLE = 16,
      SET_I = 17,
      SET_FIELD = 18,
      NEW_TABLE = 19,
      SELF = 20,
      ADD_I = 21,
      ADD_K = 22,
      SUB_K = 23,
      MUL_K = 24,
      MOD_K = 25,
      POW_K = 26,
      DIV_K = 27,
      I_DIV_K = 28,
      B_AND_K = 29,
      B_OR_K = 30,
      B_XOR_K = 31,
      SHR_I = 32,
      SHL_I = 33,
      ADD = 34,
      SUB = 35,
      MUL = 36,
      MOD = 37,
      POW = 38,
      DIV = 39,
      I_DIV = 40,
      B_AND = 41,
      B_OR = 42,
      B_XOR = 43,
      SHL = 44,
      SHR = 45,
      MM_BIN = 46,
      MM_BIN_I = 47,
      MM_BIN_K = 48,
      UNM = 49,
      B_NOT = 50,
      NOT = 51,
      LEN = 52,
      CONCAT = 53,
      CLOSE = 54,
      TBC = 55,
      EQ = 57,
      LT = 58,
      LE = 59,
      EQ_K = 60,
      EQ_I = 61,
      LT_I = 62,
      LE_I = 63,
      GT_I = 64,
      GE_I = 65,
      TEST = 66,
      TEST_SET = 67,
      CALL = 68,
      TAIL_CALL = 69,
      RETURN = 70,
      RETURN_0 = 71,
      RETURN_1 = 72,
      T_FOR_CALL = 76,
      SET_LIST = 78,
      VAR_ARG = 80,
      VAR_ARG_PREP = 81
    } code;

    ushort b;
    ushort c;
    uchar a;

    explicit InstABC(uint p_code);
    InstABC(OpCode p_code, ushort p_b, ushort p_a, uchar p_c):
      code(p_code), b(p_b), c(p_c), a(p_a) {}

    bool operator==(uint p_code) override;
  };

  struct InstABx: public Instruction {
    enum class OpCode {
      LOAD_K = 3,
      LOAD_KX = 4,
      FOR_LOOP = 73,
      FOR_PREP = 74,
      T_FOR_PREP = 75,
      T_FOR_LOOP = 77,
      CLOSURE = 79
    } code;

    uint bx;
    uchar a;

    explicit InstABx(uint p_code);
    InstABx(OpCode p_code, uint p_bx, uchar p_a):
      code(p_code), bx(p_bx), a(p_a) {}

    bool operator==(uint p_code) override;
  };

  struct InstAsBx: public Instruction {
    enum class OpCode { LOAD_I = 1, LOAD_F = 2 } code;

    int sbx;
    uchar a;

    explicit InstAsBx(uint p_code);
    InstAsBx(OpCode p_code, int p_sbx, uchar p_a):
      code(p_code), sbx(p_sbx), a(p_a) {}

    bool operator==(uint p_code) override;
  };

  struct InstAx: public Instruction {
    enum class OpCode { EXTRA_ARG = 82 } code;

    uint ax;

    explicit InstAx(uint p_code);
    InstAx(OpCode p_code, uint p_ax): code(p_code), ax(p_ax) {}

    bool operator==(uint p_code) override;
  };

  struct InstsJ: public Instruction {
    enum class OpCode { JMP = 56 } code;

    uint sj;

    explicit InstsJ(uint p_code);
    InstsJ(OpCode p_code, uint p_sj): code(p_code), sj(p_sj) {}

    bool operator==(uint p_code) override;
  };
} // namespace instructions

#endif //LUATIC_INSTRUCTIONS_H
