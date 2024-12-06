use std::{cmp::Ordering, collections::HashMap, fs::read_to_string};

fn part1() {
    let buf = read_to_string("input").unwrap();
    let mut it = buf.trim().split("\n\n");
    let mut rules = HashMap::new();

    it.next().unwrap().trim().split('\n').for_each(|line| {
        let mut ii = line.trim().split('|').map(|v| v.parse::<u32>().unwrap());
        let key = ii.next().unwrap();
        if rules.get(&key).is_none() {
            rules.insert(key, Vec::new());
        }
        rules.get_mut(&key).unwrap().push(ii.next().unwrap());
    });

    let ris: u32 = it
        .next()
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| {
            let update: Vec<u32> = line
                .trim()
                .split(',')
                .map(|v| v.parse::<u32>().unwrap())
                .collect();
            for x in 0..update.len() - 1 {
                for y in x + 1..update.len() {
                    let left = update.get(x).unwrap();
                    let right = update.get(y).unwrap();
                    if let Some(v) = rules.get(right) {
                        if v.contains(left) {
                            return 0;
                        }
                    }
                }
            }
            update.get(update.len() / 2).copied().unwrap()
        })
        .sum();
    println!("{}", ris);
}

fn part2() {
    let buf = read_to_string("input").unwrap();
    let mut it = buf.trim().split("\n\n");
    let mut rules = HashMap::new();

    it.next().unwrap().trim().split('\n').for_each(|line| {
        let mut ii = line.trim().split('|').map(|v| v.parse::<u32>().unwrap());
        let key = ii.next().unwrap();
        if rules.get(&key).is_none() {
            rules.insert(key, Vec::new());
        }
        rules.get_mut(&key).unwrap().push(ii.next().unwrap());
    });

    let ris: u32 = it
        .next()
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| {
            line.trim()
                .split(',')
                .map(|v| v.parse::<u32>().unwrap())
                .collect()
        })
        .filter(|update: &Vec<u32>| {
            for x in 0..update.len() - 1 {
                for y in x + 1..update.len() {
                    let left = update.get(x).unwrap();
                    let right = update.get(y).unwrap();
                    if let Some(v) = rules.get(right) {
                        if v.contains(left) {
                            return true;
                        }
                    }
                }
            }
            return false;
        })
        .map(|update| {
            let mut u = update.clone();
            u.sort_by(|a, b| {
                if let Some(v) = rules.get(b) {
                    if v.contains(a) {
                        return Ordering::Greater;
                    }
                }
                return Ordering::Less;
            });
            u.get(u.len() / 2).copied().unwrap()
        })
        .sum();
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
