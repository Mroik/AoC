let parse_cards () =
    let parse_card data =
        let [@warning "-8"] [_; numbers] = String.split_on_char ':' data in
        let [@warning "-8"] [winners; yours] = String.split_on_char '|' numbers
        |> List.map (String.trim)
        |> List.map (fun x ->
                String.split_on_char ' ' x
                |> List.filter ((<>) "")
                |> List.map (int_of_string)
        ) in
        (winners, yours)
    in

    let rec loop acc =
        try
            let card = read_line () |> parse_card in
            loop (card :: acc)
        with End_of_file -> acc
    in
    loop []
;;

let pile_value cards =
    let card_value card =
        let (winners, yours) = card in
        let many = List.map (fun x -> List.filter ((=) x) yours) winners
        |> List.concat
        |> List.length in
        Float.pow 2.0 (many - 1 |> float_of_int) |> int_of_float
    in
    List.map (card_value) cards |> List.fold_left (+) 0
;;

parse_cards () |> pile_value |> Printf.printf "%d\n";;
