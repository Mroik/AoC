use std::{collections::HashMap, fs::read_to_string};

fn blink(n: u64, step: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    if step == 0 {
        return 1;
    }
    if n == 0 {
        return blink(1, step - 1, cache);
    }
    if let Some(v) = cache.get(&(n, step)) {
        return *v;
    }
    let cl = ((n as f64).log10().floor() + 1.0) as u64;
    if cl % 2 == 1 {
        let ris = blink(n * 2024, step - 1, cache);
        cache.insert((n, step), ris);
        return ris;
    }
    let ris = blink(n / (10_u64.pow(cl as u32 / 2)), step - 1, cache)
        + blink(n % (10_u64.pow(cl as u32 / 2)), step - 1, cache);
    cache.insert((n, step), ris);
    return ris;
}

fn part1() {
    let mut cache = HashMap::new();
    let ris: u64 = read_to_string("input")
        .unwrap()
        .trim()
        .split(' ')
        .map(|n| blink(n.parse().unwrap(), 25, &mut cache))
        .sum();
    println!("{}", ris);
}

fn part2() {
    let mut cache = HashMap::new();
    let ris: u64 = read_to_string("input")
        .unwrap()
        .trim()
        .split(' ')
        .map(|n| blink(n.parse().unwrap(), 75, &mut cache))
        .sum();
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
