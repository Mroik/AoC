use std::fs::read_to_string;

fn part1() {
    let data = read_to_string("input").unwrap();
    let mut buckets = vec![
        Vec::new();
        data.lines()
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .count()
    ];
    let height = data.lines().count();
    data.lines().take(height - 1).for_each(|line| {
        line.split(' ')
            .filter(|s| !s.is_empty())
            .enumerate()
            .for_each(|(i, n)| {
                let num: u64 = n.parse().unwrap();
                let v = buckets.get_mut(i).unwrap();
                v.push(num);
            });
    });

    let ris: u64 = data
        .lines()
        .skip(height - 1)
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(i, operator)| {
            if operator == "+" {
                buckets.get(i).unwrap().iter().fold(0, |a, b| a + b)
            } else {
                buckets.get(i).unwrap().iter().fold(1, |a, b| a * b)
            }
        })
        .sum();
    println!("{}", ris);
}

fn part2() {
    let data = read_to_string("input").unwrap();
    let height = data.lines().count();

    let mut max_width: Vec<(u32, char)> = data
        .lines()
        .skip(height - 1)
        .next()
        .unwrap()
        .chars()
        .rev()
        .enumerate()
        .filter(|&(_, c)| c == '*' || c == '+')
        .map(|(i, c)| (i as u32, c))
        .collect();
    max_width.reverse();
    for x in 0..max_width.len() - 1 {
        max_width[x].0 -= max_width[x + 1].0 + 1;
    }
    (*max_width.last_mut().unwrap()).0 += 1;

    // Map is mirrored
    let map: Vec<Vec<u8>> = data
        .lines()
        .take(height - 1)
        .map(|line| line.chars().map(|c| c as u8).rev().collect())
        .collect();

    let mut start = 0;
    let ris: u64 = max_width
        .iter()
        .rev()
        .map(|&(w, o)| {
            let mut tot = if o == '+' { 0 } else { 1 };
            for col in 0..w {
                let mut temp = 0;
                for row in 0..height - 1 {
                    if map[row][start + col as usize] == b' ' {
                        continue;
                    }
                    temp *= 10;
                    temp += (map[row][start + col as usize] - b'0') as u64;
                }
                if o == '+' {
                    tot += temp;
                } else {
                    tot *= temp;
                }
            }
            start += w as usize + 1;
            tot
        })
        .sum();
    println!("{}", ris);
}

fn main() {
    part1();
    part2()
}
