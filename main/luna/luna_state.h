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

#ifndef LUATIC_LUNA_STATE_H
#define LUATIC_LUNA_STATE_H

#include <memory>

#include "luna_stack.h"

class LunaState {
public:
  LunaState(): m_stack(new LunaStack{32}){};

  inline size_t Top() const { return m_stack->Top() + 1; }

  void PopN(size_t p_n);
  void Copy(int p_from, int p_to);
  void PushAt(int p_index);
  void ReplaceWithTop(int p_index);
  void Rotate(int p_index, int p_n);
  void Remove(int p_index);
  void SetTop(size_t p_new_top);

  inline void InsertTop(int p_index) { this->Rotate(p_index, 1); }
  inline void PushNil() { m_stack->Push(LunaNil{}); }

private:
  std::shared_ptr<LunaStack> m_stack;
};

#endif //LUATIC_LUNA_STATE_H
