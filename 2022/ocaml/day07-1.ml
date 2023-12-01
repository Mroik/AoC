exception Exception

type filesystem =
    | File of int * string * int
    | Dir of int * string * filesystem list
;;

let rec add_node root father_index new_node =
    let [@warning "-8"] Dir (index, name, subs) = root in
    if father_index = index then
        Dir (index, name, new_node :: subs)
    else
        let new_subs = loop_add_node [] subs father_index new_node in
        Dir (index, name, new_subs)
and loop_add_node acc ss father_index new_node =
    match ss with
    | [] -> acc
    | Dir (i, n, s) :: xs ->
        let aa = add_node (Dir (i, n, s)) father_index new_node in
        loop_add_node (aa :: acc) xs father_index new_node
    | x :: xs -> loop_add_node (x :: acc) xs father_index new_node
;;

let rec find_father root index =
    let is_same_index x =
        match x with
        | Dir (i, _, _) -> index = i
        | File (i, _, _) -> index = i
    in

    let [@warning "-8"] Dir (i, rname, subs) = root in
    if List.exists (is_same_index) subs then
        Some i
    else
        loop_find_father subs index
and loop_find_father ss index =
    match ss with
    | [] -> None
    | Dir (i, name, subs) :: xs ->
        let ds = find_father (Dir (i, name, subs)) index in
        begin
        match ds with
        | None -> loop_find_father xs index
        | _ -> ds
        end
    | _ :: xs -> loop_find_father xs index
;;

let parse_shell () =
    let ii = open_in "input.txt" in
    let _ = input_line ii in

    let rec loop root current_index index =
        match input_line ii with
        | exception End_of_file -> root
        | line when String.get line 0 = '$' ->
            begin
            match String.sub line 0 4 with
            | "$ cd" when String.get line 5 = '.' ->
                let [@warning "-8"] Some new_curr = find_father root current_index in
                loop root new_curr index
            | "$ cd" ->
                let [@warning "-8"] [_; _; name] = String.split_on_char ' ' line in
                let new_root = add_node root current_index (Dir (index, name, [])) in
                loop new_root index (index + 1)
            | "$ ls" -> loop root current_index index
            | _ -> raise Exception
            end
        | line ->
            let [@warning "-8"] [a; b] = String.split_on_char ' ' line in
            if a = "dir" then
                loop root current_index index
            else
                let new_root = add_node root current_index (File (index, b, int_of_string a)) in
                loop new_root current_index (index + 1)
    in

    loop (Dir (0, "/", [])) 0 1
;;

let rec debug root =
    let index, name =
        match root with
        | Dir (i, name, _) -> i, name
        | File (i, name, _) -> i, name
    in
    let subs =
        match root with
        | Dir (_, _, subs) -> Some subs
        | _ -> None
    in
    let () = string_of_int index |> print_string in
    let () = print_string " " in
    let () = print_endline name in
    match subs with
    | None -> ()
    | Some x -> loop_debug x
and loop_debug ss =
    match ss with
    | [] -> ()
    | x :: xs ->
        let () = debug x in
        loop_debug xs
;;

let rec calculate_fs_size root =
    match root with
    | File (_, _, size) -> size
    | Dir (_, _, subs) -> loop_calculate_fs_size 0 subs
and loop_calculate_fs_size acc ss =
    match ss with
    | [] -> acc
    | x :: xs ->
        let c = calculate_fs_size x in
        loop_calculate_fs_size (acc + c) xs
;;

let rec at_most root =
    let [@warning "-8"] Dir (_, _, subs) = root in
    let size = calculate_fs_size root in
    let subs_size = loop_at_most [] subs |> List.fold_left (+) 0 in
    if size <= 100000 then
        size + subs_size
    else
        subs_size
and loop_at_most acc ss =
    match ss with
    | [] -> acc
    | Dir (i, n, s) :: xs ->
        let size = at_most (Dir (i, n, s)) in
        loop_at_most (size :: acc) xs
    | _ :: xs -> loop_at_most acc xs
;;

let fs = parse_shell ();;
(* debug fs;; *)
at_most fs |> print_int;;
