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
    List.nth values 19 * 20;
    List.nth values 59 * 60;
    List.nth values 99 * 100;
    List.nth values 139 * 140;
    List.nth values 179 * 180;
    List.nth values 219 * 220
];;

let program = read_program ();;
let cycles = compute [1] program;;
let important = get_important cycles;;
List.fold_left (+) 0 important |> string_of_int |> print_endline;;
