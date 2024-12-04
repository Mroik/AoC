use std::fs::read_to_string;

fn extract(data: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    *data.get(y).unwrap().get(x).unwrap()
}

fn part1() {
    let data: Vec<Vec<char>> = read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();
    let mut count = 0;
    for y in 0..data.len() {
        for x in 0..data.len() {
            if extract(&data, x, y) != 'X' {
                continue;
            }
            if x < data.len() - 3
                && extract(&data, x + 1, y) == 'M'
                && extract(&data, x + 2, y) == 'A'
                && extract(&data, x + 3, y) == 'S'
            {
                count += 1;
            }
            if y < data.len() - 3
                && extract(&data, x, y + 1) == 'M'
                && extract(&data, x, y + 2) == 'A'
                && extract(&data, x, y + 3) == 'S'
            {
                count += 1;
            }
            if x > 2
                && extract(&data, x - 1, y) == 'M'
                && extract(&data, x - 2, y) == 'A'
                && extract(&data, x - 3, y) == 'S'
            {
                count += 1;
            }
            if y > 2
                && extract(&data, x, y - 1) == 'M'
                && extract(&data, x, y - 2) == 'A'
                && extract(&data, x, y - 3) == 'S'
            {
                count += 1;
            }
            if x < data.len() - 3
                && y < data.len() - 3
                && extract(&data, x + 1, y + 1) == 'M'
                && extract(&data, x + 2, y + 2) == 'A'
                && extract(&data, x + 3, y + 3) == 'S'
            {
                count += 1;
            }
            if x < data.len() - 3
                && y > 2
                && extract(&data, x + 1, y - 1) == 'M'
                && extract(&data, x + 2, y - 2) == 'A'
                && extract(&data, x + 3, y - 3) == 'S'
            {
                count += 1;
            }
            if x > 2
                && y < data.len() - 3
                && extract(&data, x - 1, y + 1) == 'M'
                && extract(&data, x - 2, y + 2) == 'A'
                && extract(&data, x - 3, y + 3) == 'S'
            {
                count += 1;
            }
            if x > 2
                && y > 2
                && extract(&data, x - 1, y - 1) == 'M'
                && extract(&data, x - 2, y - 2) == 'A'
                && extract(&data, x - 3, y - 3) == 'S'
            {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn part2() {
    let data: Vec<Vec<char>> = read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();
    let mut count = 0;
    for y in 0..data.len() {
        for x in 0..data.len() {
            if extract(&data, x, y) != 'A' {
                continue;
            }
            if x > 0
                && x < data.len() - 1
                && y > 0
                && y < data.len() - 1
                && (extract(&data, x - 1, y - 1) == 'M' && extract(&data, x + 1, y + 1) == 'S'
                    || extract(&data, x - 1, y - 1) == 'S' && extract(&data, x + 1, y + 1) == 'M')
                && (extract(&data, x - 1, y + 1) == 'M' && extract(&data, x + 1, y - 1) == 'S'
                    || extract(&data, x - 1, y + 1) == 'S' && extract(&data, x + 1, y - 1) == 'M')
            {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn main() {
    part1();
    part2();
}
