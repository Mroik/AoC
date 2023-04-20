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
    let rec tallest_side ss height index =
        if index = 0 then
            true
        else
            let [@warning "-8"] x :: xs = ss in
            if x >= height then
                false
            else
                tallest_side xs height (index - 1)
    in

    let rec loop acc row column =
        if row = size then
            acc
        else if column = size then
            loop acc (row + 1) 0
        else
            let cur_row = get_row data row in
            let cur_col = get_column data column in
            let rev_row = List.rev cur_row in
            let rev_col = List.rev cur_col in

            let from_left = tallest_side cur_row (List.nth cur_row column) column in
            let from_right = tallest_side rev_row (List.nth rev_row (size - 1 - column)) (size - 1 - column) in

            let from_top = tallest_side cur_col (List.nth cur_col row) row in
            let from_bottom = tallest_side rev_col (List.nth rev_col (size - 1 - row)) (size - 1 - row) in

            if from_left || from_right || from_top || from_bottom then
                loop (acc + 1) row (column + 1)
            else
                loop acc row (column + 1)
    in
    loop 0 0 0
;;

let data = parse_map ();;
let ris = List.length data |> visibles data;;
string_of_int ris |> print_endline;;
