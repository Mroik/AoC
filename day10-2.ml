exception Exception

type instructions =
    | Noop
    | Add of int
;;

let rec compute values program =
    let [@warning "-8"] last :: _ = values in
    match program with
    | [] -> List.rev values
    | Noop :: xs -> compute (last :: values) xs
    | Add v :: xs -> compute (last + v :: last :: values) xs
;;

let read_program () =
    let ii = open_in "input.txt" in
    let rec loop acc =
        match input_line ii with
        | exception End_of_file -> List.rev acc
        | line ->
            match String.sub line 0 4 with
            | "noop" -> loop (Noop :: acc)
            | "addx" ->
                let [@warning "-8"] [_; v] = String.split_on_char ' ' line in
                loop (Add (int_of_string v) :: acc)
            | _ -> raise Exception
    in
    loop []
;;

let get_important values = [
    List.init 40 (fun x -> x |> List.nth values);
    List.init 40 (fun x -> x + 40 |> List.nth values);
    List.init 40 (fun x -> x + 80 |> List.nth values);
    List.init 40 (fun x -> x + 120 |> List.nth values);
    List.init 40 (fun x -> x + 160 |> List.nth values);
    List.init 40 (fun x -> x + 200 |> List.nth values)
];;

let rec draw values =
    match values with
    | [] -> ()
    | x :: xs ->
        let () = draw_row 0 x in
        let () = print_endline "" in
        draw xs
and draw_row counter values =
    match values with
    | [] -> ()
    | x :: xs ->
        if counter - 1 <= x && x <= counter + 1 then
            let () = print_string "#" in
            draw_row (counter + 1) xs
        else
            let () = print_string "." in
            draw_row (counter + 1) xs
;;

let program = read_program ();;
let cycles = compute [1] program;;
let sprite_positions = get_important cycles;;
draw sprite_positions;;
