use std::{collections::HashSet, fs::read_to_string};

fn extract<T>(data: &Vec<Vec<T>>, x: usize, y: usize) -> &T {
    data.get(y).unwrap().get(x).unwrap()
}

fn area(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    seed_type: char,
    x: usize,
    y: usize,
) -> u64 {
    if *extract(map, x, y) != seed_type {
        return 0;
    }
    if *extract(visited, x, y) {
        return 0;
    }
    std::mem::swap(&mut true, visited.get_mut(y).unwrap().get_mut(x).unwrap());
    let mut ris = 1;
    if x > 0 {
        ris += area(map, visited, seed_type, x - 1, y);
    }
    if y > 0 {
        ris += area(map, visited, seed_type, x, y - 1);
    }
    if x < map.len() - 1 {
        ris += area(map, visited, seed_type, x + 1, y);
    }
    if y < map.len() - 1 {
        ris += area(map, visited, seed_type, x, y + 1);
    }
    return ris;
}

fn perimeter(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    seed_type: char,
    x: usize,
    y: usize,
) -> u64 {
    if *extract(map, x, y) != seed_type {
        return 0;
    }
    if *extract(visited, x, y) {
        return 0;
    }
    std::mem::swap(&mut true, visited.get_mut(y).unwrap().get_mut(x).unwrap());
    let mut ris = 0;
    if x > 0 {
        if *extract(map, x - 1, y) != seed_type {
            ris += 1;
        }
        ris += perimeter(map, visited, seed_type, x - 1, y);
    } else {
        ris += 1;
    }
    if y > 0 {
        if *extract(map, x, y - 1) != seed_type {
            ris += 1;
        }
        ris += perimeter(map, visited, seed_type, x, y - 1);
    } else {
        ris += 1;
    }
    if x < map.len() - 1 {
        if *extract(map, x + 1, y) != seed_type {
            ris += 1;
        }
        ris += perimeter(map, visited, seed_type, x + 1, y);
    } else {
        ris += 1;
    }
    if y < map.len() - 1 {
        if *extract(map, x, y + 1) != seed_type {
            ris += 1;
        }
        ris += perimeter(map, visited, seed_type, x, y + 1);
    } else {
        ris += 1;
    }
    return ris;
}

fn part1() {
    let map: Vec<Vec<char>> = read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut visited: Vec<Vec<bool>> = map
        .iter()
        .map(|line| line.iter().map(|_| false).collect())
        .collect();
    let area: Vec<Vec<u64>> = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, plot)| area(&map, &mut visited, *plot, x, y))
                .collect()
        })
        .collect();
    visited = map
        .iter()
        .map(|line| line.iter().map(|_| false).collect())
        .collect();
    let perimeter: Vec<Vec<u64>> = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, plot)| perimeter(&map, &mut visited, *plot, x, y))
                .collect()
        })
        .collect();
    let ris: u64 = area
        .iter()
        .zip(perimeter.iter())
        .map(|(area_line, perimeter_line)| {
            area_line
                .iter()
                .zip(perimeter_line.iter())
                .map(|(a, p)| a * p)
                .sum::<u64>()
        })
        .sum();
    println!("{}", ris);
}

fn coords(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    seed_type: char,
    x: usize,
    y: usize,
    ris: &mut HashSet<(i32, i32)>,
) {
    if *extract(map, x, y) != seed_type {
        return;
    }
    if *extract(visited, x, y) {
        return;
    }
    std::mem::swap(&mut true, visited.get_mut(y).unwrap().get_mut(x).unwrap());
    ris.insert((x as i32, y as i32));
    if x > 0 {
        coords(map, visited, seed_type, x - 1, y, ris);
    }
    if y > 0 {
        coords(map, visited, seed_type, x, y - 1, ris);
    }
    if x < map.len() - 1 {
        coords(map, visited, seed_type, x + 1, y, ris);
    }
    if y < map.len() - 1 {
        coords(map, visited, seed_type, x, y + 1, ris);
    }
}

fn discounted_perimeter(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    seed_type: char,
    x: usize,
    y: usize,
) -> u64 {
    let mut coor = HashSet::new();
    coords(map, visited, seed_type, x, y, &mut coor);
    let mut ris = 0;
    coor.iter().for_each(|(x, y)| {
        let (x, y) = (*x, *y);
        // Hori top side
        if (!coor.contains(&(x - 1, y)) && !coor.contains(&(x, y - 1)))
            || (coor.contains(&(x - 1, y))
                && coor.contains(&(x - 1, y - 1))
                && !coor.contains(&(x, y - 1)))
        {
            ris += 1;
        }

        // Hori bot side
        if (!coor.contains(&(x - 1, y)) && !coor.contains(&(x, y + 1)))
            || (coor.contains(&(x - 1, y))
                && coor.contains(&(x - 1, y + 1))
                && !coor.contains(&(x, y + 1)))
        {
            ris += 1;
        }

        // Vert left side
        if (!coor.contains(&(x, y - 1)) && !coor.contains(&(x - 1, y)))
            || (coor.contains(&(x, y - 1))
                && coor.contains(&(x - 1, y - 1))
                && !coor.contains(&(x - 1, y)))
        {
            ris += 1;
        }

        // Vert right side
        if (!coor.contains(&(x, y - 1)) && !coor.contains(&(x + 1, y)))
            || (coor.contains(&(x, y - 1))
                && coor.contains(&(x + 1, y - 1))
                && !coor.contains(&(x + 1, y)))
        {
            ris += 1;
        }
    });
    return ris;
}

fn part2() {
    let map: Vec<Vec<char>> = read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut visited: Vec<Vec<bool>> = map
        .iter()
        .map(|line| line.iter().map(|_| false).collect())
        .collect();
    let area: Vec<Vec<u64>> = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, plot)| area(&map, &mut visited, *plot, x, y))
                .collect()
        })
        .collect();
    visited = map
        .iter()
        .map(|line| line.iter().map(|_| false).collect())
        .collect();
    let perimeter: Vec<Vec<u64>> = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, plot)| discounted_perimeter(&map, &mut visited, *plot, x, y))
                .collect()
        })
        .collect();
    let ris: u64 = area
        .iter()
        .zip(perimeter.iter())
        .map(|(area_line, perimeter_line)| {
            area_line
                .iter()
                .zip(perimeter_line.iter())
                .map(|(a, p)| a * p)
                .sum::<u64>()
        })
        .sum();
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
