use std::{cmp::Ordering, fs::read_to_string, mem::swap, time::Instant};

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
    unreachable!()
}

fn part1() {
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

    fresh.sort_by(|(a_l, a_r), (b_l, b_r)| {
        if a_r < b_l {
            return Ordering::Less;
        }
        if a_l > b_r {
            return Ordering::Greater;
        }
        if a_l <= b_l && a_r <= b_r {
            return Ordering::Less;
        }
        if a_l >= b_l && a_r >= b_r {
            return Ordering::Greater;
        }
        if a_l <= b_l && a_r >= b_r {
            return Ordering::Less;
        }
        if a_l >= b_l && a_r <= b_r {
            return Ordering::Greater;
        }
        Ordering::Equal
    });

    let mut start = 0;
    'outie: loop {
        for x in start..fresh.len() - 1 {
            match meld(fresh[x], fresh[x + 1]) {
                Some(v) => {
                    fresh.remove(x + 1);
                    fresh.push(v);
                    fresh.swap_remove(x);
                    start = x;
                    continue 'outie;
                }
                None => (),
            }
        }
        break;
    }

    let mut ingredients: Vec<u64> = source
        .next()
        .unwrap()
        .split('\n')
        .map(|v| v.parse().unwrap())
        .collect();

    ingredients.sort();

    let mut start = 0;
    let ris = ingredients
        .iter()
        .filter(|&&ing| {
            for (i, &(l, r)) in fresh.iter().enumerate().skip(start) {
                if ing >= l && ing <= r {
                    start = i;
                    return true;
                }
            }
            false
        })
        .count();
    println!("{}", ris);
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

    fresh.sort_by(|(a_l, a_r), (b_l, b_r)| {
        if a_r < b_l {
            return Ordering::Less;
        }
        if a_l > b_r {
            return Ordering::Greater;
        }
        if a_l <= b_l && a_r <= b_r {
            return Ordering::Less;
        }
        if a_l >= b_l && a_r >= b_r {
            return Ordering::Greater;
        }
        if a_l <= b_l && a_r >= b_r {
            return Ordering::Less;
        }
        if a_l >= b_l && a_r <= b_r {
            return Ordering::Greater;
        }
        Ordering::Equal
    });

    let mut start = 0;
    'outie: loop {
        for x in start..fresh.len() - 1 {
            match meld(fresh[x], fresh[x + 1]) {
                Some(v) => {
                    fresh.remove(x + 1);
                    fresh.push(v);
                    fresh.swap_remove(x);
                    start = x;
                    continue 'outie;
                }
                None => (),
            }
        }
        break;
    }

    let ris: u64 = fresh.iter().map(|&(l, r)| r - l + 1).sum();
    println!("{}", ris);
}

fn main() {
    let pre = Instant::now();
    part1();
    println!("Part 1: {}", pre.elapsed().as_secs_f64());
    let pre = Instant::now();
    part2();
    println!("Part 2: {}", pre.elapsed().as_secs_f64());
}
