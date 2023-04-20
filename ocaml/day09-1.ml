exception Exception

type direction =
    | Up of int
    | Down of int
    | Left of int
    | Right of int
;;

let next_tail_move h t =
    let side n = if n > 0 then 1 else if n < 0 then -1 else 0 in

    let (hx, hy) = h in
    let (tx, ty) = t in

    if abs(hx - tx) <= 1 && abs(hy - ty) <= 1 then
        t
    else
        (hx - tx |> side |> (+) tx, hy - ty |> side |> (+) ty)
;;

let rec loop_tail_move acc h t =
    let new_t = next_tail_move h t in
    if new_t = t then
        acc
    else
        loop_tail_move (new_t :: acc) h new_t
;;

let move_head (x, y) dire =
    match dire with
    | Up s -> (x, y + s)
    | Down s -> (x, y - s)
    | Left s -> (x - s, y)
    | Right s -> (x + s, y)
;;

let parse_order s =
    let [@warning "-8"] [dd; ss] = String.split_on_char ' ' s in
    match dd with
    | "U" -> Up (int_of_string ss)
    | "D" -> Down (int_of_string ss)
    | "L" -> Left (int_of_string ss)
    | "R" -> Right (int_of_string ss)
    | _ -> raise Exception
;;

let parse_movement () =
    let ii = open_in "input.txt" in
    let rec loop acc h t =
        match input_line ii with
        | exception End_of_file -> acc
        | line ->
            let order = parse_order line in
            let new_h = move_head h order in
            let t_moves = loop_tail_move [] new_h t in
            let new_acc = List.append t_moves acc in
            let new_t =
                match List.nth_opt t_moves 0 with
                | None -> t
                | Some s -> s
            in
            loop new_acc new_h new_t
    in
    loop [(0, 0)] (0, 0) (0, 0)
;;

let make_unique ss = List.fold_left (fun acc x -> if List.exists (fun y -> x = y) acc then acc else x :: acc) [] ss;;

let places = parse_movement () |> make_unique;;
let ris = List.length places;;
string_of_int ris |> print_endline;;
