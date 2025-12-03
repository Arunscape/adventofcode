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

let prefixes =
  [
    ("0", 0);
    ("1", 1);
    ("2", 2);
    ("3", 3);
    ("4", 4);
    ("5", 5);
    ("6", 6);
    ("7", 7);
    ("8", 8);
    ("9", 9);
    ("one", 1);
    ("two", 2);
    ("three", 3);
    ("four", 4);
    ("five", 5);
    ("six", 6);
    ("seven", 7);
    ("eight", 8);
    ("nine", 9);
  ]

let rec first_digit_2 s =
  match s with
  | "" -> None
  | _ -> (
      match
        List.find prefixes ~f:(fun (prefix, _) -> String.is_prefix s ~prefix)
      with
      | None -> first_digit_2 (String.sub s ~pos:1 ~len:(String.length s - 1))
      | Some (_, n) -> Some n)

let rec last_digit_2 s =
  match s with
  | "" -> None
  | _ -> (
      match
        List.find prefixes ~f:(fun (prefix, _) ->
            String.is_suffix s ~suffix:prefix)
      with
      | None -> last_digit_2 (String.sub s ~pos:0 ~len:(String.length s - 1))
      | Some (_, n) -> Some n)

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
    |> List.map ~f:(fun s -> (first_digit_2 s, last_digit_2 s))
    |> List.filter_map ~f:(fun x ->
        match x with Some a, Some b -> Some ((a * 10) + b) | _ -> None)
    |> sum |> Int.to_string
  in

  Stdio.print_endline ("part2: " ^ part_two)
