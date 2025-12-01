open Base
open Aoc_utils

type direction = L of int | R of int

let ticks = 100

let rotate current_pos dir =
  match dir with
  | L n-> (current_pos - n) % ticks
  | R n -> (current_pos + n) % ticks


let parse_direction s =
  let dir_char = String.get s 0 in
  let amount_str = String.sub s ~pos:1 ~len:(String.length s - 1) in
  let amount = Int.of_string amount_str in
  match dir_char with
  | 'R' -> R amount
  | 'L' -> L amount
  | _ -> failwith "Invalid direction"

(*the number of times the dial is pointing at 0 after any rotation in the sequence*)
let solve_1 input = 
  let position = ref 50 in
  let ans = ref 0 in
  List.iter input ~f:(fun dir ->
    position := rotate !position dir;
    if !position = 0 then ans := !ans + 1
  );
  !ans

(*supposed to count the number of times any click causes the dial to point at 0, regardless of whether it happens during a rotation or at the end of one*)
let rec rotate_part_2 current_pos dir acc =
  match dir with
  | L n -> if n = 0 then acc else 
      let next_pos = (current_pos - 1) % ticks in
      let new_acc = if next_pos = 0 then acc + 1 else acc in
      rotate_part_2 next_pos (L (n - 1)) new_acc
  | R n -> if n = 0 then acc else 
      let next_pos = (current_pos + 1) % ticks in
      let new_acc = if next_pos = 0 then acc + 1 else acc in
      rotate_part_2 next_pos (R (n - 1)) new_acc

let solve_2 (input : direction list) =
  let (final_acc, _) = List.fold_left input ~init:(0, 50) ~f:(fun (acc, pos) dir ->
    let new_acc = rotate_part_2 pos dir acc in
    let new_pos = rotate pos dir in
    (new_acc, new_pos)
  ) in
  final_acc

let () =
  let input = read_input () in
  let parsed = List.map input ~f:parse_direction in
  let part1 = solve_1 parsed in
  Stdio.printf "Part 1: %d\n" part1;

  let part2 = solve_2 parsed in
  Stdio.printf "Part 2: %d\n" part2
