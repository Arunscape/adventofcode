open Base
open Aoc_utils

let is_even n = n % 2 = 0

(* let is_invalid_id n =
  let s = Int.to_string n in
  let len = String.length s in
  is_even len && String.equal (String.sub s ~pos:0 ~len:(len / 2)) (String.sub s ~pos:(len / 2) ~len:(len / 2)) *)

let is_invalid_id n =
  let num_digits = Int.of_float (Float.log10 (Float.of_int n)) + 1 in
  if is_even num_digits then 
    let div = 10 ** (num_digits / 2) in
    n/div = n % div
else false

let invalid_ids_in_range min max = 
  List.range min (max + 1)
  |> List.filter ~f:is_invalid_id

let parse_input input =
  String.concat input ~sep:""
  |> String.filter ~f:(fun c -> not (Char.is_whitespace c))
  |> String.split ~on:','
  |> List.filter ~f:(fun s -> not (String.is_empty s))
  |> List.map ~f:(fun s ->
    match String.split s ~on:'-' with
    | [min_s; max_s] -> (Int.of_string min_s, Int.of_string max_s)
    | _ -> failwith ("Invalid range format: " ^ s)
  )

let solve_1 ranges =
  List.fold ranges ~init:0 ~f:(fun acc (min, max) ->
    let invalid_ids = invalid_ids_in_range min max in
    acc + (List.sum (module Int) invalid_ids ~f:Fn.id)
  )

let () =
  let input = read_input () in
  let ranges = parse_input input in
  let total_invalid_sum = solve_1 ranges in
  Stdio.printf "Part 1: %d\n" total_invalid_sum
