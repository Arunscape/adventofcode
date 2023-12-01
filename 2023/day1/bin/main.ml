let rec sum l = match l with
  | [] -> 0
  | head :: tail -> head + (sum tail)
;;

let char2int c = int_of_char c - int_of_char '0';;
let str2charlist s = s |> String.to_seq |> List.of_seq;; (* what I don't get this *)

let rec first_digit s: int option = 
  match s with
  | [] -> None
  | head :: tail -> match head with
    | '0' .. '9' -> Some (char2int head)
    | _ -> first_digit tail
;;

let rec read_input l =
  match read_line () with
  | exception End_of_file -> List.rev l
  | line -> read_input (line :: l)
;;

let () = 
  let input = read_input [] in
  let output = input |> List.map str2charlist |> List.map first_digit in
  output |> List.iter (fun x -> match x with | Some x -> print_int x | None -> ())
;;


(*
let rec first_digit (s: char Seq.t): int option =
  match s () with
  | Seq.Nil -> None
  | Seq.Cons (head, tail) -> match head with
    | '0' .. '9' -> Some (char2int head)
    | _ -> first_digit tail
;;


let () = 
  let input = read_input () in
  let o = input.map first_digit in
  o.iter (fun x -> match x with | Some x -> print_int x | None -> ())
;;
*)
