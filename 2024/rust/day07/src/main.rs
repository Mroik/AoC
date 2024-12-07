use std::fs::read_to_string;

fn ff(target: u64, acc: u64, vs: &[u64]) -> bool {
    if vs.is_empty() {
        return target == acc;
    }
    let s = ff(target, acc + vs[0], &vs[1..]);
    let m = ff(target, acc * vs[0], &vs[1..]);
    return s || m;
}

fn ff2(target: u64, acc: u64, vs: &[u64]) -> bool {
    if vs.is_empty() {
        return target == acc;
    }
    let s = ff2(target, acc + vs[0], &vs[1..]);
    let m = ff2(target, acc * vs[0], &vs[1..]);
    let mut c_temp = 0;
    let mut vv = vs[0];
    while vv > 0 {
        vv /= 10;
        c_temp += 1;
    }
    c_temp = 10_u64.pow(c_temp as u32) * acc + vs[0];
    let c = ff2(target, c_temp, &vs[1..]);
    return s || m || c;
}

fn part1() {
    let ris: u64 = read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| {
            let mut it = line.split(':');
            let target = it.next().unwrap().trim().parse().unwrap();
            let nums: Vec<u64> = it
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|v| v.parse().unwrap())
                .collect();
            if ff(target, 0, &nums) {
                target
            } else {
                0
            }
        })
        .sum();
    println!("{}", ris);
}

fn part2() {
    let ris: u64 = read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| {
            let mut it = line.split(':');
            let target = it.next().unwrap().trim().parse().unwrap();
            let nums: Vec<u64> = it
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(|v| v.parse().unwrap())
                .collect();
            if ff2(target, 0, &nums) {
                target
            } else {
                0
            }
        })
        .sum();
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
