(* pretty_printer, execution *)
type instruction =
  | InstABC of (unit -> string list) * (unit -> string list)
  | InstABx of (unit -> string list) * (unit -> string list)
  | InstAsBx of (unit -> string list) * (unit -> string list)
  | InstAx of (unit -> string list) * (unit -> string list)
  | InstsJ of (unit -> string list) * (unit -> string list);;

exception WrongField of string;;

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

(* put instructions in order *)
let instructions = [
  InstABC ( (* 0 *)
    (fun _ -> ["Move"]),
    (fun _ -> ["return 1;"])
  );
  InstAsBx (
    (fun _ -> ["Load I"]),
    (fun _ -> ["return 1;"])
  );
  InstAsBx (
    (fun _ -> ["Load F"]),
    (fun _ -> ["return 1;"])
  );
  InstABx (
    (fun _ -> ["Load K"]),
    (fun _ -> ["return 1;"])
  );
  InstABx (
    (fun _ -> ["Load KX"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 5 *)
    (fun _ -> ["Load False"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["L False Skip"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Load True"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Load Nil"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Get Up Value"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 10 *)
    (fun _ -> ["Set Up Value"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Get Table Up"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Get Table"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Get I"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Get Field"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 15 *)
    (fun _ -> ["Set Table Up"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Set Table"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Set I"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Set Field"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["New Table"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 20 *)
    (fun _ -> ["Self"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Add I"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Add K"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Sub K"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Mul K"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 25 *)
    (fun _ -> ["Mod K"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Pow K"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Div K"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["I Div K"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Bit And K"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 30 *)
    (fun _ -> ["Bit Or K"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Bit Xor K"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Shift Right I"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Shift Left I"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Add"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 35 *)
    (fun _ -> ["Sub"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Mul"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Mod"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Pow"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Div"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 40 *)
    (fun _ -> ["I Div"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Bit And"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Bit Or"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Bit Xor"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Shift Left"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 45 *)
    (fun _ -> ["Shift Right"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["MM Bin"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["MM Bin I"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["MM Bin K"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["UNM"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 50 *)
    (fun _ -> ["Bit Not"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Not"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Len"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Concat"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Close"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 55 *)
    (fun _ -> ["TBC"]),
    (fun _ -> ["return 1;"])
  );
  InstsJ (
    (fun _ -> ["Jump"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Equal"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Less Than"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Less Equal"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 60 *)
    (fun _ -> ["Equal K"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Equal I"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Less Than I"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Less Equal I"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Greater Than I"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 65 *)
    (fun _ -> ["Greater Equal I"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Test"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Test Set"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Call"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Tail Call"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 70 *)
    (fun _ -> ["Return"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Return 0"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Return 1"]),
    (fun _ -> ["return 1;"])
  );
  InstABx (
    (fun _ -> ["For Loop"]),
    (fun _ -> ["return 1;"])
  );
  InstABx (
    (fun _ -> ["For Prepare"]),
    (fun _ -> ["return 1;"])
  );
  InstABx ( (* 75 *)
    (fun _ -> ["T For Prepare"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["T For Call"]),
    (fun _ -> ["return 1;"])
  );
  InstABx (
    (fun _ -> ["T For Loop"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Set List"]),
    (fun _ -> ["return 1;"])
  );
  InstABx (
    (fun _ -> ["Closure"]),
    (fun _ -> ["return 1;"])
  );
  InstABC ( (* 80 *)
    (fun _ -> ["Variable Arguments"]),
    (fun _ -> ["return 1;"])
  );
  InstABC (
    (fun _ -> ["Variable Arguments Prepare"]),
    (fun _ -> ["return 1;"])
  );
  InstAx (
    (fun _ -> ["Extra Arguments"]),
    (fun _ -> ["return 1;"])
  );
];;
