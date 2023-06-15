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

#ifndef LUATIC_DIAGNOSTIC_HPP
#define LUATIC_DIAGNOSTIC_HPP

#include <optional>
#include <string>
#include <utility>

struct Position {
  int line{};
  int column{};

  Position(int p_l, int p_c): line(p_l), column(p_c) {}
};

struct Location {
  Position begin, end;

  Location(Position p_begin, Position p_end): begin(p_begin), end(p_end) {}

  static Position Begin() {
    return Position{1, 0};
  }
};

enum class DiagnosticType {
  DIAG_LEX,
  DIAG_PARSE,
  DIAG_TYPING,
  DIAG_CODEGEN
};

struct Diagnostic {
  DiagnosticType type;
  Location location;
  std::string info;
  std::optional<std::string> filename;

  Diagnostic(DiagnosticType p_t, Location p_loc, std::string p_info, std::optional<std::string> p_filename):
    type(p_t), location(p_loc), info(std::move(p_info)), filename(std::move(p_filename)) {}
};

inline Diagnostic RaiseErrorByType(DiagnosticType p_type, Location p_loc, std::string p_info, std::optional<std::string> p_filename) {
  return {p_type, p_loc, std::move(p_info), std::move(p_filename)};
}

#endif //LUATIC_DIAGNOSTIC_HPP
