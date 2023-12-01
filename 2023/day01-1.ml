let solve () =
    let v = read_line () |> String.to_seq |> List.of_seq in
    let digi = List.filter (fun x -> x >= '0' && x <= '9') v |> List.map (fun x -> int_of_char x - 48) in
    let d = List.nth digi 0 in
    let u = List.length digi - 1 |> List.nth digi in
    d * 10 + u
;;

let rec loop sums =
    let v = solve () in
    print_int v;
    print_newline ();
    try loop (v :: sums) with End_of_file -> List.fold_left (+) 0 (v :: sums)
;;

loop [] |> print_int;;
print_newline ();;
