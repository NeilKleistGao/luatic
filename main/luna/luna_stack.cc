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
  m_slots.push_back(p_value);
}

void LunaStack::Push(LunaValue&& p_value) {
  m_slots.push_back(std::move(p_value));
}

std::optional<LunaValue> LunaStack::Pop() {
  if (m_slots.empty()) {
    return {};
  }

  const auto res = m_slots.back();
  m_slots.pop_back();
  return res;
}

std::optional<LunaValue> LunaStack::Get(int p_index) {
  const auto abs_index = GetAbsIndex(p_index);
  if (IsValidIndex(abs_index)) {
    return m_slots[abs_index];
  } else {
    return {};
  }
}

void LunaStack::Set(int p_index, const LunaValue& p_value) {
  const auto abs_index = GetAbsIndex(p_index);
  if (IsValidIndex(abs_index)) {
    m_slots[abs_index] = p_value;
  }

  // TODO: throw error
}

void LunaStack::Set(int p_index, LunaValue&& p_value) {
  const auto abs_index = GetAbsIndex(p_index);
  if (IsValidIndex(abs_index)) {
    m_slots[abs_index] = std::move(p_value);
  }

  // TODO: throw error
}

size_t LunaStack::GetAbsIndex(int p_index) {
  return static_cast<size_t>((p_index > -1) ? p_index - 1
                                            : m_slots.size() + p_index);
}

void LunaStack::Reverse(int p_from, int p_to) {
  if (GetAbsIndex(p_from) == GetAbsIndex(p_to)) {
    return;
  }

  while (p_from < p_to) {
    std::swap(m_slots[p_from], m_slots[p_to]);
    ++p_from;
    --p_to;
  }
}
