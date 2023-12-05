module IMap = Map.Make (Int);;

let parse_cards () =
    let parse_card data =
        let [@warning "-8"] [iid; numbers] = String.split_on_char ':' data in
        let card_id = List.nth (String.split_on_char ' ' iid |> List.filter ((<>) "")) 1 |> int_of_string in
        let [@warning "-8"] [winners; yours] = String.split_on_char '|' numbers
        |> List.map (String.trim)
        |> List.map (fun x ->
                String.split_on_char ' ' x
                |> List.filter ((<>) "")
                |> List.map (int_of_string)
        ) in
        (card_id, winners, yours)
    in

    let rec loop acc =
        try
            let card = read_line () |> parse_card in
            loop (card :: acc)
        with End_of_file -> acc
    in
    loop [] |> List.rev
;;

let pile_value cards =
    let rewards_list card =
        let (card_id, winners, yours) = card in
        let many = List.map (fun x -> List.exists ((=) x) yours) winners
        |> List.filter ((=) true)
        |> List.length in
        let reward = List.init many (fun x -> card_id + 1 + x) in
        (card_id, reward)
    in

    let rews = List.map (rewards_list) cards in
    let rec tot_of acc cache (id, rv) =
        match rv with
        | [] ->
            let new_cache = IMap.add id (acc) cache in
            (acc, new_cache)
        | x :: xs ->
            let (sid, srv) = List.nth rews (x - 1) in
            let (stot, new_cache) =
                match IMap.find_opt sid cache with
                | Some a -> (a, cache)
                | None ->
                    tot_of 0 cache (sid, srv)
            in
            tot_of (acc + stot + 1) new_cache (id, xs)
    in

    let rec loop acc cache lst =
        match lst with
        | [] -> (acc, cache)
        | (id, rv) :: xs ->
            match IMap.find_opt id cache with
            | Some x -> loop (acc + x + 1) cache xs
            | None ->
                let (tot_sub, new_cache) = tot_of 0 cache (id, rv) in
                loop (acc + tot_sub + 1) new_cache xs
    in
    let (ris, _) = loop 0 IMap.empty rews in
    ris
;;

parse_cards () |> pile_value |> Printf.printf "%d\n";;
