use std::fs::read_to_string;

fn main() {
    part1();
    part2();
}

fn get_mask(c: char) -> u32 {
    let r = 1 << (c as u32 - 97);
    return r;
}

fn find_marker(data: String, size: usize) -> usize {
    let mut mask: u32 = 0;
    let packet = data.chars().collect::<Vec<char>>();
    for i in 0..size {
        mask ^= get_mask(packet[i]);
    }
    for i in size..data.len() {
        mask ^= get_mask(packet[i]);
        mask ^= get_mask(packet[i - size]);
        if u32::count_ones(mask) == size as u32 {
            return i + 1;
        }
    }
    unreachable!();
}

fn part1() {
    let buf = read_to_string("input.txt").unwrap();
    let pos = find_marker(buf, 4);
    println!("{pos}");
}

fn part2() {
    let buf = read_to_string("input.txt").unwrap();
    let pos = find_marker(buf, 14);
    println!("{pos}");
}
