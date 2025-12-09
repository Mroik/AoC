use std::fs::read_to_string;

fn part1() {
    let coords: Vec<(u64, u64)> = read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let mut source = line.split(',');
            (
                source.next().unwrap().parse::<u64>().unwrap(),
                source.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();
    let coords_n = coords.len();

    let biggest = coords
        .iter()
        .take(coords_n - 1)
        .enumerate()
        .map(|(i, (x1, y1))| {
            coords
                .iter()
                .skip(i + 1)
                .map(|&(x2, y2)| (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    println!("{}", biggest);
}

fn part2() {
    let coords: Vec<(u64, u64)> = read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let mut source = line.split(',');
            (
                source.next().unwrap().parse::<u64>().unwrap(),
                source.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();
    let coords_n = coords.len();

    let mut perimeter = Vec::new();
    coords
        .iter()
        .zip(coords.iter().skip(1))
        .for_each(|(&(x1, y1), &(x2, y2))| {
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    perimeter.push((x1, y));
                }
            } else {
                for x in x1.min(x2)..=x1.max(x2) {
                    perimeter.push((x, y1));
                }
            }
        });

    let mut biggest = 0;
    coords
        .iter()
        .take(coords_n - 1)
        .enumerate()
        .for_each(|(i, &(x1, y1))| {
            coords.iter().skip(i + 1).for_each(|&(x2, y2)| {
                let area = (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1);
                if !perimeter.iter().any(|&(x, y)| {
                    x1.min(x2) < x && x < x1.max(x2) && y1.min(y2) < y && y < y1.max(y2)
                }) && area > biggest
                {
                    biggest = area;
                }
            });
        });

    println!("{}", biggest);
}

fn main() {
    part1();
    part2();
}
