use std::{collections::HashMap, fs::read_to_string};

fn calculate(
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    target_x: i64,
    target_y: i64,
    cache: &mut HashMap<(i64, i64), (i64, i64)>,
) -> (i64, i64) {
    if target_x == 0 && target_y == 0 {
        return (0, 0);
    }
    if target_x < 0 || target_y < 0 {
        return (-1, -1);
    }

    match cache.get(&(target_x, target_y)) {
        Some(v) => return *v,
        None => (),
    }

    let (ara, arb) = calculate(a_x, a_y, b_x, b_y, target_x - a_x, target_y - a_y, cache);
    let (bra, brb) = calculate(a_x, a_y, b_x, b_y, target_x - b_x, target_y - b_y, cache);
    if ara == -1 {
        cache.insert((target_x, target_y), (bra, brb + 1));
        return (bra, brb + 1);
    }
    if bra == -1 {
        cache.insert((target_x, target_y), (ara + 1, arb));
        return (ara + 1, arb);
    }

    let aa = ara * 3 + arb;
    let bb = bra * 3 + brb;
    if aa < bb {
        cache.insert((target_x, target_y), (ara + 1, arb));
        return (ara + 1, arb);
    }
    cache.insert((target_x, target_y), (bra, brb + 1));
    return (bra, brb + 1);
}

fn part1() {
    let ris: i64 = read_to_string("input")
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|machine| {
            let config: Vec<(i64, i64)> = machine
                .trim()
                .split('\n')
                .map(|line| {
                    let dd: Vec<i64> = line
                        .split(':')
                        .nth(1)
                        .unwrap()
                        .split(',')
                        .map(|v| v.trim()[2..].parse().unwrap())
                        .collect();
                    (dd.get(0).copied().unwrap(), dd.get(1).copied().unwrap())
                })
                .collect();
            let a = config.get(0).unwrap();
            let b = config.get(1).unwrap();
            let t = config.get(2).unwrap();
            let mut cache = HashMap::new();
            let v = calculate(a.0, a.1, b.0, b.1, t.0, t.1, &mut cache);
            if v.0 == -1 {
                0
            } else {
                v.0 * 3 + v.1
            }
        })
        .sum();
    println!("{}", ris);
}

fn part2() {
}

fn main() {
    part1();
    part2();
}
