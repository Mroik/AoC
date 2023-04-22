use std::fs::read_to_string;

fn main() {
    part1();
    part2();
}

fn parse_commands(buf: String, f_sizes: &mut Vec<u32>) {
    let mut i = 0;
    let mut stack: Vec<usize> = vec![];

    for line in buf.trim().split("\n") {
        if line.starts_with("$ cd .") {
            stack.pop();
        } else if line.starts_with("$ c") {
            f_sizes.push(0);
            stack.push(i);
            i += 1;
        } else if line.chars().nth(0).unwrap().is_numeric() {
            let size: u32 = line.split_once(" ").unwrap().0.trim().parse().unwrap();
            for x in stack.as_slice() {
                f_sizes[*x] += size;
            }
        }
    }
}

fn part1() {
    let buf = read_to_string("input.txt").unwrap();
    let mut tot = 0;
    let mut f_sizes = vec![];
    parse_commands(buf, &mut f_sizes);

    for size in f_sizes.as_slice() {
        if *size <= 100000 {
            tot += size;
        }
    }
    println!("{tot}");
}

fn part2() {
    let buf = read_to_string("input.txt").unwrap();
    let mut f_sizes = vec![];
    parse_commands(buf, &mut f_sizes);
    let need_to_free = 30000000 - (70000000 - f_sizes[0]);
    let selected = f_sizes.iter().filter(|s| **s >= need_to_free).min().unwrap();
    println!("{selected}");
}
