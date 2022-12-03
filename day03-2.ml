exception Exception

let convert x =
    let cc = Char.code x in
    if cc >= 97 && cc <= 122 then
        cc - 96
    else
        cc - 38
;;

(* Produces a warning since `[]` is not matched *)
let find_common (a :: b) =
    let contains v ll = List.exists (fun x -> x = v) ll in
    let rec loop ss =
        match ss with
        | [] -> raise Exception
        | x :: xs ->
            let temp = List.map (contains x) b in
            if List.fold_left (fun acc x -> acc && x) true temp then
                convert x
            else
                loop xs
    in
    loop a
;;

let get_rucksacks () =
    let ii = open_in "input.txt" in
    let rec loop acc team =
        match List.length team with
        | 3 -> loop (team :: acc) []
        | _ ->
            match input_line ii with
            | exception End_of_file -> acc
            | ss -> loop acc ((List.init (String.length ss) (String.get ss)) :: team)
    in
    loop [] []
;;

let make_unique lst =
    let rec loop acc ss =
        match ss with
        | [] -> acc
        | a :: b -> if List.exists (fun x -> x = a) acc then acc else loop (a :: acc) b
    in
    loop [] lst
;;

let ris = get_rucksacks () |> List.map (fun x -> find_common x) |> List.fold_left (fun acc x -> acc + x) 0;;
print_endline (string_of_int ris);;
