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
        | line ->
            if String.get line 0 = '$' then
                match String.sub line 0 4 with
                | "$ cd" ->
                    if String.get line 5 = '.' then
                        let [@warning "-8"] Some new_curr = find_father root current_index in
                        loop root new_curr index
                    else
                        let [@warning "-8"] [_; _; name] = String.split_on_char ' ' line in
                        let new_root = add_node root current_index (Dir (index, name, [])) in
                        loop new_root index (index + 1)
                | "$ ls" -> loop root current_index index
                | _ -> raise Exception
            else
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
    | Dir (_, _, subs) ->
        loop_calculate_fs_size 0 subs
and loop_calculate_fs_size acc ss =
    match ss with
    | [] -> acc
    | x :: xs ->
        let c = calculate_fs_size x in
        loop_calculate_fs_size (acc + c) xs
;;

let rec at_most min root =
    let [@warning "-8"] Dir (_, _, subs) = root in
    let size = calculate_fs_size root in
    let sub_sizes = loop_at_most [] min subs in
    if size >= min then
        size :: sub_sizes
    else
        sub_sizes
and loop_at_most acc min ss =
    match ss with
    | [] -> acc
    | Dir (i, n, s) :: xs ->
        let sizes = at_most min (Dir (i, n, s)) in
        loop_at_most (List.append sizes acc) min xs
    | _ :: xs -> loop_at_most acc min xs
;;

let fs = parse_shell ();;
let total = calculate_fs_size fs;;
let free_at_least = 30000000 - (70000000 - total);;
let candidates = at_most free_at_least fs;;
let ris = List.fold_left (min) 70000000 candidates;;
string_of_int ris |> print_endline;;
