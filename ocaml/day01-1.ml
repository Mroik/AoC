let max_calories inventories =
    let sums = List.map (fun x -> List.fold_left (fun acc v -> acc + v) 0 x) inventories in
    List.fold_left (fun acc x -> if x > acc then x else acc) (List.nth sums 0) sums
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
