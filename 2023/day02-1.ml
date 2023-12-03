let rec is_possible games =
    let rec is_possible_single game =
        match game with
        | [] -> true
        | x :: xs ->
            let [@warning "-8"] [n; color] = String.split_on_char ' ' x in
            let n = int_of_string n in
            match color with
            | "red" when n <= 12 -> is_possible_single xs
            | "green" when n <= 13 -> is_possible_single xs
            | "blue" when n <= 14 -> is_possible_single xs
            | _ -> false
    in

    match games with
    | [] -> true
    | game :: xs ->
        if is_possible_single game then
            is_possible xs
        else
            false
;;

let parse_game data =
    let [@warning "-8"] [mmg; mgames] = String.split_on_char ':' data in
    let [@warning "-8"] [_; mm] = String.split_on_char ' ' mmg in
    let game_id = int_of_string mm in
    let games = String.split_on_char ';' mgames |> List.map (fun x ->
        String.trim x |> String.split_on_char ',' |> List.map (String.trim)
    ) in
    if is_possible games then
        game_id
    else
        0
;;

let rec loop tot =
    try
        let data = read_line () in
        parse_game data + tot |> loop
    with End_of_file -> tot
;;

let tot = loop 0;;
Printf.printf "%d\n" tot;;
