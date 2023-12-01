let digi_set = [
    "one"; "two"; "three"; "four"; "five"; "six"; "seven"; "eight"; "nine";
    "1"; "2"; "3"; "4"; "5"; "6"; "7"; "8"; "9"
];;
let digi_ass = List.mapi (
    fun i v ->
        let ss = String.to_seq v |> List.of_seq in
        let ss = List.nth ss 0 |> int_of_char in
        if ss >= 48 && ss <= 57 then
            (v, ss - 48)
        else
            (v, i + 1)
) digi_set;;
let contains_sub hay needle =
    let rec loop ris off h n =
        if String.length needle |> String.sub h off = needle then
            try loop ((off, needle) :: ris) (off + 1) h n with Invalid_argument _ -> ((off, needle) :: ris)
        else
            try loop ris (off + 1) h n with Invalid_argument _ -> ris
    in
    if String.length hay < String.length needle then
        []
    else
        loop [] 0 hay needle
;;

let contains_digi hay =
    let rec loop ris d =
        match d with
        | [] -> ris
        | x :: xs ->
            match contains_sub hay x with
            | [] -> loop ris xs
            | a -> loop (List.append ris a) xs
    in
    loop [] digi_set
;;

let line () =
    let v = read_line () in
    let vv = contains_digi v |> List.sort (fun (a, _) (b, _) -> a - b) |> List.map (fun (_, x) -> List.assoc x digi_ass) in
    let d = List.nth vv 0 in
    let u = List.length vv - 1 |> List.nth vv in
    d * 10 + u
;;

let solve () =
    let rec loop ris =
        let v = line () in
        try loop (v :: ris) with End_of_file -> List.fold_left (+) 0 (v :: ris)
    in
    loop [] |> print_int;
    print_newline ()
;;

solve ();;
