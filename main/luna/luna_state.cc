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

#include "luna_state.h"

void LunaState::PopN(size_t p_n) {
  for (auto i = 0; i < p_n; ++i) {
    m_stack->Pop();
  }
}

void LunaState::PushAt(int p_index) {
  const auto val = m_stack->Get(p_index);
  if (val.has_value()) {
    m_stack->Push(val.value());
  }
  // TODO: throw?
}

void LunaState::Rotate(int p_index, int p_n) {
  const auto top = static_cast<int>(m_stack->Top());
  const auto pos = p_index;
  const auto pilot = (p_n >= 0) ? top - p_n : pos - p_n - 1;
  m_stack->Reverse(pos, pilot);
  m_stack->Reverse(pilot + 1, top);
  m_stack->Reverse(pos, top);
}

void LunaState::Remove(int p_index) {
  this->Rotate(p_index, -1);
  this->PopN(1);
}

void LunaState::SetTop(size_t p_new_top) {
  const auto top = m_stack->Top();
  --p_new_top;
  const auto sub = static_cast<long>(p_new_top) - top;
  if (sub > 0) {
    for (auto i = 0; i < p_new_top - top; ++i) {
      m_stack->Push(LunaNil{});
    }
  } else if (sub < 0) {
    this->PopN(-sub);
  }
}

LunaType LunaState::GetTypeAt(int p_index) {
  const auto& res = m_stack->Get(p_index);
  if (res.has_value()) {
    return GetTypeOf(res.value());
  }

  return LunaType::LUNA_NONE;
}
