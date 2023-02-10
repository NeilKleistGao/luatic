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

#include "binary_chunk.h"

#include <cstdio>
#include <filesystem>

namespace chunk {
  std::variant<BinaryChunk, std::string>
    ReadAndCheckBinaryChunk(const std::string& p_filename) {
    using namespace std;
    const filesystem::path path = p_filename;
    if (!exists(path)) {
      return "can't find file " + p_filename;
    }

    FILE* fp = fopen(p_filename.c_str(), "rb");
    if (fp == nullptr) {
      return "can't open file " + p_filename;
    }

    const size_t file_size =
      sizeof(BinaryChunk); // TODO: get real file size. this is only for test.
    BinaryChunk chunk{};
    fread(&chunk, file_size, 1, fp);

    const auto& header = chunk.header;
    const auto are_bytes_same =
      [](const size_t& p_size, const byte* const p_b1, const byte* const p_b2) {
        for (int i = 0; i < p_size; ++i) {
          if (p_b1[i] != p_b2[i]) {
            return false;
          }
        }

        return true;
      };

    if (!are_bytes_same(sizeof(MAGIC_NUMBER), header.signature, MAGIC_NUMBER)) {
      return "this is not a lua binary chunk file.";
    }
    if (header.version != VERSION_NUMBER) {
      return "only 5.4.x lua files are supported.";
    }
    if (header.format != FORMAT_NUMBER) {
      return "wrong format.";
    }
    if (!are_bytes_same(sizeof(LUAC_DATA), header.luac_data, LUAC_DATA)) {
      return "wrong luac data.";
    }
    if (header.instruction_size != INSTRUCTION_SIZE) {
      return "wrong instruction size.";
    }
    if (header.lua_integer_size != LUA_INTEGER_SIZE) {
      return "wrong lua integer size.";
    }
    if (header.lua_number_size != LUA_NUMBER_SIZE) {
      return "wrong lua number size.";
    }
    if (header.luac_int != LUAC_INT) {
      return "wrong endian.";
    }
    if (header.luac_num != LUAC_NUMBER) {
      return "wrong float number format.";
    }

    return chunk;
  }
} // namespace chunk
