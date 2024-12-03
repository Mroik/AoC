use std::fs::read_to_string;

use regex::Regex;

fn part1() {
    let memory = read_to_string("input").unwrap();
    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let ris: i32 = reg
        .captures_iter(&memory)
        .map(|cap| &cap[1].parse::<i32>().unwrap() * &cap[2].parse::<i32>().unwrap())
        .sum();
    println!("{}", ris);
}

fn skip(data: &str) -> String {
    let mut data = data;
    let mut ris = String::new();
    let mut state = true;
    loop {
        match state {
            true => {
                if let Some(i) = data.find("don't()") {
                    ris.push_str(data.get(..i).unwrap());
                    data = data.get(i..).unwrap();
                    state = false;
                } else {
                    ris.push_str(data.get(..).unwrap());
                    break;
                }
            }
            false => {
                if let Some(i) = data.find("do()") {
                    data = data.get(i..).unwrap();
                    state = true;
                } else {
                    break;
                }
            }
        }
    }
    return ris;
}

fn part2() {
    let mut memory = read_to_string("input").unwrap();
    memory = skip(&memory);
    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let ris: i32 = reg
        .captures_iter(&memory)
        .map(|cap| &cap[1].parse::<i32>().unwrap() * &cap[2].parse::<i32>().unwrap())
        .sum();
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
