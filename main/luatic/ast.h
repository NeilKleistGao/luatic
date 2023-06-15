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
#include <utility>
#include <variant>
#include <vector>

#include "diagnostic.hpp"
#include "operators.hpp"

#define CASE(__NAME__) struct __NAME__: public ASTNode
#define INIT(__NAME__)                                                         \
explicit __NAME__(const Position& p_begin): ASTNode(p_begin)
#define ADT(__NAME__, __FIRST__, __REST__...)                                  \
struct __NAME__ {                                                              \
std::variant<__FIRST__, ##__REST__> value;                                     \
explicit __NAME__(std::variant<__FIRST__, ##__REST__> p):                      \
  value(std::move(p)) {}                                                       \
}

template<typename T>
using Ptr = std::shared_ptr<T>;

struct ASTNode {
  Location loc;

  ASTNode(const Position& p_begin):
    loc{
      Location{p_begin, p_begin}
  } {}
};

struct Block;
struct Expr;

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
CASE(UnaryExpr) {
  UnaryOperator op;
  Ptr<Expr> expr;
};
CASE(BinaryExpr) {
  BinaryOperator op;
  Ptr<Expr> lhs;
  Ptr<Expr> rhs;
};
CASE(TableCtorExpr) {
  using KV = std::pair<Ptr<Expr>, Ptr<Expr>>;
  std::vector<KV> elements;
};
CASE(FunctionExpr) {
  std::vector<Ptr<Expr>> params;
  bool is_var;
  Ptr<Block> body;
};
CASE(AccessExpr) {
  enum class AccessType { ACC_DOT, ACC_COL, ACC_IDX } type;
  Ptr<Expr> lhs;
  Ptr<Expr> rhs;
};
CASE(CallExpr) {
  Ptr<Expr> callee;
  std::vector<Ptr<Expr>> params;
};

ADT(Expr,
    NilExpr,
    BoolExpr,
    VarArgExpr,
    IntExpr,
    FloatExpr,
    StringExpr,
    VarExpr,
    UnaryExpr,
    BinaryExpr,
    TableCtorExpr,
    FunctionExpr,
    AccessExpr,
    CallExpr);

CASE(EmptyStmt) {
  INIT(EmptyStmt){};
};
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
CASE(CallStmt) {
  Ptr<CallExpr> call;
};
CASE(WhileStmt) {
  Expr cond;
  Ptr<Block> block;
};
CASE(RepeatStmt) {
  Expr cond;
  Ptr<Block> block;
};
CASE(IfStmt) {
  using CondBranch = std::pair<Expr, Ptr<Block>>;
  std::vector<CondBranch> branches;
  Ptr<Block> else_branch;
};
CASE(ForStmt) {
  using AssignPair = std::pair<std::string, Expr>;
  std::vector<AssignPair> assigns;
  Ptr<Block> block;
};
CASE(LocalVarDeclStmt) {
  using AssignPair = std::pair<std::string, Expr>;
  std::vector<AssignPair> assigns;
};
CASE(AssignStmt) {
  using AssignPair = std::pair<Expr, Expr>;
  std::vector<AssignPair> assigns;
};
CASE(FuncStmt) {
  std::string name;
  FunctionExpr func;
};

ADT(Stmt,
    EmptyStmt,
    BreakStmt,
    LabelStmt,
    GotoStmt,
    DoStmt,
    CallStmt,
    WhileStmt,
    RepeatStmt,
    IfStmt,
    ForStmt,
    LocalVarDeclStmt,
    AssignStmt,
    FuncStmt);

CASE(Block) {
  INIT(Block) {}
  std::vector<Stmt> stmts;
};

#undef ADT
#undef INIT
#undef CASE

#endif //LUATIC_AST_H
