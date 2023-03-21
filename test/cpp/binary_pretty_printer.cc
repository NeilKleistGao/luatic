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

#include "binary_pretty_printer.h"

#include "instructions_pretty_printer.h"

namespace chunk {
  void PrintPrototype(FILE* p_fp,
                      const Prototype& p_pro,
                      const std::string& p_indent) {
    fputs((p_indent + " PROTOTYPE\n").c_str(), p_fp);
    fprintf(p_fp, "%s source: %s\n", p_indent.c_str(), p_pro.source.c_str());
    fprintf(p_fp,
            "%s define: line %lu to line %lu\n",
            p_indent.c_str(),
            p_pro.line_defined,
            p_pro.last_line_defined);
    fprintf(p_fp, "%s num params: %d\n", p_indent.c_str(), p_pro.num_params);
    fprintf(p_fp,
            "%s is var arg: %s\n",
            p_indent.c_str(),
            (p_pro.is_vararg) ? "yes" : "no");
    fprintf(p_fp,
            "%s max stack size: %d\n",
            p_indent.c_str(),
            p_pro.max_stack_size);

    fprintf(p_fp, "%s code: \n", p_indent.c_str());
    for (const auto& code : p_pro.code) {
      instructions::PrintInstruction(p_fp, code, p_indent + "--");
    }

    fprintf(p_fp, "%s constant: \n", p_indent.c_str());
    for (const auto& lit : p_pro.constants) {
      switch (lit.index()) {
        case 0:
          fprintf(p_fp, "%s-- Nil\n", p_indent.c_str());
          break;
        case 1:
          fprintf(p_fp, "%s-- Boolean\n", p_indent.c_str());
          break;
        case 2:
          fprintf(p_fp, "%s-- Number\n", p_indent.c_str());
          break;
        case 3:
          fprintf(p_fp, "%s-- Int\n", p_indent.c_str());
          break;
        case 4:
          fprintf(p_fp, "%s-- String\n", p_indent.c_str());
          break;
      }
    }

    if (!p_pro.proto.empty()) {
      fprintf(p_fp, "%s sub prototypes: \n", p_indent.c_str());
      for (const auto& pro : p_pro.proto) {
        PrintPrototype(p_fp, pro, p_indent + "--");
      }
    }
  }

  void PrintChunk(FILE* p_fp, const BinaryChunk& p_chunk) {
    fputs("-- BINARY CHUNK\n", p_fp);
    fprintf(p_fp, "-- up values size: %d\n", p_chunk.size_up_values);
    PrintPrototype(p_fp, p_chunk.main_proto, "--");
  }
} // namespace chunk
