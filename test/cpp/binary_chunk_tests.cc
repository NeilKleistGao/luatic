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
#include <filesystem>
#include <gtest/gtest.h>
#include <unordered_set>

#include "binary_pretty_printer.h"
#include "shared/chunk/binary_chunk.h"

// help you skip some tests
static const std::unordered_set<std::string> TEST_FILTER = {};

TEST(LuaticTests, BinaryChunkTest) {
  const auto dir = std::filesystem::path{"../test/luac"};
  const auto out_dir = std::filesystem::path{"../test/dump"};
  for (const auto& it : std::filesystem::directory_iterator{dir}) {
    const auto filename = it.path().filename().string();
    if (TEST_FILTER.find(filename) != TEST_FILTER.end()) {
      continue;
    }

    std::cout << "check binary chunk in " << filename << std::endl;
    const auto path = it.path().string();
    const auto res = chunk::ReadAndCheckBinaryChunk(path);

    // luac -> lua
    const auto output =
      (out_dir / filename.substr(0, filename.length() - 1)).string();
    FILE* fp = fopen(output.c_str(), "w");
    EXPECT_NE(fp, nullptr);
    if (res.index() == 0) {
      chunk::PrintChunk(fp, std::get<0>(res));
    } else {
      fputs(("-- " + std::get<1>(res)).c_str(), fp);
    }

    fclose(fp);
  }
}
