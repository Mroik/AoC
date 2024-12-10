use std::{
    collections::{HashSet, LinkedList},
    fs::read_to_string,
};

fn extract(data: &Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
    data.get(y).unwrap().get(x).copied().unwrap()
}

fn find_paths(process: &mut LinkedList<(u8, usize, usize)>, map: &Vec<Vec<u8>>) -> u32 {
    for step in 0..9 {
        for _ in 0..process.iter().filter(|(v, _, _)| *v == step).count() {
            let (cur, x, y) = process.pop_front().unwrap();
            if x > 0 {
                let k = extract(map, x - 1, y);
                if k == cur + 1 {
                    process.push_back((k, x - 1, y));
                }
            }
            if y > 0 {
                let k = extract(map, x, y - 1);
                if k == cur + 1 {
                    process.push_back((k, x, y - 1));
                }
            }
            if x < map.len() - 1 {
                let k = extract(map, x + 1, y);
                if k == cur + 1 {
                    process.push_back((k, x + 1, y));
                }
            }
            if y < map.len() - 1 {
                let k = extract(map, x, y + 1);
                if k == cur + 1 {
                    process.push_back((k, x, y + 1));
                }
            }
        }
    }
    return process
        .iter()
        .copied()
        .map(|(_, x, y)| (x, y))
        .collect::<HashSet<(usize, usize)>>()
        .len() as u32;
}

fn part1() {
    let mut starts = Vec::new();
    let map: Vec<Vec<u8>> = read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, v)| {
                    if v == '0' {
                        starts.push((x, y));
                    }
                    v.to_digit(10).unwrap() as u8
                })
                .collect()
        })
        .collect();
    let ris: u32 = starts
        .iter()
        .map(|(x, y)| {
            let mut v = LinkedList::new();
            v.push_back((0, *x, *y));
            find_paths(&mut v, &map)
        })
        .sum();
    println!("{}", ris);
}

fn trail_tree(step: u8, x: usize, y: usize, map: &Vec<Vec<u8>>) -> u32 {
    let cur = extract(map, x, y);
    if step == 9 && cur == 9 {
        return 1;
    }
    if cur != step {
        return 0;
    }

    let mut ris = 0;
    if x > 0 {
        ris += trail_tree(step + 1, x - 1, y, map);
    }
    if y > 0 {
        ris += trail_tree(step + 1, x, y - 1, map);
    }
    if x < map.len() - 1 {
        ris += trail_tree(step + 1, x + 1, y, map);
    }
    if y < map.len() - 1 {
        ris += trail_tree(step + 1, x, y + 1, map);
    }
    return ris;
}

fn part2() {
    let mut starts = Vec::new();
    let map: Vec<Vec<u8>> = read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, v)| {
                    if v == '0' {
                        starts.push((x, y));
                    }
                    v.to_digit(10).unwrap() as u8
                })
                .collect()
        })
        .collect();
    let ris: u32 = starts
        .iter()
        .map(|(x, y)| trail_tree(0, *x, *y, &map))
        .sum();
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
