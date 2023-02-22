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

#include <gtest/gtest.h>
#include <memory>

#include "luna/luna_stack.h"

TEST(LuaticTests, StackTest) {
  auto stack = std::make_shared<LunaStack>(32);
  EXPECT_NE(stack.get(), nullptr);

  stack->Push(LunaNumber{42ll});
  stack->Push(std::string{"abc"});
  EXPECT_EQ(stack->Top(), 2);

  const auto temp1 = stack->Pop();
  EXPECT_TRUE(temp1.has_value());
  EXPECT_EQ(temp1.value().index(), 5); // string
  EXPECT_EQ(std::get<5>(temp1.value()), std::string{"abc"});

  stack->Pop();
  const auto temp2 = stack->Pop();
  EXPECT_FALSE(temp2.has_value());

  stack->Push(false);
  const auto temp3 = stack->Get(1);
  EXPECT_TRUE(temp3.has_value());
  EXPECT_EQ(temp3.value().index(), 2); // boolean
  EXPECT_EQ(std::get<2>(temp3.value()), false);

  const auto temp4 = stack->Get(114514);
  EXPECT_FALSE(temp4.has_value());

  const auto temp5 = stack->Get(-1);
  EXPECT_TRUE(temp5.has_value());
  EXPECT_EQ(temp5.value().index(), 2); // boolean
  EXPECT_EQ(std::get<2>(temp5.value()), false);

  stack->Set(-1, LunaNil{});
  const auto temp6 = stack->Get(-1);
  EXPECT_TRUE(temp6.has_value());
  EXPECT_EQ(temp6.value().index(), 1); // nil

  stack->Set(1, LunaNumber{3.14});
  const auto temp7 = stack->Get(-1);
  EXPECT_TRUE(temp7.has_value());
  EXPECT_EQ(temp7.value().index(), 4); // number

  stack->Push(std::string{"abc"});
  stack->Reverse(1, 2);
  const auto temp8 = stack->Get(-2);
  EXPECT_TRUE(temp8.has_value());
  EXPECT_EQ(temp8.value().index(), 5); // string

  stack->Reverse(-2, -1);
  const auto temp9 = stack->Get(-2);
  EXPECT_TRUE(temp9.has_value());
  EXPECT_EQ(temp9.value().index(), 4); // number

  stack->Reverse(1, -1);
  const auto temp10 = stack->Get(-2);
  EXPECT_TRUE(temp10.has_value());
  EXPECT_EQ(temp10.value().index(), 4); // number
}
