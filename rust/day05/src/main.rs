use std::{fs::read_to_string, iter};

fn main() {
    part1();
    part2();
}

fn parse_stacks(stacks: &mut Vec<Vec<char>>, buf: &String) {
    for line in buf.split("\n") {
        if line.trim() == "" {
            break;
        }

        let ll: Vec<char> = line.chars().collect();
        if ll[1].is_numeric() {
            return;
        }

        for i in 0..stacks.len() {
            let cur = ll[i * 4 + 1];
            if cur != ' ' {
                stacks.get_mut(i).unwrap().insert(0, cur);
            }
        }
    }
}

fn move_items(many: i32, from: usize, to: usize, stacks: &mut Vec<Vec<char>>) {
    for _ in 0..many {
        let s_from = stacks.get_mut(from - 1).unwrap();
        let item = s_from.remove(s_from.len() - 1);
        let s_to = stacks.get_mut(to - 1).unwrap();
        s_to.insert(s_to.len(), item);
    }
}

fn move_items2(many: usize, from: usize, to: usize, stacks: &mut Vec<Vec<char>>) {
    let s_from = stacks.get_mut(from - 1).unwrap();
    let items = s_from[(s_from.len() - many)..s_from.len()].to_vec();

    for _ in 0..many {
        s_from.remove(s_from.len() - 1);
    }

    let s_to = stacks.get_mut(to - 1).unwrap();
    s_to.extend(items.iter());
}

fn part1() {
    let buf = read_to_string("input.txt").unwrap();
    let size = (buf.split_once("\n").unwrap().0.len() - 3) / 4 + 1;
    let mut stacks: Vec<Vec<char>> = iter::repeat(vec![]).take(size).collect();
    parse_stacks(&mut stacks, &buf);

    let move_list = buf.split_once("\n\n").unwrap().1;
    for line in move_list.trim().split("\n") {
        let pp: Vec<&str> = line.split(" ").collect();
        move_items(pp[1].parse().unwrap(), pp[3].parse().unwrap(), pp[5].parse().unwrap(), &mut stacks);
    }
    
    for i in 0..size {
        let cur = stacks.get(i).unwrap();
        print!("{}", cur.get(cur.len() - 1).unwrap());
    }
    println!();
}

fn part2() {
    let buf = read_to_string("input.txt").unwrap();
    let size = (buf.split_once("\n").unwrap().0.len() - 3) / 4 + 1;
    let mut stacks: Vec<Vec<char>> = iter::repeat(vec![]).take(size).collect();
    parse_stacks(&mut stacks, &buf);

    let move_list = buf.split_once("\n\n").unwrap().1;
    for line in move_list.trim().split("\n") {
        let pp: Vec<&str> = line.split(" ").collect();
        move_items2(pp[1].parse().unwrap(), pp[3].parse().unwrap(), pp[5].parse().unwrap(), &mut stacks);
    }
    
    for i in 0..size {
        let cur = stacks.get(i).unwrap();
        print!("{}", cur.get(cur.len() - 1).unwrap());
    }
    println!();
}
