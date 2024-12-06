use std::fs::read_to_string;

fn part1() {
    let buf = read_to_string("input").unwrap();
    let ris = buf
        .trim()
        .split('\n')
        .map(|line| {
            let mut it = line.split(' ').map(|v| v.parse::<i32>().unwrap());
            let mut prev = (it.next().unwrap(), 0);
            for x in it {
                let diff = (x - prev.0).abs();
                if diff > 3 || diff < 1 {
                    return false;
                }
                let nn = if x > prev.0 { 1 } else { -1 };
                if prev.1 == -nn {
                    return false;
                }
                prev = (x, nn);
            }
            return true;
        })
        .filter(|v| *v == true)
        .count();
    println!("{}", ris);
}

fn part2() {
    let buf = read_to_string("input").unwrap();
    let ris = buf
        .trim()
        .split('\n')
        .map(|line| {
            let it: Vec<i32> = line.split(' ').map(|v| v.parse::<i32>().unwrap()).collect();
            // This can be done in linear time but I can't be bothered
            (0..it.len())
                .map(|i| {
                    let mut ii = it.clone();
                    ii.remove(i);
                    let mut it = ii.iter();
                    let mut prev = (it.next().unwrap(), 0);
                    for x in it {
                        let diff = (x - prev.0).abs();
                        if diff > 3 || diff < 1 {
                            return false;
                        }
                        let nn = if x > prev.0 { 1 } else { -1 };
                        if prev.1 == -nn {
                            return false;
                        }
                        prev = (x, nn);
                    }
                    return true;
                })
                .any(|v| v == true)
        })
        .filter(|v| *v == true)
        .count();
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
