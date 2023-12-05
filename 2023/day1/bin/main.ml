open Base


let sum l = 
  let rec inner t l = match l with
  | [] -> t
  | head :: tail -> inner (t + head) tail
  in inner 0 l
;;


let read_input () : string list =
  Stdio.In_channel.input_lines In_channel.stdin


let is_digit c = match c with
  | '0' .. '9' -> true
  | _ -> false
;;

let first_digit s = match String.lfindi s ~f:(fun _ c -> is_digit c) with 
  | None -> None
  | Some (i) -> Some(String.get s i)
;;

let last_digit s = match String.rfindi s ~f:(fun _ c -> is_digit c) with
  | None -> None
  | Some (i) -> Some(String.get s i)
;;


let () = 
  let input = read_input () in
  let part_one = input
  |> List.map ~f:(fun s -> (first_digit s, last_digit s)) 
  |> List.map ~f:(fun x -> match x with
    | (Some a, Some b) -> Char.to_string a ^ Char.to_string b
    | _ -> "None") 
  |> List.map ~f:Int.of_string
  |> sum
  |> Int.to_string in

  Stdio.print_endline part_one;
;;

  
(*
let char2int c = int_of_char c - int_of_char '0';;
*)

(* gonna use iter and containers instead
let str2charlist s = s |> String.to_seq |> List.of_seq;;
*)

(*
let rec first_digit (s:char list): char option = 
  match s with
  | [] -> None
  | head :: tail -> match head with
    | '0' .. '9' -> Some (head)
    | _ -> first_digit tail
;;

let last_digit s =
  first_digit (List.rev s)
;;
*)

(*
let read_input () =
  let rec inner l = 
  match Stdio.In_channel.input_line with
  | exception End_of_file -> List.rev l
  | line -> inner (line :: l)
  in inner [] 
;;
*)
