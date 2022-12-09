exception Exception

type direction =
    | Up of int
    | Down of int
    | Left of int
    | Right of int
;;

let next_tail_move h t =
    let (hx, hy) = h in
    let (tx, ty) = t in

    if hy - ty > 1 && hx - tx = 0 then
        (tx, ty + 1)
    else if (hx - tx > 1 && hy - ty > 0) || (hy - ty > 1 && hx - tx > 0) then
        (tx + 1, ty + 1)
    else if hx - tx > 1 && hy - ty = 0 then
        (tx + 1, ty)
    else if (hx - tx > 1 && hy - ty < 0) || (ty - hy > 1 && hx - tx > 0) then
        (tx + 1, ty - 1)
    else if ty - hy > 1 && tx - hx = 0 then
        (tx, ty - 1)
    else if (ty - hy > 1 && hx - tx < 0) || (tx - hx > 1 && hy - ty < 0) then
        (tx - 1, ty - 1)
    else if tx - hx > 1 && ty - hy = 0 then
        (tx - 1, ty)
    else if (hy - ty > 1 && hx - tx < 0) || (tx - hx > 1 && hy - ty > 0) then
        (tx - 1, ty + 1)
    else
        t
;;

let rec loop_knots acc rope =
    match rope with
    | [] -> List.rev acc
    | [s] -> List.rev (s :: acc)
    | h :: t :: xs ->
        let new_tail = next_tail_move h t in
        loop_knots (h :: acc) (new_tail :: xs)
;;

let rec loop_tail_move acc rope =
    let new_rope = loop_knots [] rope in
    if new_rope = rope then
        List.rev acc, new_rope
    else
        loop_tail_move (List.nth new_rope 9 :: acc) new_rope
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

    let rec loop acc rope =
        match input_line ii with
        | exception End_of_file -> acc
        | line ->
            let order = parse_order line in
            let new_h = move_head (List.nth rope 0) order in
            let [@warning "-8"] _ :: temp = rope in
            let to_co = new_h :: temp in
            let t_moves, new_rope = loop_tail_move [] to_co in
            let new_acc = List.append t_moves acc in
            loop new_acc new_rope
    in
    loop [(0, 0)] [(0, 0); (0, 0); (0, 0); (0, 0); (0, 0); (0, 0); (0, 0); (0, 0); (0, 0); (0, 0)]
;;

let make_unique ss = List.fold_left (fun acc x -> if List.exists (fun y -> x = y) acc then acc else x :: acc) [] ss;;

let places = parse_movement () |> make_unique;;
let ris = List.length places;;
string_of_int ris |> print_endline;;
