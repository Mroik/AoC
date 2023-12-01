use std::fs::read_to_string;

fn main() {
    part1();
    part2();
}

fn convert(item: &char) -> u32 {
    if item.is_uppercase() {
        return *item as u32 - 65 + 27;
    } else {
        return *item as u32 - 97 + 1;
    }
}

fn part1() {
    let buf = read_to_string("input.txt").unwrap();
    let mut tot = 0;
    for bag in buf.trim().split("\n") {
        for item in bag[..bag.len() / 2].chars() {
            if bag[bag.len()/2..].contains(item) {
                tot += convert(&item);
                break;
            }
        }
    }
    println!("{tot}");
}

fn part2() {
    fn get_common(group: &Vec<&str>) -> char {
        for i in 0..3 {
            match group[i].chars().find(|c| group[(i + 1) % 3].contains(*c) && group[(i + 2) % 3].contains(*c)) {
                Some(c) => return c,
                None => ()
            }
        }
        unreachable!();
    }

    let buf = read_to_string("input.txt").unwrap();
    let mut tot = 0;
    let mut i = 0;
    let mut group: Vec<&str> = vec![];

    for bag in buf.trim().split("\n") {
        group.insert(0, bag);
        i += 1;
        if i == 3 {
            tot += convert(&get_common(&group));
            i = 0;
        }
    }
    println!("{tot}");
}
