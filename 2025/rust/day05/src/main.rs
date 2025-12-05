use std::{fs::read_to_string, mem::swap};

fn part1() {
    let data = read_to_string("input").unwrap();
    let mut source = data.trim().split("\n\n");
    let fresh: Vec<(u64, u64)> = source
        .next()
        .unwrap()
        .split('\n')
        .map(|l| {
            let mut s = l.trim().split('-');
            let left = s.next().unwrap();
            let right = s.next().unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();
    let ingredients: Vec<u64> = source
        .next()
        .unwrap()
        .split('\n')
        .map(|v| v.parse().unwrap())
        .collect();

    let ris = ingredients
        .iter()
        .filter(|&&ing| fresh.iter().any(|&(l, r)| ing >= l && ing <= r))
        .count();
    println!("{}", ris);
}

fn meld(range: (u64, u64), other: (u64, u64)) -> Option<(u64, u64)> {
    if range.0 > other.1 || range.1 < other.0 {
        return None;
    }
    if range.0 <= other.0 && range.1 >= other.1 {
        return Some(range);
    }
    if range.0 >= other.0 && range.1 <= other.1 {
        return Some(other);
    }
    if range.0 <= other.0 && range.1 <= other.1 {
        return Some((range.0, other.1));
    }
    if other.0 <= range.0 && other.1 <= range.1 {
        return Some((other.0, range.1));
    }
    return None;
}

fn part2() {
    let data = read_to_string("input").unwrap();
    let mut source = data.trim().split("\n\n");
    let mut fresh: Vec<(u64, u64)> = Vec::new();
    let mut to_add = Vec::new();
    source
        .next()
        .unwrap()
        .split('\n')
        .map(|l| {
            let mut s = l.trim().split('-');
            let left = s.next().unwrap();
            let right = s.next().unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .for_each(|range| {
            to_add.clear();
            for (i, &fr) in fresh.iter().enumerate() {
                match meld(range, fr) {
                    Some(v) => to_add.push((i, v)),
                    None => (),
                }
            }

            if to_add.is_empty() {
                fresh.push(range);
                return;
            }

            to_add.iter_mut().for_each(|(i, v)| {
                swap(fresh.get_mut(*i).unwrap(), v);
            });
        });
    drop(to_add);

    loop {
        let mut melded = false;
        'outie: for x in 0..fresh.len() - 1 {
            for y in x + 1..fresh.len() {
                match meld(fresh[x], fresh[y]) {
                    Some(v) => {
                        fresh.remove(y);
                        fresh.remove(x);
                        fresh.push(v);
                        melded = true;
                        break 'outie;
                    }
                    None => (),
                }
            }
        }

        if !melded {
            break;
        }
    }

    let ris: u64 = fresh.iter().map(|&(l, r)| r - l + 1).sum();
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
