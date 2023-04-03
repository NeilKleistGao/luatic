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

#include "luna_values.h"

LunaType GetTypeOf(const LunaValue& p_value) {
  const auto index = p_value.index();
  if (index > 0 && index < 10) {
    return static_cast<LunaType>(index);
  } else {
    return LunaType::LUNA_NONE;
  }
}

std::string GetTypeName(LunaType p_type) {
  switch (p_type) {
    case LunaType::LUNA_NONE:
      return "no value";
    case LunaType::LUNA_NIL:
      return "nil";
    case LunaType::LUNA_BOOLEAN:
      return "boolean";
    case LunaType::LUNA_NUMBER:
      return "number";
    case LunaType::LUNA_STRING:
      return "string";
    case LunaType::LUNA_TABLE:
      return "table";
    case LunaType::LUNA_FUNCTION:
      return "function";
    case LunaType::LUNA_THREAD:
      return "thread";
    default:
      return "userdata";
  }
}

LunaValue FromLiteral(const chunk::Literal& p_lit) {
  switch (p_lit.index()) {
    case 0:
      return std::get<0>(p_lit);
    case 1:
      return std::get<1>(p_lit);
    case 2:
      return std::get<2>(p_lit);
    case 3:
      return std::get<3>(p_lit);
    case 4:
      return std::get<4>(p_lit);
  }

  // TODO: throw?
}

LunaBoolean ToBoolean(const LunaValue& p_value) {
  const auto type = static_cast<LunaType>(p_value.index());
  switch (type) {
    case LunaType::LUNA_BOOLEAN:
      return std::get<LunaType::LUNA_BOOLEAN>(p_value);
    case LunaType::LUNA_NIL:
      return false;
    default:
      return true;
  }
}

size_t LunaHash::operator()(const LunaValue& p_value) const {
  const auto type = static_cast<LunaType>(p_value.index());
  switch (type) {
    case LunaType::LUNA_NONE:
    case LunaType::LUNA_NIL:
      return 0; // TODO: throw?
    case LunaType::LUNA_BOOLEAN:
    case LunaType::LUNA_NUMBER:
    case LunaType::LUNA_STRING:
    case LunaType::LUNA_TABLE:
    case LunaType::LUNA_FUNCTION:
    case LunaType::LUNA_THREAD:
    default:
      return 0; // TODO: check the hash algorithm
  }
}
