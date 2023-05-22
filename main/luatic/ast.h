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

#ifndef LUATIC_AST_H
#define LUATIC_AST_H

#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "diagnostic.hpp"

#define CASE(__NAME__) struct __NAME__: public ASTNode
#define ADT(__NAME__, __FIRST__, __REST__...)                                  \
using __NAME__ = std::variant<__FIRST__, ##__REST__>

template<typename T>
using Ptr = std::shared_ptr<T>;

struct ASTNode {
  Location loc;
};

struct Block;

CASE(NilExpr){};
CASE(BoolExpr) {
  bool value = false;
};
CASE(VarArgExpr){};
CASE(IntExpr) {
  long long value = 0L;
};
CASE(FloatExpr) {
  double value = 0.0;
};
CASE(StringExpr) {
  std::string value;
};
CASE(VarExpr) {
  std::string name;
};

ADT(Expr,
    NilExpr,
    BoolExpr,
    VarArgExpr,
    IntExpr,
    FloatExpr,
    StringExpr,
    VarExpr);

CASE(EmptyStmt){};
CASE(BreakStmt){};
CASE(LabelStmt) {
  std::string name;
};
CASE(GotoStmt) {
  std::string name;
};
CASE(DoStmt) {
  Ptr<Block> block;
};

// TODO: more statements

ADT(Stmt, EmptyStmt, BreakStmt, LabelStmt, GotoStmt, DoStmt);

CASE(Block) {
  std::vector<Stmt> stmts;
};

#undef ADT
#undef CASE

#endif //LUATIC_AST_H