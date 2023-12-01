exception Exception

let parse_map () =
    let ii = open_in "input.txt" in
    let rec loop acc =
        match input_line ii with
        | exception End_of_file -> acc
        | temp ->
            let row = List.init (String.length temp) (fun x -> (String.get temp x |> Char.code) - 48) in
            loop (row :: acc)
    in
    loop [] |> List.rev
;;

let get_column data column = List.map (fun x -> List.nth x column) data;;
let get_row data row = List.nth data row;;

let visibles data size =
    let rec count acc target ss =
        match ss with
        | [] -> acc
        | x :: xs when x >= target -> acc + 1
        | x :: xs -> count (acc + 1) target xs
    in

    let scenic_score ss row column =
        let ro = get_row data row in
        let col = get_column data column in

        let top = List.filteri (fun i _ -> i < row) col |> List.rev |> count 0 (List.nth col row) in
        let bottom = List.filteri (fun i _ -> i > row) col |> count 0 (List.nth col row) in
        let left = List.filteri (fun i _ -> i < column) ro |> List.rev |> count 0 (List.nth ro column) in
        let right = List.filteri (fun i _ -> i > column) ro |> count 0 (List.nth ro column) in
        top * bottom * left * right
    in

    let rec loop acc row column =
        if row = size then
            acc
        else if column = size then
            loop acc (row + 1) 0
        else
            let score = scenic_score data row column in
            if score > acc then
                loop score row (column + 1)
            else
                loop acc row (column + 1)
    in
    loop 0 0 0
;;

let data = parse_map ();;
let ris = List.length data |> visibles data;;
string_of_int ris |> print_endline;;
