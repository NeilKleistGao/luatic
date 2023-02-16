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
    static constexpr uint OP_CODE_MASK = 0b00111111;
    virtual bool operator==(uint p_code) = 0;
  };

  struct InstABC: public Instruction {
    enum class OpCode {
      MOVE = 0,
      LOAD_BOOL = 3,
      LOAD_INT = 4,
      GET_UP_VAL = 5,
      GET_TAB_UP = 6,
      GET_TABLE = 7,
      SET_TAB_UP = 8,
      SET_UP_VAL = 9,
      SET_TABLE = 10,
      NEW_TABLE = 11,
      SELF = 12,
      ADD = 13,
      SUB = 14,
      MUL = 15,
      MOD = 16,
      POW = 17,
      DIV = 18,
      IDIV = 19,
      BAND = 20,
      BOR = 21,
      BXOR = 22,
      SHL = 23,
      SHR = 24,
      UNM = 25,
      BNOT = 26,
      NOT = 27,
      LEN = 28,
      CONCAT = 29,
      EQ = 31,
      LT = 32,
      LE = 33,
      TEST = 34,
      TEST_SET = 35,
      CALL = 36,
      TAIL_CALL = 37,
      RETURN = 38,
      TFOR_CALL = 41,
      SET_LIST = 43,
      VAR_ARG = 45
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
    enum class OpCode { LOAD_K = 1, LOAD_KX = 2, CLOSURE = 44 } code;

    uint bx;
    uchar a;

    explicit InstABx(uint p_code);
    InstABx(OpCode p_code, uint p_bx, uchar p_a):
      code(p_code), bx(p_bx), a(p_a) {}

    bool operator==(uint p_code) override;
  };

  struct InstIAsBx: public Instruction {
    enum class OpCode {
      JMP = 30,
      FOR_LOOP = 39,
      FOR_PREP = 40,
      TFOR_LOOP = 42
    } code;

    int sbx;
    uchar a;

    explicit InstIAsBx(uint p_code);
    InstIAsBx(OpCode p_code, int p_sbx, uchar p_a):
      code(p_code), sbx(p_sbx), a(p_a) {}

    bool operator==(uint p_code) override;
  };

  struct InstAx: public Instruction {
    enum class OpCode { EXTRA_ARG = 46 } code;

    uint ax;

    explicit InstAx(uint p_code);
    InstAx(OpCode p_code, uint p_ax): code(p_code), ax(p_ax) {}

    bool operator==(uint p_code) override;
  };
} // namespace instructions

#endif //LUATIC_INSTRUCTIONS_H
