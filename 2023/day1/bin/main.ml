open Base

let sum l =
  let rec inner t l =
    match l with [] -> t | head :: tail -> inner (t + head) tail
  in
  inner 0 l

let read_input () : string list = Stdio.In_channel.input_lines In_channel.stdin
let is_digit c = match c with '0' .. '9' -> true | _ -> false

let first_digit s =
  match String.lfindi s ~f:(fun _ c -> is_digit c) with
  | None -> None
  | Some i -> Some (String.get s i)

let last_digit s =
  match String.rfindi s ~f:(fun _ c -> is_digit c) with
  | None -> None
  | Some i -> Some (String.get s i)

let rec parse_digits s l =
  match s with
  | "" -> l
  | _ -> (
      let first_char = String.get s 0 in
      let len = String.length s in
      let rest_of_string = String.drop_prefix s 1 in
      match first_char with
      | '0' .. '9' ->
          parse_digits (String.drop_prefix s 1)
            (((first_char |> Char.to_int) - Char.to_int '0') :: l)
      | _ ->
          if len >= 3 then
            match String.sub s ~pos:0 ~len:3 with
            | "one" -> parse_digits (String.drop_prefix s 3) (1 :: l)
            | "two" -> parse_digits (String.drop_prefix s 3) (2 :: l)
            | "six" -> parse_digits (String.drop_prefix s 3) (6 :: l)
            | _ ->
                if len >= 4 then
                  match String.sub s ~pos:0 ~len:4 with
                  | "zero" -> parse_digits (String.drop_prefix s 4) (0 :: l)
                  | "four" -> parse_digits (String.drop_prefix s 4) (4 :: l)
                  | "five" -> parse_digits (String.drop_prefix s 4) (5 :: l)
                  | "nine" -> parse_digits (String.drop_prefix s 4) (9 :: l)
                  | _ ->
                      if len >= 5 then
                        match String.sub s ~pos:0 ~len:5 with
                        | "three" ->
                            parse_digits (String.drop_prefix s 5) (3 :: l)
                        | "seven" ->
                            parse_digits (String.drop_prefix s 5) (7 :: l)
                        | "eight" ->
                            parse_digits (String.drop_prefix s 5) (8 :: l)
                        | _ -> parse_digits rest_of_string l
                      else parse_digits rest_of_string l
                else parse_digits rest_of_string l
          else parse_digits rest_of_string l)

let () =
  let input = read_input () in
  let part_one =
    input
    |> List.map ~f:(fun s -> (first_digit s, last_digit s))
    |> List.map ~f:(fun x ->
           match x with
           | Some a, Some b -> Some (Char.to_string a ^ Char.to_string b)
           | _ -> None)
    |> List.filter_map ~f:(fun x -> x)
    |> List.map ~f:Int.of_string |> sum |> Int.to_string
  in

  Stdio.print_endline ("part1: " ^ part_one);

  let part_two =
    input
    |> List.map ~f:(fun s -> parse_digits s [])
    |> List.map ~f:(fun l -> List.hd_exn l + ((List.rev l |> List.hd_exn) * 10))
    |> sum |> Int.to_string
  in

  Stdio.print_endline ("part2: " ^ part_two)
