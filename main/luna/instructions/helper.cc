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

#include "helper.h"

namespace instructions {
  LunaNumber CalcArith(math::ArithOperator p_ao, LunaValue p1, LunaValue p2) {
    if (p1.index() == LunaType::LUNA_NUMBER &&
        p2.index() == LunaType::LUNA_NUMBER) {
      return math::CalcArith(p_ao,
                             std::get<LunaType::LUNA_NUMBER>(p1),
                             std::get<LunaType::LUNA_NUMBER>(p2));
    } else {
      return 0.0; // TODO: throw?
    }
  }

  LunaInt Mod(LunaValue p1, LunaValue p2) {
    if (p1.index() == LunaType::LUNA_NUMBER &&
        p2.index() == LunaType::LUNA_NUMBER) {
      const auto& n1 = std::get<LunaType::LUNA_NUMBER>(p1);
      const auto& n2 = std::get<LunaType::LUNA_NUMBER>(p2);
      if (n1.index() == LUNA_INT && n2.index() == LUNA_INT) {
        return math::Mod(std::get<LUNA_INT>(n1), std::get<LUNA_INT>(n2));
      }
    }
    return 0; // TODO: throw?
  }

  LunaNumber Neg(LunaValue p) {
    if (p.index() == LunaType::LUNA_NUMBER) {
      return math::Neg(std::get<LunaType::LUNA_NUMBER>(p));
    } else {
      return 0.0; // TODO: throw?
    }
  }

  LunaBoolean Not(LunaValue p) {
    if (p.index() == LunaType::LUNA_BOOLEAN) {
      return math::Not(std::get<LunaType::LUNA_BOOLEAN>(p));
    } else {
      return false; // TODO: throw?
    }
  }

  LunaInt Len(LunaValue p) {
    if (p.index() == LunaType::LUNA_STRING) {
      return static_cast<LunaInt>(std::get<LunaType::LUNA_STRING>(p).size());
      // TODO: tables
    } else {
      return 0; // TODO: throw?
    }
  }

  LunaValue Concat(LunaValue p1, LunaValue p2) {
    if (p1.index() == LunaType::LUNA_STRING &&
        p2.index() == LunaType::LUNA_STRING) {
      const auto& s1 = std::get<LunaType::LUNA_STRING>(p1);
      const auto& s2 = std::get<LunaType::LUNA_STRING>(p2);
      return LunaValue{s1 + s2};
    } else {
      return LunaValue{0}; // TODO: throw?
    }
  }

  void UpdateForCount(const std::shared_ptr<LunaStack>& p_stack, int p_base) {
    const auto next = CalcArith(math::ArithOperator::ADD,
                                p_stack->Get(p_base),
                                p_stack->Get(p_base + 2));

    p_stack->Set(p_base, next);
  }

  bool ShouldSkipForLoop(const std::shared_ptr<LunaStack>& p_stack,
                         int p_base) {
    return math::Compare(math::ComOperator::LE,
                         p_stack->Get(p_base),
                         p_stack->Get(p_base + 1));
  }

  LunaValue GetFromStackOrConst(const std::shared_ptr<LunaStack>& p_stack,
                                const std::vector<chunk::Literal>& p_const,
                                int p_index) {
    if (p_index > 0xff) {
      if ((p_index & 0xff) < p_const.size()) {
        return FromLiteral(p_const[p_index & 0xff]);
      } else {
        return LunaNone{};
      }
    } else {
      return p_index;
    }
  }
} // namespace instructions
