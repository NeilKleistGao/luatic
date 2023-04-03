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

#include "luna_stack.h"

LunaStack::LunaStack(size_t p_capacity) {
  m_slots.reserve(p_capacity);
}

void LunaStack::Push(const LunaValue& p_value) {
  m_slots.emplace_back(p_value);
}

LunaValue LunaStack::Pop() {
  if (m_slots.empty()) {
    return LunaNone{};
  }

  const auto res = m_slots.back();
  m_slots.pop_back();
  return res;
}

LunaValue LunaStack::Get(int p_index) {
  const auto abs_index = GetAbsIndex(p_index);
  if (IsValidIndex(abs_index)) {
    return m_slots[abs_index];
  } else {
    return LunaNone{};
  }
}

void LunaStack::Set(int p_index, const LunaValue& p_value) {
  const auto abs_index = GetAbsIndex(p_index);
  if (IsValidIndex(abs_index)) {
    m_slots[abs_index] = LunaValue{p_value};
  }

  // TODO: throw error
}

size_t LunaStack::GetAbsIndex(int p_index) {
  return static_cast<size_t>((p_index > -1) ? p_index - 1
                                            : m_slots.size() + p_index);
}

void LunaStack::Reverse(int p_from, int p_to) {
  while (p_from < p_to) {
    const auto from = GetAbsIndex(p_from);
    const auto to = GetAbsIndex(p_to);
    if (from == to) {
      return;
    }

    std::swap(m_slots[from], m_slots[to]);
    ++p_from;
    --p_to;
  }
}

void LunaStack::Copy(int p_from, int p_to) {
  const auto val = this->Get(p_from);
  this->Set(p_to, val);
}

void LunaStack::ReplaceWithTop(int p_index) {
  const auto top = this->Pop();
  this->Set(p_index, top);
}
