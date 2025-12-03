use std::fs::read_to_string;

fn part1() {
    let ris: u32 = read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(|bank| {
            let mut max = (0, 0);
            for (i, v) in bank[..bank.len() - 1]
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .enumerate()
            {
                if v > max.1 {
                    max = (i, v);
                }
            }

            let max2 = bank[max.0 + 1..]
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .max()
                .unwrap();
            max.1 * 10 + max2
        })
        .sum();
    println!("{}", ris);
}

fn largest(data: &[u64]) -> usize {
    let mut max = (0, 0);
    for (i, v) in data.iter().copied().enumerate() {
        if v > max.1 {
            max = (i, v);
        }
    }
    max.0
}

fn part2() {
    let ris: u64 = read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(|bank| {
            let mut r = 0;
            let bank: Vec<u64> = bank
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect();
            let mut start = 0;
            let length = bank.len();
            for m in (0..12).rev() {
                let v = largest(&bank[start..length - m]);
                r += bank[start..][v] * 10_u64.pow(m as u32);
                start += v + 1;
            }
            r
        })
        .sum();
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
