use std::fs::read_to_string;

fn main() {
    part1();
    part2();
}

fn conv(x: (&str, &str)) -> (i32, i32) {
    (x.0.parse().unwrap(), x.1.parse().unwrap())
}

fn part1() {
    let buf = read_to_string("input.txt").unwrap();
    let mut tot = 0;
    for pair in buf.trim().split("\n") {
        let (first, second) = pair.split_once(",").unwrap();
        let (f1, f2) = conv(first.split_once("-").unwrap());
        let (s1, s2) = conv(second.split_once("-").unwrap());
        if (f1 <= s1 && f2 >= s2) || (s1 <= f1 && s2 >= f2) {
            tot += 1;
        }
    }

    println!("{tot}");
}

fn part2() {
    let buf = read_to_string("input.txt").unwrap();
    let mut tot = 0;
    for pair in buf.trim().split("\n") {
        let (first, second) = pair.split_once(",").unwrap();
        let (f1, f2) = conv(first.split_once("-").unwrap());
        let (s1, s2) = conv(second.split_once("-").unwrap());
        if (f1 <= s1 && f2 >= s1) || (s1 <= f1 && s2 >= f1) {
            tot += 1;
        }
    }

    println!("{tot}");
}
