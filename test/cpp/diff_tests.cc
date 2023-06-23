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

#include <cstdio>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <gtest/gtest.h>
#include <regex>

#include "lua/lua_vm.h"
#include "luatic/lexer.h"
#include "luatic/parser.h"
#include "pretty_printer.h"

static std::string ReadFile(const std::string& p_filename) {
  FILE* fp = fopen(p_filename.c_str(), "r");
  if (fp == nullptr) {
    return ""; // TODO: err?
  }

  std::string res;
  const size_t buffer_size = 1024;
  char buffer[buffer_size];
  while (true) {
    std::memset(buffer, 0, sizeof(buffer));
    auto size = fread(buffer, sizeof(char), buffer_size - 1, fp);
    res += buffer;
    if (size < buffer_size - 1) {
      break;
    }
  }

  return res;
}

TEST(LuaticDiffTests, LuaVM) {
  namespace fs = std::filesystem;
  const auto path = fs::path{"./test/lua"};
  const auto reg = std::regex{R"((.*)\\.luac)"};
  const auto vm = LuaVM::StartVM();

  for (const auto& fp : fs::directory_iterator(path)) {
    const auto filename = fp.path().string();
    if (std::regex_match(filename, reg)) {
      ASSERT_EQ(vm->DoFile(filename), 0);
    }
  }

  LuaVM::Halt();
}

TEST(LuaticDiffTests, LuaticCompiler) {
  namespace fs = std::filesystem;
  const auto path = fs::path{"./test/ltc"};
  const auto reg = std::regex{R"((.*)\.ltc)"};
  const auto vm = LuaVM::StartVM();

  bool success = true;
  for (const auto& fp : fs::directory_iterator(path)) {
    const auto filename = fp.path().string();
    if (std::regex_match(filename, reg)) {
      const auto lex = Lexer(filename, ReadFile(filename));
      const auto lex_res = lex.Parse();
      if (lex_res.index() != 0) {
        success = false;
        const auto diags = std::get<1>(lex_res);
        for (const auto& diag : diags) {
          PrintDiagnostic(diag);
        }

        continue;
      }

      const auto parser = Parser(filename, std::get<0>(lex_res));
      const auto parse_res = parser.Parse();
      if (parse_res.index() != 0) { // TODO:
        //        success = false;
        //        const auto diags = std::get<1>(parse_res);
        //        for (const auto& diag : diags) {
        //          PrintDiagnostic(diag);
        //        }
      }
    }
  }

  LuaVM::Halt();
  ASSERT_TRUE(success);
}

TEST(LuaticDiffTests, LuaticLexError) {
  namespace fs = std::filesystem;
  const auto path = fs::path{"./test/error"};
  const auto reg = std::regex{R"(.*lex([0-9]+)\.ltc)"};
  const auto vm = LuaVM::StartVM();

  bool success = true;
  for (const auto& fp : fs::directory_iterator(path)) {
    const auto filename = fp.path().string();
    if (std::regex_match(filename, reg)) {
      const auto lex = Lexer(filename, ReadFile(filename));
      const auto lex_res = lex.Parse();
      if (lex_res.index() != 0) {
        const auto diags = std::get<1>(lex_res);
        const auto backup = std::cerr.rdbuf();
        std::ofstream log{filename.substr(0, filename.find(".ltc")) + ".log"};
        std::cerr.rdbuf(log.rdbuf());
        for (const auto& diag : diags) {
          PrintDiagnostic(diag);
        }

        std::cerr.rdbuf(backup);
        log.close();
      } else {
        success = false;
      }
    }
  }

  LuaVM::Halt();
  ASSERT_TRUE(success);
}

// TODO: more error test
// TODO: module test
