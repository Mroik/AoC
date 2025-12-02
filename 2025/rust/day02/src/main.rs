use std::fs::read_to_string;

fn part1() {
    let mut ris: u64 = 0;
    read_to_string("input")
        .unwrap()
        .trim()
        .split(',')
        .for_each(|range| {
            let d = range.trim().split('-').collect::<Vec<&str>>();
            let (start, end): (u64, u64) = (d[0].parse().unwrap(), d[1].parse().unwrap());
            for i in start..=end {
                let mut cur_len = 0;
                let mut v = i;
                if cur_len % 2 == 1 {
                    continue;
                }

                while v > 0 {
                    v /= 10;
                    cur_len += 1;
                }

                if i / 10_u64.pow(cur_len / 2) == i % 10_u64.pow(cur_len / 2) {
                    ris += i;
                }
            }
        });
    println!("{}", ris);
}

fn part2() {
    let mut ris: u64 = 0;
    read_to_string("input")
        .unwrap()
        .trim()
        .split(',')
        .for_each(|range| {
            let d = range.trim().split('-').collect::<Vec<&str>>();
            let (start, end): (u64, u64) = (d[0].parse().unwrap(), d[1].parse().unwrap());
            for asd in start..=end {
                let i = asd.to_string();
                let mut broken;
                for l in 1..i.len() {
                    broken = false;
                    if i.len() % l != 0 {
                        continue;
                    }

                    let part = &i[..l];
                    for s in (l..i.len()).step_by(l) {
                        if part != &i[s..s + l] {
                            broken = true;
                            break;
                        }
                    }
                    if !broken {
                        ris += asd;
                        break;
                    }
                }
            }
        });
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
