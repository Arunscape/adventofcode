open Base
open Aoc_utils

type direction = L of int | R of int

let ticks = 100

let rotate current_pos dir =
  match dir with
  | L n-> (current_pos - n) % ticks
  | R n -> (current_pos + n) % ticks

let parse_direction s =
  match String.chop_prefix s ~prefix:"R" with
  | Some amount_str -> R (Int.of_string amount_str)
  | None ->
    (match String.chop_prefix s ~prefix:"L" with
    | Some amount_str -> L (Int.of_string amount_str)
    | None -> failwith "Invalid direction")

(*the number of times the dial is pointing at 0 after any rotation in the sequence*)
let solve_1 input =
  let _, count =
    List.fold_left input ~init:(50, 0) ~f:(fun (pos, ans) dir ->
      let pos' = rotate pos dir in
      let ans' = if pos' = 0 then ans + 1 else ans in
      (pos', ans')
    )
  in
  count

let count_zeros start_pos dir =
  let floor_div a = Int.(round_down a ~to_multiple_of:ticks) / ticks in
  match dir with
  | R n -> floor_div (start_pos + n) - floor_div start_pos
  | L n -> floor_div (start_pos - 1) - floor_div (start_pos - n - 1)


let solve_2 input =
  let final_acc, _ =
    List.fold_left input ~init:(0, 50) ~f:(fun (acc, pos) dir ->
      let acc' = acc + count_zeros pos dir in
      let pos' = rotate pos dir in
      (acc', pos')
    )
  in
  final_acc

let () =
  let input = read_input () in
  let parsed = List.map input ~f:parse_direction in
  let part1 = solve_1 parsed in
  Stdio.printf "Part 1: %d\n" part1;

  let part2 = solve_2 parsed in
  Stdio.printf "Part 2: %d\n" part2
