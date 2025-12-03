open Base
open Aoc_utils

(* let is_invalid_id n =
  let s = Int.to_string n in
  let len = String.length s in
  is_even len && String.equal (String.sub s ~pos:0 ~len:(len / 2)) (String.sub s ~pos:(len / 2) ~len:(len / 2)) *)

let repeated_twice n =
  let num_digits = num_digits n in
  if num_digits |> is_even then
    let div = 10 ** (num_digits / 2) in
    n / div = n % div
  else false

let invalid_ids_in_range min max f = range_inclusive min max |> List.filter ~f

let parse_input input =
  String.concat input ~sep:""
  |> String.filter ~f:(fun c -> not (Char.is_whitespace c))
  |> String.split ~on:','
  |> List.filter ~f:(fun s -> not (String.is_empty s))
  |> List.map ~f:(fun s ->
      match String.split s ~on:'-' with
      | [ min_s; max_s ] -> (Int.of_string min_s, Int.of_string max_s)
      | _ -> failwith ("Invalid range format: " ^ s))

let solve ranges f =
  List.fold ranges ~init:0 ~f:(fun acc (min, max) ->
      let invalid_ids = invalid_ids_in_range min max f in
      acc + List.sum (module Int) invalid_ids ~f:Fn.id)

let get_divisors n =
  range_inclusive 1 (n / 2) |> List.filter ~f:(fun i -> n |> is_multiple_of i)

let repeated_at_least_once n =
  let num_digits = num_digits n in
  let divisors = get_divisors num_digits in
  List.exists divisors ~f:(fun len_of_repeat ->
      let n_repeats = num_digits / len_of_repeat in
      let multiplier =
        List.range 0 n_repeats
        |> List.fold ~init:0 ~f:(fun acc i ->
            acc + Int.pow 10 (i * len_of_repeat))
      in
      n |> is_multiple_of multiplier)

let solve_1 ranges = solve ranges repeated_twice
let solve_2 ranges = solve ranges repeated_at_least_once

let () =
  let input = read_input () in
  let ranges = parse_input input in
  Stdio.printf "Part 1: %d\n" (solve_1 ranges);
  Stdio.printf "Part 2: %d\n" (solve_2 ranges)
