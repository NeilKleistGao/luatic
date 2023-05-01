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

#include <filesystem>
#include <gtest/gtest.h>
#include <regex>
#include <cstdio>

#include "lua/lua_vm.h"
#include "luatic/lexer.h"

static std::string ReadFile(const std::string& p_filename) {
  FILE* fp = fopen(p_filename.c_str(), "r");
  if (fp == nullptr) {
    return ""; // TODO: err?
  }

  std::string res;
  char buffer[1024];
  while (true) {
    auto size = fread(buffer, sizeof(char), 1024, fp);
    res += buffer;
    if (size < 1024) {
      break;
    }
  }

  return res;
}

TEST(LuaticDiffTests, LuaVM) {
  namespace fs = std::filesystem;
  const auto path = fs::path{"./test/lua"};
  const auto reg = std::regex{"(.*)\\.luac"};
  const auto vm = LuaVM::StartVM();

  for (const auto& fp : fs::directory_iterator(path)) {
    const auto filename = fp.path().string();
    if (std::regex_match(filename, reg)) {
      assert(vm->DoFile(filename) == 0);
    }
  }

  LuaVM::Halt();
}

TEST(LuaticDiffTests, LuaticCompiler) {
  namespace fs = std::filesystem;
  const auto path = fs::path{"./test/ltc"};
  const auto reg = std::regex{"(.*)\\.ltc"};
  const auto vm = LuaVM::StartVM();

  for (const auto& fp : fs::directory_iterator(path)) {
    const auto filename = fp.path().string();
    if (std::regex_match(filename, reg)) {
      const auto lex = Lexer(filename);
      const auto lex_res = lex.Parse(ReadFile(filename));
      assert(lex_res.index() == 0);
    }
  }

  LuaVM::Halt();
}

// TODO: error test
