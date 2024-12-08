use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn part1() {
    let mut freq: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let map = read_to_string("input").unwrap();
    let map_size = map.trim().split('\n').count();
    map.trim().split('\n').zip(0..).for_each(|(line, y)| {
        line.trim().chars().zip(0..).for_each(|(c, x)| {
            if c != '.' {
                match freq.get_mut(&c) {
                    Some(v) => {
                        v.push((x, y));
                    }
                    None => {
                        freq.insert(c, Vec::new());
                        freq.get_mut(&c).unwrap().push((x, y));
                    }
                }
            }
        })
    });

    let mut locs: HashSet<(i32, i32)> = HashSet::new();
    freq.iter().for_each(|(_, vs)| {
        for a in 0..vs.len() - 1 {
            for b in a + 1..vs.len() {
                let (x_a, y_a) = vs.get(a).copied().unwrap();
                let (x_b, y_b) = vs.get(b).copied().unwrap();
                let m = (y_a - y_b) as f32 / (x_a - x_b) as f32;
                let q = y_a as f32 - m * x_a as f32;
                let distance = (x_a - x_b).abs() as f32;
                let left = x_a.min(x_b) as f32 - distance;
                let right = x_a.max(x_b) as f32 + distance;
                let left_y = (m * left + q).round();
                let right_y = (m * right + q).round();
                if (left as i32) >= 0
                    && (left as i32) < map_size as i32
                    && left_y as i32 >= 0
                    && (left_y as i32) < map_size as i32
                {
                    locs.insert((left as i32, left_y as i32));
                }
                if (right as i32) >= 0
                    && (right as i32) < map_size as i32
                    && right_y as i32 >= 0
                    && (right_y as i32) < map_size as i32
                {
                    locs.insert((right as i32, right_y as i32));
                }
            }
        }
    });
    println!("{}", locs.len());
}

fn part2() {
    let mut freq: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let map = read_to_string("input").unwrap();
    let map_size = map.trim().split('\n').count();
    map.trim().split('\n').zip(0..).for_each(|(line, y)| {
        line.trim().chars().zip(0..).for_each(|(c, x)| {
            if c != '.' {
                match freq.get_mut(&c) {
                    Some(v) => {
                        v.push((x, y));
                    }
                    None => {
                        freq.insert(c, Vec::new());
                        freq.get_mut(&c).unwrap().push((x, y));
                    }
                }
            }
        })
    });

    let mut locs: HashSet<(i32, i32)> = HashSet::new();
    freq.iter().for_each(|(_, vs)| {
        for a in 0..vs.len() - 1 {
            for b in a + 1..vs.len() {
                let (x_a, y_a) = vs.get(a).copied().unwrap();
                let (x_b, y_b) = vs.get(b).copied().unwrap();
                let m = (y_a - y_b) as f32 / (x_a - x_b) as f32;
                let q = y_a as f32 - m * x_a as f32;
                let distance = (x_a - x_b).abs() as f32;
                let left = x_a.min(x_b) as f32;
                let right = x_a.max(x_b) as f32;
                let mut xx = right;
                while xx < map_size as f32 {
                    let yy = (m * xx + q).round();
                    if yy as i32 >= 0 && (yy as i32) < (map_size as i32) {
                        locs.insert((xx as i32, yy as i32));
                    }
                    xx += distance;
                }
                xx = left;
                while xx >= 0_f32 {
                    let yy = (m * xx + q).round();
                    if yy as i32 >= 0 && (yy as i32) < (map_size as i32) {
                        locs.insert((xx as i32, yy as i32));
                    }
                    xx -= distance;
                }
            }
        }
    });
    println!("{}", locs.len());
}

fn main() {
    part1();
    part2();
}
