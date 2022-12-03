exception Exception

let convert x =
    let cc = Char.code x in
    if cc >= 97 && cc <= 122 then
        cc - 96
    else
        cc - 38
;;

let [@warning "-8"] find_common (a :: b) =
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

let ris = get_rucksacks () |> List.map (find_common) |> List.fold_left (fun acc x -> acc + x) 0;;
print_endline (string_of_int ris);;
