open Base

let sum l =
  let rec inner t l =
    match l with [] -> t | head :: tail -> inner (t + head) tail
  in
  inner 0 l

let read_input () : string list = Stdio.In_channel.input_lines Stdio.In_channel.stdin
let is_digit c = match c with '0' .. '9' -> true | _ -> false

let first_digit s =
  match String.lfindi s ~f:(fun _ c -> is_digit c) with
  | None -> None
  | Some i -> Some (String.get s i)

let last_digit s =
  match String.rfindi s ~f:(fun _ c -> is_digit c) with
  | None -> None
  | Some i -> Some (String.get s i)
