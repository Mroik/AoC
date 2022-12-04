let get_pairs () =
    let ii = open_in "input.txt" in
    let rec loop acc =
        match input_line ii with
        | exception End_of_file -> acc
        | ss ->
            let [@warning "-8"] [a; b] = String.split_on_char ',' ss in
            let [@warning "-8"] [a1; a2] = String.split_on_char '-' a in
            let [@warning "-8"] [b1; b2] = String.split_on_char '-' b in
            let aa = (int_of_string a1, int_of_string a2) in
            let bb = (int_of_string b1, int_of_string b2) in
            loop ((aa, bb) :: acc)
    in
    loop []
;;

let fully_contains ((a1, a2), (b1, b2)) = if (a1 <= b2 && a2 >= b1) || (b1 <= a2 && b2 >= a1) then
        1
    else
        0
;;

let ris = get_pairs () |> List.fold_left (fun acc x -> acc + fully_contains x) 0;;
print_endline (string_of_int ris);;
