use std::{collections::HashSet, fs::read_to_string, mem};

fn extract(data: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    return data.get(y).unwrap().get(x).copied().unwrap();
}

fn part1() {
    let map: Vec<Vec<char>> = read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut current = (0, 0);
    let mut been = HashSet::new();
    // 0 up, 1 right, 2 down, 3 left
    let mut dir = 0;
    for x in 0..map.first().unwrap().len() {
        for y in 0..map.len() {
            if extract(&map, x, y) == '^' {
                current = (x, y);
                break;
            }
        }
    }

    been.insert(current);
    loop {
        let (x, y) = current;
        match dir {
            0 => {
                if y == 0 {
                    break;
                }
                if extract(&map, x, y - 1) == '#' {
                    dir = (dir + 1) % 4;
                } else {
                    current = (x, y - 1);
                }
            }
            1 => {
                if map.first().unwrap().len() == x + 1 {
                    break;
                }
                if extract(&map, x + 1, y) == '#' {
                    dir = (dir + 1) % 4;
                } else {
                    current = (x + 1, y);
                }
            }
            2 => {
                if map.len() == y + 1 {
                    break;
                }
                if extract(&map, x, y + 1) == '#' {
                    dir = (dir + 1) % 4;
                } else {
                    current = (x, y + 1);
                }
            }
            3 => {
                if x == 0 {
                    break;
                }
                if extract(&map, x - 1, y) == '#' {
                    dir = (dir + 1) % 4;
                } else {
                    current = (x - 1, y);
                }
            }
            _ => unreachable!(),
        }
        been.insert(current);
    }
    println!("{}", been.len());
}

fn simul(map: &Vec<Vec<char>>, start: (usize, usize), xp: usize, yp: usize) -> bool {
    let mut map = map.clone();
    let _ = mem::replace(map.get_mut(yp).unwrap().get_mut(xp).unwrap(), '#');
    let mut steps = 0;
    let mut dir = 0;
    let mut current = start;
    loop {
        // Apparently checking steps up to N is faster than
        // checking againts an HashSet
        steps += 1;
        if steps > map.len() * map.len() {
            return true;
        }
        let (x, y) = current;
        match dir {
            0 => {
                if y == 0 {
                    return false;
                }
                if extract(&map, x, y - 1) == '#' {
                    dir = (dir + 1) % 4;
                } else {
                    current = (x, y - 1);
                }
            }
            1 => {
                if map.first().unwrap().len() == x + 1 {
                    return false;
                }
                if extract(&map, x + 1, y) == '#' {
                    dir = (dir + 1) % 4;
                } else {
                    current = (x + 1, y);
                }
            }
            2 => {
                if map.len() == y + 1 {
                    return false;
                }
                if extract(&map, x, y + 1) == '#' {
                    dir = (dir + 1) % 4;
                } else {
                    current = (x, y + 1);
                }
            }
            3 => {
                if x == 0 {
                    return false;
                }
                if extract(&map, x - 1, y) == '#' {
                    dir = (dir + 1) % 4;
                } else {
                    current = (x - 1, y);
                }
            }
            _ => unreachable!(),
        }
    }
}

fn part2() {
    let map: Vec<Vec<char>> = read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut current = (0, 0);
    // 0 up, 1 right, 2 down, 3 left
    for x in 0..map.first().unwrap().len() {
        for y in 0..map.len() {
            if extract(&map, x, y) == '^' {
                current = (x, y);
                break;
            }
        }
    }

    let mut many = 0;
    for xp in 0..map.first().unwrap().len() {
        for yp in 0..map.len() {
            if simul(&map, current, xp, yp) {
                many += 1;
            }
        }
    }
    println!("{}", many);
}

fn main() {
    part1();
    part2();
}
