exception Exception

let rec debug stacks =
    let rec inner ss =
        match ss with
        | [] -> ()
        | a :: b ->
            let () = String.init 1 (fun x -> a) |> print_string in
            inner b
    in
    match stacks with
    | [] -> ()
    | a :: b ->
        let () = inner a in
        let () = print_string "\n" in
        debug b
;;

let build_crates () =
    let contains_number ss = String.exists (fun x -> let aa = Char.code x in aa >= 48 && aa <= 57) ss in
    let rec strip_empty acc lst =
        match lst with
        | [] -> acc
        | "" :: rest -> let [@warning "-8"] _ :: _ :: _ :: rr = rest in strip_empty ([] :: acc) rr
        | a :: ss -> strip_empty ([(String.get a 1)] :: acc) ss
    in

    let ii = open_in "input.txt" in
    let rec parse_crates stacks =
        let line = input_line ii in
        if contains_number line then
            List.map (fun x -> List.rev x) stacks |> List.rev
        else
            let to_add = String.split_on_char ' ' line |> strip_empty [] in
            let new_stacks = List.map2 (fun x y -> match y with [] -> x | s :: _ -> s :: x) stacks to_add in
            parse_crates new_stacks
    in
    
    let rec update_stack acc stacks new_data target =
        match stacks with
        | [] -> List.rev acc
        | a :: b when target = 0 -> update_stack (new_data :: acc) b new_data (target - 1)
        | a :: b -> update_stack (a :: acc) b new_data (target - 1)
    in

    let rec remove n ss = if n = 0 then ss else remove (n - 1) (List.tl ss) in

    let rec parse_commands stacks =
        (*
        let () = debug stacks in
        let () = print_endline "-------" in
        *)
        match input_line ii with
        | exception End_of_file -> stacks
        | ss ->
            let tokens = String.split_on_char ' ' ss in
            (* let () = print_endline ss in *)
            let m = List.nth tokens 1 |> int_of_string in
            let f = (List.nth tokens 3 |> int_of_string) - 1 in
            let t = (List.nth tokens 5 |> int_of_string) - 1 in
            let from_stack = (List.nth stacks f) in
            let lifeted_crates = List.init m (List.nth from_stack) |> List.rev in
            let new_stack_t = List.append lifeted_crates (List.nth stacks t) in
            let new_stack_f = remove m from_stack in
            let new_stacks = update_stack [] stacks new_stack_f f in
            parse_commands (update_stack [] new_stacks new_stack_t t)
    in

    let init_stack = String.split_on_char ' ' (input_line ii) |> strip_empty [] in
    let stacks = parse_crates init_stack in
    let _ = input_line ii in
    parse_commands stacks
;;

let stacks = build_crates ();;
List.iter (fun x -> print_string (String.init 1 (fun a -> List.nth x a))) stacks;;
print_string "\n";;
