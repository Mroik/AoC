let find_common rucksack =
    let l_rucksack = String.length rucksack in
    let t1 = String.sub rucksack 0 (l_rucksack / 2) in
    let t2 = String.sub rucksack (l_rucksack / 2) (l_rucksack / 2) in
    let a = List.init (String.length t1) (String.get t1) in
    let b = List.init (String.length t2) (String.get t2) in

    let contains v ll = List.exists (fun x -> x = v) ll in
    List.fold_left (fun acc x -> if contains x b then (x :: acc) else acc) [] a
;;

let get_rucksacks () =
    let ii = open_in "input.txt" in
    let rec loop acc =
        match input_line ii with
        | exception End_of_file -> acc
        | ss -> loop (ss :: acc)
    in
    loop []
;;

let convert x =
    let cc = Char.code x in
    if cc >= 97 && cc <= 122 then
        cc - 96
    else
        cc - 38
;;

let make_unique lst =
    let rec loop acc ss =
        match ss with
        | [] -> acc
        | a :: b -> if List.exists (fun x -> x = a) acc then acc else loop (a :: acc) b
    in
    loop [] lst
;;


let sacks = get_rucksacks ();;
let sums = List.map (fun x -> find_common x |> make_unique |> List.map (convert) |> List.fold_left (fun acc x -> acc + x) 0) sacks;;
let ris = List.fold_left (fun acc x -> acc + x) 0 sums;;
print_endline (string_of_int ris);;
