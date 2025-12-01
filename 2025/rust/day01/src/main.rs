use std::fs::read_to_string;

fn part1() {
    let mut cur = 50;
    let mut ris = 0;
    read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .for_each(|line| {
            let mut v: i32 = line.trim()[1..].parse().unwrap();
            match line.trim().chars().nth(0).unwrap() {
                'L' => v = -v,
                _ => (),
            }
            cur += v;
            while cur < 0 || cur > 99 {
                if cur < 0 {
                    cur += 100;
                } else if cur > 99 {
                    cur -= 100;
                }
            }

            if cur == 0 {
                ris += 1;
            }
        });
    println!("{}", ris);
}

fn part2() {
    let mut cur = 50;
    let mut ris = 0;
    read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .for_each(|line| {
            let mut v: i32 = line.trim()[1..].parse().unwrap();
            match line.trim().chars().nth(0).unwrap() {
                'L' => v = -v,
                _ => (),
            }
            if cur == 0 && v < 0 {
                ris -= 1;
            }
            cur += v;
            while cur < 0 || cur > 99 {
                if cur < 0 {
                    cur += 100;
                } else if cur > 99 {
                    cur -= 100;
                }
                ris += 1;
            }
            if cur == 0 && v < 0 {
                ris += 1;
            }
        });
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
