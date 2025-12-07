use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn follow(
    splitters: &[(usize, usize)],
    (s_x, s_y): (usize, usize),
    hit: &mut HashSet<(usize, usize)>,
) {
    match splitters
        .iter()
        .filter(|&&(x, y)| x == s_x && y > s_y)
        .next()
    {
        Some(&(x, y)) => {
            if hit.contains(&(x, y)) {
                return;
            }
            hit.insert((x, y));
            follow(splitters, (x - 1, y), hit);
            follow(splitters, (x + 1, y), hit);
        }
        None => (),
    }
}

fn part1() {
    let data = read_to_string("input").unwrap();
    let mut splitters: Vec<(usize, usize)> = data
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter(|&(_, c)| c == '^')
                .map(|(x, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect();
    splitters.sort_by(|a, b| {
        let v = a.1.cmp(&b.1);
        if v == Ordering::Equal {
            return a.0.cmp(&b.0);
        }
        v
    });
    let start = data
        .trim()
        .lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .find(|&(_, c)| c == 'S')
        .unwrap()
        .0;
    let mut hit = HashSet::new();
    follow(&splitters, (start, 0), &mut hit);
    let ris = hit.len();
    println!("{}", ris);
}

fn follow_quantum(
    splitters: &[(usize, usize)],
    (s_x, s_y): (usize, usize),
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    match splitters
        .iter()
        .filter(|&&(x, y)| x == s_x && y > s_y)
        .next()
    {
        Some(&(x, y)) => {
            if let Some(&v) = cache.get(&(x, y)) {
                return v;
            }
            let tot = follow_quantum(splitters, (x - 1, y), cache)
                + follow_quantum(splitters, (x + 1, y), cache);
            cache.insert((x, y), tot);
            return tot;
        }
        None => (),
    }
    return 1;
}

fn part2() {
    let data = read_to_string("input").unwrap();
    let mut splitters: Vec<(usize, usize)> = data
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter(|&(_, c)| c == '^')
                .map(|(x, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect();
    splitters.sort_by(|a, b| {
        let v = a.1.cmp(&b.1);
        if v == Ordering::Equal {
            return a.0.cmp(&b.0);
        }
        v
    });
    let start = data
        .trim()
        .lines()
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .find(|&(_, c)| c == 'S')
        .unwrap()
        .0;
    let mut hit = HashMap::new();
    let ris = follow_quantum(&splitters, (start, 0), &mut hit);
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
