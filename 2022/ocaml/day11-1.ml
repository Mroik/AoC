exception Exception

type monkey = Monkey of (int -> int) * int * int * int;;

let parse_monkey ii =
    let item_line = input_line ii in
    let items =
        String.length item_line - 18
        |> String.sub item_line 18
        |> String.split_on_char ','
        |> List.map (fun a -> String.trim a |> int_of_string)
    in

    let op_line = input_line ii in
    let op =
        match String.get op_line 23 with
        | '*' -> ( * )
        | '+' -> ( + )
        | _ -> raise Exception
    in
    let operation =
        let n = String.length op_line - 25 |> String.sub op_line 25 in
        match n with
        | "old" -> (fun a -> op a a)
        | _ -> (fun a -> int_of_string n |> op a)
    in

    let test_line = input_line ii in
    let test = String.length test_line - 21 |> String.sub test_line 21 |> String.trim |> int_of_string in

    let true_line = input_line ii in
    let t_con = (String.get true_line 29 |> int_of_char) - 48 in

    let false_line = input_line ii in
    let f_con = (String.get false_line 30 |> int_of_char) - 48 in

    Monkey (operation, test, t_con, f_con), items
;;

let parse_input () =
    let ii = open_in "input.txt" in
    let rec loop monkeys item_queues =
        match input_line ii with
        | exception End_of_file -> (List.rev monkeys), (List.rev item_queues)
        | line ->
            match line with
            | "" -> loop monkeys item_queues
            | keyword ->
                match String.sub keyword 0 6 with
                | "Monkey" ->
                    let mm, items = parse_monkey ii in
                    loop (mm :: monkeys) (items :: item_queues)
                | _ -> raise Exception
    in
    loop [] []
;;

let calculate_worry monkey item =
    let Monkey (f, test, tt, ff) = monkey in
    let new_worry = (f item) / 3 in
    let throw_to = if new_worry mod test = 0 then tt else ff in
    new_worry, throw_to
;;

let rec turn id monkey items queues =
    match items with
    | [] -> queues
    | item :: rest ->
        let new_worry, receiver = calculate_worry monkey item in
        let new_queues = List.mapi (fun i ii ->
            if i = receiver then
                List.append ii [new_worry]
            else if i = id then
                []
            else
                ii
        ) queues in
        turn id monkey rest new_queues
;;

let rec round monkeys id many queues =
    if id = List.length monkeys then
        queues, many
    else
        let items = (List.nth queues id) in
        let new_queues = turn id (List.nth monkeys id) items queues in
        let new_many = List.mapi (fun i ii -> if i = id then ii + List.length items else ii) many in
        round monkeys (id + 1) new_many new_queues
;;

let rec times monkeys queues many time =
    if time = 20 then
        many
    else
        let new_queues, new_many = round monkeys 0 many queues in
        times monkeys new_queues new_many (time + 1)
;;


let monkeys, queues = parse_input ();;
let many = times monkeys queues (List.init (List.length monkeys) (fun _ -> 0)) 0;;
let sorted = List.sort (fun a b -> b - a) many;;
List.nth sorted 0 * (List.nth sorted 1) |> string_of_int |> print_endline;;
