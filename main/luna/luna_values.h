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

#ifndef LUATIC_LUNA_VALUES_H
#define LUATIC_LUNA_VALUES_H

#include <memory>
#include <string>
#include <unordered_map>
#include <utility>
#include <variant>
#include <vector>

#include "shared/chunk/literal.h"

enum LunaType {
  LUNA_NONE,
  LUNA_NIL,
  LUNA_BOOLEAN,
  LUNA_LIGHT_USERDATA,
  LUNA_NUMBER,
  LUNA_STRING,
  LUNA_TABLE,
  LUNA_FUNCTION,
  LUNA_USERDATA,
  LUNA_THREAD
};

// This type is designed to break the cycle dependency
struct LunaValue2;
using LunaPointer = std::shared_ptr<LunaValue2>;

struct LunaNone {};
using LunaNil = std::nullptr_t;
using LunaBoolean = bool;
struct LunaLightUserdata {};
using LunaFloat = double;
using LunaInt = long long;
using LunaNumber = std::variant<LunaFloat, LunaInt>;
using LunaString = std::string;
using LunaTable = std::variant<std::vector<LunaPointer>,
                               std::unordered_map<LunaPointer, LunaPointer>>;
struct LunaFunction {};
struct LunaUserdata {};
struct LunaThread {};

using LunaValue = std::variant<LunaNone,
                               LunaNil,
                               LunaBoolean,
                               LunaLightUserdata,
                               LunaNumber,
                               LunaString,
                               LunaTable,
                               LunaFunction,
                               LunaUserdata,
                               LunaThread>;

struct LunaValue2 {
  LunaValue value;

  explicit LunaValue2(LunaValue p_value): value(std::move(p_value)) {}
};

constexpr size_t LUNA_FLOAT = 0;
constexpr size_t LUNA_INT = 1;

LunaType GetTypeOf(const LunaValue& p_value);
std::string GetTypeName(LunaType p_type);
LunaValue FromLiteral(const chunk::Literal& p_lit);

// This function is only for compatibility
LunaBoolean ToBoolean(const LunaValue& p_value);

#endif //LUATIC_LUNA_VALUES_H
