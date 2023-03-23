(*
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
 *)

open Cpplib

(* put instructions in order *)
(* TODO: add more check for C++ AST *)
let instructions = [
  InstABC ( (* 0 *)
    (fun ins -> ["Move"]),
    (fun ins -> [
      "const auto value = p_stack->Get(" ^ a(ins) ^ ");";
      "const auto pos = " ^ b(ins) ^ ";";
      "if (value.has_value()) { p_stack->Set(pos, value.value()); }"; (* TODO: throw *)
      "return 1;"
    ])
  );
  InstAsBx (
    (fun ins -> ["Load I at %d, value: %d"; a(ins); sbx(ins)]),
    (fun ins -> [
      "p_stack->Set(" ^ a(ins) ^ ", " ^ sbx(ins) ^ ");";
      "return 1;"
    ])
  );
  InstAsBx (
    (fun ins -> ["Load F at %d, value: %f"; a(ins); "static_cast<double>(" ^ sbx(ins) ^ ")"]),
    (fun ins -> [
      "p_stack->Set(" ^ a(ins) ^ ", " ^ "static_cast<double>(" ^ sbx(ins) ^ "));";
      "return 1;"
    ])
  );
  InstABx (
    (fun ins -> ["Load K(constant) from %d to %d"; bx(ins); a(ins)]),
    (fun ins -> [
      "p_stack->Push(FromLiteral(p_const[" ^ bx(ins) ^ "]));";
      "p_stack->ReplaceWithTop(" ^ a(ins) ^ ");";
      "return 1;"
    ])
  );
  InstABx (
    (fun ins -> ["Load KX"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 5 *)
    (fun ins -> ["Load False at %d"; a(ins)]),
    (fun ins -> [
      "p_stack->Set(" ^ a(ins) ^ ", false);";
      "return 1;"
    ])
  );
  InstABC (
    (fun ins -> ["L False Skip"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Load True at %d"; a(ins)]),
    (fun ins -> [
      "p_stack->Set(" ^ a(ins) ^ ", true);";
      "return 1;"
    ])
  );
  InstABC (
    (fun ins -> ["Load Nil: [%d, %d]"; a(ins); a(ins) ^ " + " ^ b(ins)]),
    (fun ins -> [
      "p_stack->Push(nullptr);";
      "for (int i = 0; i <= " ^ b(ins) ^ "; ++i) p_stack->Copy(-1, i);";
      "p_stack->Pop();";
      "return 1;"
    ])
  );
  InstABC (
    (fun ins -> ["Get Up Value"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 10 *)
    (fun ins -> ["Set Up Value"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Get Table Up"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Get Table"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Get I"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Get Field"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 15 *)
    (fun ins -> ["Set Table Up"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Set Table"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Set I"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Set Field"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["New Table"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 20 *)
    (fun ins -> ["Self"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Add I"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Add K"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Sub K"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Mul K"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 25 *)
    (fun ins -> ["Mod K"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Pow K"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Div K"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["I Div K"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Bit And K"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 30 *)
    (fun ins -> ["Bit Or K"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Bit Xor K"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Shift Right I"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Shift Left I"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Add R[%d] = R[%d] + R[%d]"; a(ins); b(ins); c(ins)]),
    (fun ins -> [
      "const auto p1 = p_stack->Get(" ^ b(ins) ^ ");";
      "const auto p2 = p_stack->Get(" ^ c(ins) ^ ");";
      "const auto res = CalcArith(math::ArithOperator::ADD, p1.value(), p2.value());";
      "p_stack->Push(res);";
      "p_stack->ReplaceWithTop(" ^ a(ins) ^ ");";
      "return 1;"
    ])
  );
  InstABC ( (* 35 *)
    (fun ins -> ["Sub R[%d] = R[%d] - R[%d]"; a(ins); b(ins); c(ins)]),
    (fun ins -> [
      "const auto p1 = p_stack->Get(" ^ b(ins) ^ ");";
      "const auto p2 = p_stack->Get(" ^ c(ins) ^ ");";
      "const auto res = CalcArith(math::ArithOperator::SUB, p1.value(), p2.value());";
      "p_stack->Push(res);";
      "p_stack->ReplaceWithTop(" ^ a(ins) ^ ");";
      "return 1;"
    ])
  );
  InstABC (
    (fun ins -> ["Mul R[%d] = R[%d] * R[%d]"; a(ins); b(ins); c(ins)]),
    (fun ins -> [
      "const auto p1 = p_stack->Get(" ^ b(ins) ^ ");";
      "const auto p2 = p_stack->Get(" ^ c(ins) ^ ");";
      "const auto res = CalcArith(math::ArithOperator::MUL, p1.value(), p2.value());";
      "p_stack->Push(res);";
      "p_stack->ReplaceWithTop(" ^ a(ins) ^ ");";
      "return 1;"
    ])
  );
  InstABC (
    (fun ins -> ["Mod R[%d] = R[%d] %% R[%d]"; a(ins); b(ins); c(ins)]),
    (fun ins -> [
      "const auto p1 = p_stack->Get(" ^ b(ins) ^ ");";
      "const auto p2 = p_stack->Get(" ^ c(ins) ^ ");";
      "const auto res = Mod(p1.value(), p2.value());";
      "p_stack->Push(res);";
      "p_stack->ReplaceWithTop(" ^ a(ins) ^ ");";
      "return 1;"
    ])
  );
  InstABC (
    (fun ins -> ["Pow R[%d] = R[%d] ^ R[%d]"; a(ins); b(ins); c(ins)]),
    (fun ins -> [
      "const auto p1 = p_stack->Get(" ^ b(ins) ^ ");";
      "const auto p2 = p_stack->Get(" ^ c(ins) ^ ");";
      "const auto res = CalcArith(math::ArithOperator::POW, p1.value(), p2.value());";
      "p_stack->Push(res);";
      "p_stack->ReplaceWithTop(" ^ a(ins) ^ ");";
      "return 1;"
    ])
  );
  InstABC (
    (fun ins -> ["Div R[%d] = R[%d] / R[%d]"; a(ins); b(ins); c(ins)]),
    (fun ins -> [
      "const auto p1 = p_stack->Get(" ^ b(ins) ^ ");";
      "const auto p2 = p_stack->Get(" ^ c(ins) ^ ");";
      "const auto res = CalcArith(math::ArithOperator::DIV, p1.value(), p2.value());";
      "p_stack->Push(res);";
      "p_stack->ReplaceWithTop(" ^ a(ins) ^ ");";
      "return 1;"
    ])
  );
  InstABC ( (* 40 *)
    (fun ins -> ["I Div R[%d] = R[%d] // R[%d]"; a(ins); b(ins); c(ins)]),
    (fun ins -> [
      "const auto p1 = p_stack->Get(" ^ b(ins) ^ ");";
      "const auto p2 = p_stack->Get(" ^ c(ins) ^ ");";
      "const auto res = CalcArith(math::ArithOperator::I_DIV, p1.value(), p2.value());";
      "p_stack->Push(res);";
      "p_stack->ReplaceWithTop(" ^ a(ins) ^ ");";
      "return 1;"
    ])
  );
  InstABC (
    (fun ins -> ["Bit And"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Bit Or"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Bit Xor"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Shift Left"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 45 *)
    (fun ins -> ["Shift Right"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["MM Bin"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["MM Bin I"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["MM Bin K"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["UNM R[%d] = -R[%d]"; a(ins); b(ins)]),
    (fun ins -> [
      "const auto p = p_stack->Get(" ^ b(ins) ^ ");";
      "const auto res = Neg(p.value());";
      "p_stack->Push(res);";
      "p_stack->ReplaceWithTop(" ^ a(ins) ^ ");";
      "return 1;"
    ])
  );
  InstABC ( (* 50 *)
    (fun ins -> ["Bit Not"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Not R[%d] = not R[%d]"; a(ins); b(ins)]),
    (fun ins -> [
      "const auto p = p_stack->Get(" ^ b(ins) ^ ");";
      "const auto res = Not(p.value());";
      "p_stack->Push(res);";
      "p_stack->ReplaceWithTop(" ^ a(ins) ^ ");";
      "return 1;"
    ])
  );
  InstABC (
    (fun ins -> ["Len R[%d] = len(R[%d])"; a(ins); b(ins)]),
    (fun ins -> [
      "const auto p = p_stack->Get(" ^ b(ins) ^ ");";
      "const auto res = Len(p.value());";
      "p_stack->Push(res);";
      "p_stack->ReplaceWithTop(" ^ a(ins) ^ ");";
      "return 1;"
    ])
  );
  InstABC (
    (fun ins -> ["Concat"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Close"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 55 *)
    (fun ins -> ["TBC"]),
    (fun ins -> ["return 1;"])
  );
  InstsJ (
    (fun ins -> ["Jump: %d"; sj(ins)]),
    (fun ins -> ["return " ^ sj(ins) ^ " + 1;"])
  );
  InstABC (
    (fun ins -> ["Equal"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Less Than"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Less Equal"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 60 *)
    (fun ins -> ["Equal K"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Equal I"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Less Than I"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Less Equal I"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Greater Than I"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 65 *)
    (fun ins -> ["Greater Equal I"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Test"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Test Set"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Call"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Tail Call"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 70 *)
    (fun ins -> ["Return: %d, %d"; (a ins); (b ins)]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Return 0"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Return 1"]),
    (fun ins -> ["return 1;"])
  );
  InstABx (
    (fun ins -> ["For Loop"]),
    (fun ins -> ["return 1;"])
  );
  InstABx (
    (fun ins -> ["For Prepare"]),
    (fun ins -> ["return 1;"])
  );
  InstABx ( (* 75 *)
    (fun ins -> ["T For Prepare"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["T For Call"]),
    (fun ins -> ["return 1;"])
  );
  InstABx (
    (fun ins -> ["T For Loop"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Set List"]),
    (fun ins -> ["return 1;"])
  );
  InstABx (
    (fun ins -> ["Closure"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 80 *)
    (fun ins -> ["Variable Arguments"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Variable Arguments Prepare"]),
    (fun ins -> ["return 1;"])
  );
  InstAx (
    (fun ins -> ["Extra Arguments"]),
    (fun ins -> ["return 1;"])
  );
];;

let pretty_print_cpp = (pretty_print
  (List.map (fun ins -> match ins with
    | InstABC(pp, _) as ins -> pp ins
    | InstABx (pp, _) as ins -> pp ins
    | InstAsBx(pp, _) as ins -> pp ins
    | InstAx(pp, _) as ins -> pp ins
    | InstsJ(pp, _) as ins -> pp ins
  ) instructions));;

let impl_cpp = (execute
(List.map (fun ins -> match ins with
  | InstABC(_, stmts) as ins -> stmts ins
  | InstABx (_, stmts) as ins -> stmts ins
  | InstAsBx(_, stmts) as ins -> stmts ins
  | InstAx(_, stmts) as ins -> stmts ins
  | InstsJ(_, stmts) as ins -> stmts ins
) instructions));;

let filename = "test/cpp/instructions_pretty_printer.cc" in
  let oc = open_out filename in
    Printf.fprintf oc "%s\n" pretty_print_cpp;
    close_out oc;;

let filename = "main/luna/instructions/instructions_impl.cc" in
  let oc = open_out filename in
    Printf.fprintf oc "%s\n" impl_cpp;
    close_out oc;;
