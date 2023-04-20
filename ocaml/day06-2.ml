exception Exception;;

let rec find_message_marker counter stream =
    let rec is_marker s =
        match s with
        | [] -> true
        | x :: xs ->
            if List.exists (fun y -> x = y) xs then
                false
            else
                is_marker xs
    in

    match stream with
    | [] -> raise Exception
    | x :: xs ->
        if List.init 14 (List.nth xs) |> is_marker then
            counter + 15
        else
            find_message_marker (counter + 1) xs
;;

let signal =
    let s = open_in "input.txt" |> input_line in
    List.init (String.length s) (String.get s);;
let init_message = find_message_marker 0 signal;;
string_of_int (init_message) |> print_endline;;
