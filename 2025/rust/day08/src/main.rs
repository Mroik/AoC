use std::{collections::HashMap, fs::read_to_string};

const TO_CONSIDER: usize = 1000;

fn distance((x1, y1, z1): (u64, u64, u64), (x2, y2, z2): (u64, u64, u64)) -> f64 {
    ((x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2) + z1.abs_diff(z2).pow(2)) as f64).sqrt()
}

fn find(boxes: &mut [(usize, (u64, u64, u64))], i: usize) -> usize {
    if boxes[i].0 == i {
        return i;
    }

    let r = find(boxes, boxes[i].0);
    boxes[i].0 = r;
    r
}

fn part1() {
    let mut boxes: Vec<(usize, (u64, u64, u64))> = read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut source = line.trim().split(',').map(|v| v.parse::<u64>().unwrap());
            (
                i,
                (
                    source.next().unwrap(),
                    source.next().unwrap(),
                    source.next().unwrap(),
                ),
            )
        })
        .collect();

    let mut all_pairs = Vec::new();
    for x in 0..boxes.len() - 1 {
        for y in x + 1..boxes.len() {
            all_pairs.push((x, y));
        }
    }

    all_pairs.sort_by(|&(a, b), &(c, d)| {
        let a = boxes[a].1;
        let b = boxes[b].1;
        let c = boxes[c].1;
        let d = boxes[d].1;
        let ab = distance(a, b);
        let cd = distance(c, d);
        ab.total_cmp(&cd)
    });

    all_pairs.iter().take(TO_CONSIDER).for_each(|&(a, b)| {
        let root_a = find(&mut boxes, a);
        let root_b = find(&mut boxes, b);
        if root_a != root_b {
            boxes[root_a].0 = root_b;
        }
    });

    let mut s = HashMap::new();
    for i in 0..boxes.len() {
        let root = find(&mut boxes, i);
        match s.get_mut(&root) {
            Some(v) => *v += 1,
            None => {
                s.insert(root, 1 as u64);
            }
        }
    }
    let mut temp: Vec<u64> = s.iter().map(|(_, &v)| v).collect();
    temp.sort();

    let ris = temp.iter().rev().take(3).fold(1, |a, b| a * b);
    println!("{}", ris);
}

fn part2() {
    let mut boxes: Vec<(usize, (u64, u64, u64))> = read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut source = line.trim().split(',').map(|v| v.parse::<u64>().unwrap());
            (
                i,
                (
                    source.next().unwrap(),
                    source.next().unwrap(),
                    source.next().unwrap(),
                ),
            )
        })
        .collect();

    let mut all_pairs = Vec::new();
    for x in 0..boxes.len() - 1 {
        for y in x + 1..boxes.len() {
            all_pairs.push((x, y));
        }
    }

    all_pairs.sort_by(|&(a, b), &(c, d)| {
        let a = boxes[a].1;
        let b = boxes[b].1;
        let c = boxes[c].1;
        let d = boxes[d].1;
        let ab = distance(a, b);
        let cd = distance(c, d);
        ab.total_cmp(&cd)
    });

    let mut last = (0, 0);
    all_pairs.iter().for_each(|&(a, b)| {
        let root_a = find(&mut boxes, a);
        let root_b = find(&mut boxes, b);
        if root_a != root_b {
            boxes[root_a].0 = root_b;
            last = (a, b);
        }
    });

    let ris = boxes[last.0].1.0 * boxes[last.1].1.0;
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
