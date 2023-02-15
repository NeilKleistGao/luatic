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
    virtual bool operator==(int p_code) = 0;
  };

  struct InstABC: public Instruction {
    enum class OpCode {

    } code;

    ushort b;
    ushort c;
    uchar a;

    InstABC(OpCode p_code, ushort p_b, ushort p_a, uchar p_c):
      code(p_code), b(p_b), c(p_c), a(p_a) {}

    bool operator==(int p_code) override {
      return false; // TODO:
    };
  };

  struct InstABx: public Instruction {
    enum class OpCode {

    } code;

    uint bx;
    uchar a;

    InstABx(OpCode p_code, uint p_bx, uchar p_a):
      code(p_code), bx(p_bx), a(p_a) {}

    bool operator==(int p_code) override {
      return false; // TODO:
    };
  };

  struct InstIAsBx: public Instruction {
    enum class OpCode {

    } code;

    int sbx;
    uchar a;

    InstIAsBx(OpCode p_code, int p_sbx, uchar p_a):
      code(p_code), sbx(p_sbx), a(p_a) {}

    bool operator==(int p_code) override {
      return false; // TODO:
    };
  };

  struct InstAx: public Instruction {
    enum class OpCode {

    } code;

    uint ax;

    InstAx(OpCode p_code, uint p_ax): ax(p_ax) {}

    bool operator==(int p_code) override {
      return false; // TODO:
    };
  };
} // namespace instructions

#endif //LUATIC_INSTRUCTIONS_H
