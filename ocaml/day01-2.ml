let max_calories inventories =
    let sums = List.map (fun x -> List.fold_left (fun acc v -> acc + v) 0 x) inventories
    |> List.sort (fun x y -> y - x) in
    let top_3 = List.init 3 (fun x -> List.nth sums x) in
    List.fold_left (fun acc x -> acc + x) 0 top_3
;;

let make_inventories () =
    let ii = open_in "input.txt" in
    let rec loop acc inv =
        match input_line ii with
        | "" -> loop (inv :: acc) []
        | exception End_of_file -> inv :: acc
        | line -> loop acc ((int_of_string line) :: inv)
    in
    loop [] []
;;

let result = max_calories (make_inventories ()) |> string_of_int;;
print_endline result;;
