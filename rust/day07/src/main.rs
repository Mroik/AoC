use std::fs::read_to_string;

fn main() {
    part1();
    part2();
}

fn part1() {
    let buf = read_to_string("input.txt").unwrap();
    let mut i = 0;
    let mut f_sizes: Vec<u32> = vec![];
    let mut stack: Vec<usize> = vec![];
    let mut tot = 0;

    for line in buf.trim().split("\n") {
        let ss = line.trim();
        if ss.starts_with("dir") || ss.starts_with("$ ls") {
            continue;
        }

        if ss.starts_with("$ cd ..") {
            stack.pop();
        } else if ss.starts_with("$ cd") {
            f_sizes.insert(f_sizes.len(), 0);
            stack.insert(stack.len(), i);
            i += 1;
        } else {
            let (size, _) = ss.split_once(" ").unwrap();
            for x in stack.as_slice() {
                f_sizes[*x] += size.trim().parse::<u32>().unwrap();
            }
        }
    }

    for size in f_sizes.as_slice() {
        if *size <= 100000 {
            tot += size;
        }
    }
    println!("{tot}");
}

fn part2() {
    let buf = read_to_string("input.txt").unwrap();
    let mut i = 0;
    let mut f_sizes: Vec<u32> = vec![];
    let mut stack: Vec<usize> = vec![];

    for line in buf.trim().split("\n") {
        let ss = line.trim();
        if ss.starts_with("dir") || ss.starts_with("$ ls") {
            continue;
        }

        if ss.starts_with("$ cd ..") {
            stack.pop();
        } else if ss.starts_with("$ cd") {
            f_sizes.insert(f_sizes.len(), 0);
            stack.insert(stack.len(), i);
            i += 1;
        } else {
            let (size, _) = ss.split_once(" ").unwrap();
            for x in stack.as_slice() {
                f_sizes[*x] += size.trim().parse::<u32>().unwrap();
            }
        }
    }

    let need_to_free = 30000000 - (70000000 - f_sizes[0]);
    let selected = f_sizes.iter().filter(|s| **s >= need_to_free).min().unwrap();
    println!("{selected}");
}
