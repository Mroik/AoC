use std::{collections::VecDeque, fs::read_to_string};

fn part1() {
    let map: Vec<Vec<bool>> = read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { true } else { false })
                .collect()
        })
        .collect();
    let ris: usize = (0..map.len())
        .map(|y| {
            (0..map.len())
                .filter(|&x| {
                    if !map[y][x] {
                        return false;
                    }

                    let mut adj = 0;
                    if x != 0 && y != 0 && map[y - 1][x - 1] {
                        adj += 1;
                    }

                    if x != 0 && map[y][x - 1] {
                        adj += 1;
                    }

                    if x != 0 && y < map.len() - 1 && map[y + 1][x - 1] {
                        adj += 1;
                    }

                    if y != 0 && map[y - 1][x] {
                        adj += 1;
                    }

                    if y < map.len() - 1 && map[y + 1][x] {
                        adj += 1;
                    }

                    if x < map.len() - 1 && y != 0 && map[y - 1][x + 1] {
                        adj += 1;
                    }

                    if x < map.len() - 1 && map[y][x + 1] {
                        adj += 1;
                    }

                    if x < map.len() - 1 && y < map.len() - 1 && map[y + 1][x + 1] {
                        adj += 1;
                    }

                    adj < 4
                })
                .count()
        })
        .sum();
    println!("{}", ris);
}

fn part2() {
    let mut map: Vec<Vec<bool>> = read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { true } else { false })
                .collect()
        })
        .collect();
    let mut rolls = VecDeque::new();
    for y in 0..map.len() {
        for x in 0..map.len() {
            if map[y][x] {
                rolls.push_back((x, y));
            }
        }
    }

    let mut ris = 0;
    let mut prev;

    loop {
        prev = rolls.len();
        for _ in 0..prev {
            let (x, y) = rolls.pop_front().unwrap();
            if !map[y][x] {
                continue;
            }

            let mut adj = 0;
            if x != 0 && y != 0 && map[y - 1][x - 1] {
                adj += 1;
            }

            if x != 0 && map[y][x - 1] {
                adj += 1;
            }

            if x != 0 && y < map.len() - 1 && map[y + 1][x - 1] {
                adj += 1;
            }

            if y != 0 && map[y - 1][x] {
                adj += 1;
            }

            if y < map.len() - 1 && map[y + 1][x] {
                adj += 1;
            }

            if x < map.len() - 1 && y != 0 && map[y - 1][x + 1] {
                adj += 1;
            }

            if x < map.len() - 1 && map[y][x + 1] {
                adj += 1;
            }

            if x < map.len() - 1 && y < map.len() - 1 && map[y + 1][x + 1] {
                adj += 1;
            }

            if adj < 4 {
                map[y][x] = false;
                ris += 1;
                continue;
            }

            rolls.push_back((x, y));
        }

        if rolls.len() == prev {
            break;
        }
    }
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
