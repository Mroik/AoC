use std::{collections::BTreeSet, fs::read_to_string};

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
    let height = map.len();
    let width = map[0].len();
    let ris: usize = (0..height)
        .map(|y| {
            (0..width)
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

                    if x != 0 && y < height - 1 && map[y + 1][x - 1] {
                        adj += 1;
                    }

                    if y != 0 && map[y - 1][x] {
                        adj += 1;
                    }

                    if y < height - 1 && map[y + 1][x] {
                        adj += 1;
                    }

                    if x < width - 1 && y != 0 && map[y - 1][x + 1] {
                        adj += 1;
                    }

                    if x < width - 1 && map[y][x + 1] {
                        adj += 1;
                    }

                    if x < width - 1 && y < height - 1 && map[y + 1][x + 1] {
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
    let height = map.len();
    let width = map[0].len();
    let mut rolls = BTreeSet::new();
    for y in 0..height {
        for x in 0..width {
            if map[y][x] {
                rolls.insert((x, y));
            }
        }
    }

    let mut ris: u64 = 0;

    while !rolls.is_empty() {
        let (x, y) = rolls.iter().next().copied().unwrap();
        rolls.remove(&(x, y));
        if !map[y][x] {
            continue;
        }

        let mut to_add = Vec::new();

        let mut adj = 0;
        if x != 0 && y != 0 && map[y - 1][x - 1] {
            to_add.push((x - 1, y - 1));
            adj += 1;
        }

        if x != 0 && map[y][x - 1] {
            to_add.push((x - 1, y));
            adj += 1;
        }

        if x != 0 && y < height - 1 && map[y + 1][x - 1] {
            to_add.push((x - 1, y + 1));
            adj += 1;
        }

        if y != 0 && map[y - 1][x] {
            to_add.push((x, y - 1));
            adj += 1;
        }

        if y < height - 1 && map[y + 1][x] {
            to_add.push((x, y + 1));
            adj += 1;
        }

        if x < width - 1 && y != 0 && map[y - 1][x + 1] {
            to_add.push((x + 1, y - 1));
            adj += 1;
        }

        if x < width - 1 && map[y][x + 1] {
            to_add.push((x + 1, y));
            adj += 1;
        }

        if x < width - 1 && y < height - 1 && map[y + 1][x + 1] {
            to_add.push((x + 1, y + 1));
            adj += 1;
        }

        if adj < 4 {
            map[y][x] = false;
            to_add.iter().for_each(|&v| {
                rolls.insert(v);
            });
            ris += 1;
            continue;
        }
    }
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
