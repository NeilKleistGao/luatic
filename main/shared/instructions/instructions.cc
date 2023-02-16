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
  bool InstABC::operator==(uint p_code) {
    return false; // TODO:
  }

  InstABC::InstABC(uint p_code) {}

  bool InstABx::operator==(uint p_code) {
    return false; // TODO:
  }

  InstABx::InstABx(uint p_code) {}

  bool InstAsBx::operator==(uint p_code) {
    return false; // TODO:
  }

  InstAsBx::InstAsBx(uint p_code) {}

  bool InstAx::operator==(uint p_code) {
    return false; // TODO:
  }

  InstAx::InstAx(uint p_code) {}

  bool InstsJ::operator==(uint p_code) { return false; }

  InstsJ::InstsJ(uint p_code) {}
} // namespace instructions
