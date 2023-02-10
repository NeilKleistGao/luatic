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

#ifndef LUATIC_BINARY_CHUNK_H
#define LUATIC_BINARY_CHUNK_H

#include <string>
#include <variant>

#pragma pack(1) // cancel the alignment

namespace chunk {
  using byte = unsigned char;

  constexpr byte MAGIC_NUMBER[4] = {27u, 76u, 117u, 97u}; // \x1bLua
  constexpr byte VERSION_NUMBER = 84u; // for 5.4.x
  constexpr byte FORMAT_NUMBER = 0u;
  constexpr byte LUAC_DATA[6] = {25u, 147u, 13u, 10u, 26u, 10u};
  constexpr byte INSTRUCTION_SIZE = 4u;
  constexpr byte LUA_INTEGER_SIZE = 8u;
  constexpr byte LUA_NUMBER_SIZE = 8u;
  constexpr long long LUAC_INT = 0x5678;
  constexpr double LUAC_NUMBER = 370.5;

  struct Header {
    byte signature[4];
    byte version;
    byte format;
    byte luac_data[6];
    byte instruction_size;
    byte lua_integer_size;
    byte lua_number_size;
    long long luac_int;
    double luac_num;
  };

  struct BinaryChunk {
    Header header;
  };

  std::variant<BinaryChunk, std::string>
    ReadAndCheckBinaryChunk(const std::string& p_filename);
} // namespace chunk

#pragma pack()

#endif //LUATIC_BINARY_CHUNK_H
