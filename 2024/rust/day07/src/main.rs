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
    let mut c_temp = acc.to_string();
    c_temp.push_str(vs[0].to_string().as_str());
    let c = ff2(target, c_temp.parse().unwrap(), &vs[1..]);
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
            (target, nums)
        })
        .filter(|(t, n)| ff(*t, 0, n))
        .map(|(t, _)| t)
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
            (target, nums)
        })
        .filter(|(t, n)| ff2(*t, 0, n))
        .map(|(t, _)| t)
        .sum();
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
