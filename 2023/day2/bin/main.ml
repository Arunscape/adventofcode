open Base

let read_input () : string list = Stdio.In_channel.input_lines In_channel.stdin

type bag = { red : int; green : int; blue : int }

let new_bag r g b = { red = r; green = g; blue = b }

type game = { id : int; bags : bag list }
(* why the fuck is dead code a compile error and not a warning *)
(*
let print_bag b =
  Stdio.print_endline
    ("Red: " ^ Int.to_string b.red ^ " Green: " ^ Int.to_string b.green
   ^ " Blue: " ^ Int.to_string b.blue)

let print_game g =
  Stdio.print_endline ("ID: " ^ Int.to_string g.id);
  List.iter g.bags ~f:print_bag
  *)

let process_round b rm =
  b.red - rm.red >= 0 && b.green - rm.green >= 0 && b.blue - rm.blue >= 0

let process_game g b = List.for_all g.bags ~f:(fun rm -> process_round b rm)

let bag_of_string s =
  let s = String.split s ~on:',' in
  let red, green, blue = (ref 0, ref 0, ref 0) in
  List.iter s ~f:(fun s ->
      if String.is_suffix s ~suffix:"red" then
        red := String.chop_suffix_exn s ~suffix:"red" |> Int.of_string
      else if String.is_suffix s ~suffix:"green" then
        green := String.chop_suffix_exn s ~suffix:"green" |> Int.of_string
      else if String.is_suffix s ~suffix:"blue" then
        blue := String.chop_suffix_exn s ~suffix:"blue" |> Int.of_string);
  new_bag !red !green !blue

let game_of_string s =
  let s = String.filter s ~f:(fun x -> not (Char.equal x ' ')) in
  let id, game = String.lsplit2_exn s ~on:':' in
  let id = String.chop_prefix_exn id ~prefix:"Game" |> Int.of_string in
  let remove_bags = String.split game ~on:';' |> List.map ~f:bag_of_string in
  { id; bags = remove_bags }

let () =
  let part_1_bag = new_bag 12 13 14 in

  let part_1 =
    read_input () |> List.map ~f:game_of_string
    (*  |> List.map ~f:(fun g -> print_game g; g) *)
    |> List.filter ~f:(fun g -> process_game g part_1_bag)
    (*  |> List.map ~f:(fun g -> print_game g; g) *)
    |> List.map ~f:(fun g -> g.id)
    (*  |> List.map ~f:(fun id -> Stdio.print_endline (Int.to_string id); id ) *)
    |> List.fold ~init:0 ~f:( + )
    |> Int.to_string
  in
  Stdio.print_endline ("Part 1: " ^ part_1)
