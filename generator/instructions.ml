(* pretty_printer, execution *)
type instruction =
  | InstABC of (instruction -> string list) * (instruction -> string list)
  | InstABx of (instruction -> string list) * (instruction -> string list)
  | InstAsBx of (instruction -> string list) * (instruction -> string list)
  | InstAx of (instruction -> string list) * (instruction -> string list)
  | InstsJ of (instruction -> string list) * (instruction -> string list);;

exception WrongField of string;;

let op_code = "(p_code & 0x7F)";;

let a ins = match ins with
  | InstABC (_, _) | InstABx (_, _) -> "((p_code >> 7) & 0xFF)"
  | _ -> raise (WrongField "no field a found.");;

let b ins = match ins with
  | InstABC(_, _) -> "((p_code >> 16) & 0xFF)"
  | _ -> raise (WrongField "no field b found.");;

let c ins = match ins with
  | InstABC(_, _) -> "((p_code >> 24) & 0xFF)"
  | _ -> raise (WrongField "no field c found.");;

let k ins = match ins with
  | InstABC(_, _) -> "((p_code >> 15) & 1)"
  | _ -> raise (WrongField "no field k found.");;

let bx ins = match ins with
  | InstABx(_, _) -> "((p_code >> 15) & 0x1FFFF)"
  | _ -> raise (WrongField "no field Bx found.");;

let sbx ins = match ins with
  | InstAsBx(_, _) -> "((p_code >> 15) & 0x1FFFF)"
  | _ -> raise (WrongField "no field sBx found.");;

let ax ins = match ins with
  | InstAx(_, _) -> "((p_code >> 7) & 0x1FFFFFF)"
  | _ -> raise (WrongField "no field Ax found.");;

let sj ins = match ins with
  | InstsJ(_, _) -> "((p_code >> 7) & 0x1FFFFFF)"
  | _ -> raise (WrongField "no field sJ found.");;

let fprintf params =
  match params with
    | [] -> ""
    | format :: values ->
      (List.fold_left (fun r s -> r ^ "," ^ s) ("fprintf(p_fp, \"%s" ^ format ^ "\\n\", p_indent.c_str()") values) ^
      ");";;

let rec pretty_print_rec i lst res = 
  match lst with
    | [] -> res
    | head :: tail ->
      pretty_print_rec (i + 1) tail (res ^ "case " ^ (string_of_int i) ^ ": " ^ (fprintf head) ^ "break;\n");;

let pretty_print lst =
  "#include \"instructions_pretty_printer.h\"\n" ^
  "\n" ^
  "namespace instructions {\n" ^
  "void PrintInstruction(FILE* p_fp,\n" ^
  "Instruction p_ins," ^
  "const std::string& p_indent) {\n" ^
  "switch" ^ op_code ^ "{\n" ^
  (pretty_print_rec 0 lst "") ^
  "default: fprintf(p_fp, \"%sInvalid Op Code %d\n\", p_indent.c_str(), p_ins);\n" ^
  "}\n" ^
  "}\n" ^
  "}\n";;

(* put instructions in order *)
let instructions = [
  InstABC ( (* 0 *)
    (fun ins -> ["Move"]),
    (fun ins -> ["return 1;"])
  );
  InstAsBx (
    (fun ins -> ["Load I"]),
    (fun ins -> ["return 1;"])
  );
  InstAsBx (
    (fun ins -> ["Load F"]),
    (fun ins -> ["return 1;"])
  );
  InstABx (
    (fun ins -> ["Load K"]),
    (fun ins -> ["return 1;"])
  );
  InstABx (
    (fun ins -> ["Load KX"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 5 *)
    (fun ins -> ["Load False"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["L False Skip"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Load True"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Load Nil"]),
    (fun ins -> ["return 1;"])
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
    (fun ins -> ["Add"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 35 *)
    (fun ins -> ["Sub"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Mul"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Mod"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Pow"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Div"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 40 *)
    (fun ins -> ["I Div"]),
    (fun ins -> ["return 1;"])
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
    (fun ins -> ["UNM"]),
    (fun ins -> ["return 1;"])
  );
  InstABC ( (* 50 *)
    (fun ins -> ["Bit Not"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Not"]),
    (fun ins -> ["return 1;"])
  );
  InstABC (
    (fun ins -> ["Len"]),
    (fun ins -> ["return 1;"])
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
    (fun ins -> ["Jump"]),
    (fun ins -> ["return 1;"])
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
    (fun ins -> ["Return %d, %d"; (a ins); (b ins)]),
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

print_endline (pretty_print
  (List.map (fun ins -> match ins with
    | InstABC(pp, _) as ins -> pp ins
    | InstABx (pp, _) as ins -> pp ins
    | InstAsBx(pp, _) as ins -> pp ins
    | InstAx(pp, _) as ins -> pp ins
    | InstsJ(pp, _) as ins -> pp ins
  ) instructions));;
