let rec power_of games ris =
    let rec min_game ris game =
        match game with
        | [] -> ris
        | x :: xs ->
            let [@warning "-8"] (rn, gn, bn) = ris in
            let [@warning "-8"] [n; color] = String.split_on_char ' ' x in
            let n = int_of_string n in
            match color with
            | "red" when rn < n -> min_game (n, gn, bn) xs
            | "green" when gn < n -> min_game (rn, n, bn) xs
            | "blue" when bn < n -> min_game (rn, gn, n) xs
            | _ -> min_game ris xs
    in

    match games with
    | [] ->
        let (a, b, c) = ris in
        a * b * c
    | x :: xs -> min_game ris x |> power_of xs
;;

let parse_game data =
    let [@warning "-8"] [mmg; mgames] = String.split_on_char ':' data in
    let games = String.split_on_char ';' mgames |> List.map (fun x ->
        String.trim x |> String.split_on_char ',' |> List.map (String.trim)
    ) in
    power_of games
;;

let rec loop tot =
    try
        let data = read_line () in
        parse_game data (0, 0, 0) + tot |> loop
    with End_of_file -> tot
;;

let tot = loop 0;;
Printf.printf "%d\n" tot;;
