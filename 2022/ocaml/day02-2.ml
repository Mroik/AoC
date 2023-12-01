exception Exception

type choice =
    | Rock
    | Paper
    | Scissors
;;

let convert x =
    match x with
    | "A" -> Rock
    | "B" -> Paper
    | "C" -> Scissors
    | _ -> raise Exception
;;

let choose combo =
    match combo with
    | (Rock, "X") -> Scissors
    | (Rock, "Z") -> Paper
    | (Paper, "X") -> Rock
    | (Paper, "Z") -> Scissors
    | (Scissors, "X") -> Paper
    | (Scissors, "Z") -> Rock
    | (a, _) -> a

let calculate_win combination =
    let (oppo, me) = combination in
    let bonus = if oppo = me then 3 else 0 in
    match combination with
    | (Scissors, Rock) -> 7 + bonus
    | (_, Rock) -> 1 + bonus
    | (Rock, Paper) -> 8 + bonus
    | (_, Paper) -> 2 + bonus
    | (Paper, Scissors) -> 9 + bonus
    | (_, Scissors) -> 3 + bonus
;;

let get_turns () =
    let ii = open_in "input.txt" in
    let rec loop acc =
        match input_line ii with
        | exception End_of_file -> acc
        | ss ->
            let dd = String.split_on_char ' ' ss in
            let a = List.nth dd 0 |> convert in
            let b = List.nth dd 1 in
            let choice = choose (a, b) in
            loop ((a, choice) :: acc)
    in
    loop []
;;

let turns = get_turns ();;
let tot = List.fold_left (fun acc combo -> acc + (calculate_win combo)) 0 turns;;
print_endline (string_of_int tot);;
