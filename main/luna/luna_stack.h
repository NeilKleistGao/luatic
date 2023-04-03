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

#ifndef LUATIC_LUNA_STACK_H
#define LUATIC_LUNA_STACK_H

#include <vector>

#include "luna_values.h"

class LunaStack {
public:
  explicit LunaStack(size_t p_capacity);

  void Push(const LunaValue& p_value);
  LunaValue Pop();

  inline size_t Top() const { return m_slots.size(); }

  LunaValue Get(int p_index);
  void Set(int p_index, const LunaValue& p_value);
  void Reverse(int p_from, int p_to);
  void Copy(int p_from, int p_to);
  void ReplaceWithTop(int p_index);

private:
  std::vector<LunaValue> m_slots;

  size_t GetAbsIndex(int p_index);
  inline bool IsValidIndex(size_t p_index) { return p_index < m_slots.size(); }
};

#endif //LUATIC_LUNA_STACK_H
