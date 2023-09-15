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

#include "pretty_printer.h"

#include <iostream>

namespace std {
  string to_string(DiagnosticType p_type) {
    if (p_type == DiagnosticType::DIAG_LEX) {
      return string{"lex"};
    }
    if (p_type == DiagnosticType::DIAG_PARSE) {
      return string{"parser"};
    }
    return string{"codegen"};
  }
} // namespace std

static void PrintLocation(const Location& p_loc) {
  std::cout << "line " << p_loc.begin.line << ", " << p_loc.begin.column
            << " .. "
            << "line " << p_loc.end.line << ", " << p_loc.end.column;
}

void PrintDiagnostic(const Diagnostic& p_diag) {
  std::cout << "[" << std::to_string(p_diag.type) << " error]:" << std::endl;
  std::cout << "  " << p_diag.info << std::endl;

  const auto& loc = p_diag.location;
  if (p_diag.filename.has_value()) {
    std::cout << "  at " << p_diag.filename.value() << ", ";
  } else {
    std::cout << "  at ";
  }

  PrintLocation(loc);
  std::cout << std::endl;
}
