let parse_almanac () =
    let parse_seeds () =
        let data = read_line () in
        let _ = read_line () in
        List.nth (String.split_on_char ':' data) 1
        |> String.trim
        |> String.split_on_char ' '
        |> List.map (int_of_string)
    in

    let parse_parts () =
        let _ = read_line () in
        let rec loop acc =
            try
                let data = read_line () in
                if data = "" then
                    acc
                else
                    let [@warning "-8"] [dest; src; len] =
                        String.split_on_char ' ' data
                        |> List.map (int_of_string)
                    in
                    loop ((dest, src, len) :: acc)
            with End_of_file -> acc
        in
        loop []
    in

    let seeds = parse_seeds () in
    let maps = List.init 7 (fun _ -> parse_parts ()) in
    (seeds, maps)
;;

let transfer maps seed =
    let conv acc maps =
        let res = List.filter (fun (dst, src, len) -> acc >= src && acc < src + len) maps in
        match res with
        | [] -> acc
        | [(dst, src, len)] -> dst + (acc - src)
        | _ -> Invalid_argument "" |> raise
    in
    List.fold_left (conv) seed maps
;;

let find_lowest maps seeds =
    List.map (transfer maps) seeds |> List.fold_left (Int.min) Int.max_int
;;

let (seeds, maps) = parse_almanac ();;
find_lowest maps seeds |> Printf.printf "%d\n";;
