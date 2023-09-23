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
#include "luatic/parser.h"
#include "luatic/tokenizer.h"
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

TEST(LuaticDiffTests, LuaticInterpreter) {
  namespace fs = std::filesystem;
  const auto path = fs::path{"./test/ltc"};
  const auto reg = std::regex{R"((.*)\.ltc)"};
  const auto vm = LuaVM::StartVM();
  const auto backup = std::cout.rdbuf();

  for (const auto& fp : fs::directory_iterator(path)) {
    const auto filename = fp.path().string();
    if (std::regex_match(filename, reg)) {
      std::cout << "> test " << fp.path().filename().string() << "..."
                << std::endl;
      std::ofstream log{filename.substr(0, filename.find(".ltc")) + ".check"};
      std::cout.rdbuf(log.rdbuf());

      auto tokenizer = Tokenizer{filename, ReadFile(filename)};
      const auto lex_res = tokenizer.Parse();
      if (lex_res.index() != 0) {
        const auto diags = std::get<Tokenizer::DiagnosticList>(lex_res);
        for (const auto& diag : diags) {
          PrintDiagnostic(diag);
        }

        continue;
      }

      auto tokens = std::get<Parser::TokenStream>(lex_res);
      auto parser = Parser{filename, std::move(tokens)};
      const auto parse_res = parser.Parse();
      if (parse_res.index() != 0) {
        const auto diags = std::get<Parser::DiagnosticList>(lex_res);
        for (const auto& diag : diags) {
          PrintDiagnostic(diag);
        }

        continue;
      }

      // TODO:
    }
  }

  std::cout.rdbuf(backup);
  LuaVM::Halt();
}
