(* op_code, pretty_printer, execution *)
type instruction =
  | InstABC of int * (unit -> string) * (unit -> string)
  | InstABx of int * (unit -> string) * (unit -> string)
  | InstAsBx of int * (unit -> string) * (unit -> string)
  | InstAx of int * (unit -> string) * (unit -> string)
  | InstsJ of int * (unit -> string) * (unit -> string);;

exception WrongField of string;;

let a ins = match ins with
  | InstABC (_, _, _) | InstABx (_, _, _) -> "((p_code >> 7) & 0xFF)"
  | _ -> raise (WrongField "no field a found.");;

let b ins = match ins with
  | InstABC(_, _, _) -> "((p_code >> 16) & 0xFF)"
  | _ -> raise (WrongField "no field b found.");;

let c ins = match ins with
  | InstABC(_, _, _) -> "((p_code >> 24) & 0xFF)"
  | _ -> raise (WrongField "no field c found.");;

let k ins = match ins with
  | InstABC(_, _, _) -> "((p_code >> 15) & 1)"
  | _ -> raise (WrongField "no field k found.");;

let bx ins = match ins with
  | InstABx(_, _, _) -> "((p_code >> 15) & 0x1FFFF)"
  | _ -> raise (WrongField "no field Bx found.");;

let sbx ins = match ins with
  | InstAsBx(_, _, _) -> "((p_code >> 15) & 0x1FFFF)"
  | _ -> raise (WrongField "no field sBx found.");;

let ax ins = match ins with
  | InstAx(_, _, _) -> "((p_code >> 7) & 0x1FFFFFF)"
  | _ -> raise (WrongField "no field Ax found.");;

let sj ins = match ins with
  | InstsJ(_, _, _) -> "((p_code >> 7) & 0x1FFFFFF)"
  | _ -> raise (WrongField "no field sJ found.");;
