use std::fs::read_to_string;

fn main() {
    part1();
    part2();
}

fn convert(x: char) -> i32 {
    match x {
        'A' | 'X' => 0,
        'B' | 'Y' => 1,
        'C' | 'Z' => 2,
        _ => unreachable!(),
    }
}

fn part1() {
    fn score(x: i32, y: i32) -> i32 {
        let mut tot = x + 1;
        if x == (y + 1) % 3 {
            tot += 6;
        } else if x == y {
            tot += 3;
        }
        return tot;
    }

    let buf = read_to_string("input.txt").unwrap();
    let mut scre = 0;
    for match_ in buf.trim().split("\n") {
        let their = convert(match_.chars().nth(0).unwrap());
        let mine = convert(match_.chars().nth(2).unwrap());
        scre += score(mine, their);
    }
    println!("{scre}");
}

fn part2() {
    fn score(x: i32, is_win: i32) -> i32 {
        let tot = match is_win {
            0 => 0 + (x + 2) % 3,
            1 => 3 + x,
            2 => 6 + (x + 1) % 3,
            _ => unreachable!(),
        };
        return tot + 1;
    }

    let buf = read_to_string("input.txt").unwrap();
    let mut scre = 0;
    for match_ in buf.trim().split("\n") {
        let their = convert(match_.chars().nth(0).unwrap());
        let is_win = convert(match_.chars().nth(2).unwrap());
        scre += score(their, is_win);
    }
    println!("{scre}");
}
