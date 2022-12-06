exception Exception;;

let rec find_marker counter stream =
    match stream with
    | a :: b :: c :: d :: rest ->
        begin
        if a != b && a != c && a != d && b != c && b != d && c != d then
            counter + 4
        else
            find_marker (counter + 1) (b :: c :: d :: rest)
        end
    | _ -> raise Exception
;;

let signal =
    let s = open_in "input.txt" |> input_line in
    List.init (String.length s) (String.get s);;
let result = find_marker 0 signal;;
string_of_int result |> print_endline;;
