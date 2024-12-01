use std::io::stdin;

fn part1() {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut buf = String::new();
    loop {
        buf.clear();
        match stdin().read_line(&mut buf) {
            Ok(_) => {
                if buf.trim() == "" {
                    break;
                }
                let mut vals = buf.trim().split("   ").map(|v| v.parse::<i32>().unwrap());
                left.push(vals.next().unwrap());
                right.push(vals.next().unwrap());
            }
            Err(_) => break,
        }
    }
    left.sort();
    right.sort();
    let ris: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    println!("{}", ris);
}

fn part2() {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut buf = String::new();
    loop {
        buf.clear();
        match stdin().read_line(&mut buf) {
            Ok(_) => {
                if buf.trim() == "" {
                    break;
                }
                let mut vals = buf.trim().split("   ").map(|v| v.parse::<i32>().unwrap());
                left.push(vals.next().unwrap());
                right.push(vals.next().unwrap());
            }
            Err(_) => break,
        }
    }
    let ris: i32 = left
        .iter()
        .map(|v| right.iter().filter(|a| **a == *v).count() as i32 * v)
        .sum();
    println!("{}", ris);
}

fn main() {
    //part1();
    part2();
}
