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

namespace chunk {
  size_t ReadVarInt(FILE* p_fp) {
    size_t x = 0;
    byte b;
    do {
      b = fgetc(p_fp);
      x = (x << 7) | (b & 0x7f);
    } while ((b & 0x80) == 0);
    return x;
  }

  std::variant<std::string, Error> ReadString(FILE* p_fp) {
    size_t length = ReadVarInt(p_fp) - 1;
    if (length == -1) {
      return std::string{}; // empty string
    }

    char* buffer = new (std::nothrow) char[length + 1];
    if (buffer == nullptr) {
      return "out of memory.";
    }
    buffer[length] = 0;

    if (fread(buffer, sizeof(byte), length, p_fp) != length) {
      return "can't read string.";
    } else {
      std::string str = std::string{buffer};
      delete[] buffer;
      return str;
    }
  }
} // namespace chunk
