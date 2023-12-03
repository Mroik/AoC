let schema =
    let rec loop acc =
        try
            let line = read_line () |> String.to_seq |> List.of_seq in
            loop (line :: acc)
        with End_of_file -> List.rev acc
    in
    loop []
;;

let find_sym schema =
    let rec loop acc n schema =
        match schema with
        | [] -> acc
        | x :: xs ->
            let sym_in_line =
                List.mapi (fun i x -> if x <> '.' && (x < '0' || x > '9') then i else -1) x
                |> List.filter (fun x -> x > -1)
                |> List.map (fun x -> (n, x))
            in
            loop (acc @ sym_in_line) (n + 1) xs
    in
    loop [] 0 schema
;;

let find_nums schema =
    let rec find_nums_in_line acc n start data =
        match data with
        | [] -> if start < 0 then acc else (start, n) :: acc
        | x :: xs when x >= '0' && x <= '9' && start < 0 -> find_nums_in_line acc (n + 1) n xs
        | x :: xs when x >= '0' && x <= '9' -> find_nums_in_line acc (n + 1) start xs
        | x :: xs when start < 0 -> find_nums_in_line acc (n + 1) start xs
        | x :: xs -> find_nums_in_line ((start, n) :: acc) (n + 1) (-1) xs
    in

    let rec loop acc n schema =
        match schema with
        | [] -> acc
        | x :: xs ->
            let nums_in_line = find_nums_in_line [] 0 (-1) x |> List.map (fun x -> (n, x)) in
            loop (nums_in_line @ acc) (n + 1) xs
    in
    loop [] 0 schema
;;

let int_of_coord schema (line, (start, finish)) =
    List.nth schema line
    |> List.filteri (fun i _ -> not (i < start))
    |> List.filteri (fun i _ -> i < finish - start)
    |> List.to_seq
    |> String.of_seq
    |> int_of_string
;;

let sym_pos = find_sym schema;;
let num_pos = find_nums schema;;

let parts =
    let adjacent sym num =
        let (nl, (ns, nf)), (sl, sr) = num, sym in
        sl >= nl - 1 && sl <= nl + 1 && sr >= ns - 1 && sr < nf + 1
    in
    let parts nums sym = List.filter (adjacent sym) nums in
    List.map (parts num_pos) sym_pos |> List.concat |> List.map (int_of_coord schema)
;;

let tot = List.fold_left (+) 0 parts;;
Printf.printf "%d\n" tot;;
