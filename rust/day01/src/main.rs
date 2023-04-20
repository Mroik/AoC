use std::fs::read_to_string;

fn main() {
    part1();
    part2();
}

fn part1() {
    let buf = read_to_string("input.txt").unwrap();
    let mut tot = 0;
    let mut max = 0;
    for item in buf.split("\n") {
        let cur = item.trim();
        if cur == "" {
            tot = 0;
        } else {
            tot += cur.parse::<i32>().unwrap();
        }
        if tot > max {
            max = tot;
        }
    }
    println!("{max}");
}

fn part2() {
    let buf = read_to_string("input.txt").unwrap();
    let mut tot;
    let mut max = vec![0, 0, 0];
    for bag in buf.split("\n\n") {
        let mut tot = 0;
        for item in bag.trim().split("\n") {
            tot += item.parse::<i32>().unwrap_or(0);
        }
        for x in 0..3 {
            if tot > *max.get(x).unwrap() {
                max.insert(x, tot);
                max.pop();
                break;
            }
        }
    }

    tot = 0;
    for inv in max {
        tot += inv;
    }
    println!("{tot}");
}
