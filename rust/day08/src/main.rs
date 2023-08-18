use std::io::stdin;

fn parse_input() -> Vec<Vec<u8>> {
    let mut ris = vec![];
    for line in stdin().lines() {
        let pr = line
            .unwrap()
            .trim()
            .chars()
            .map(|cc| cc.to_digit(10).unwrap() as u8)
            .collect();
        ris.push(pr);
    }
    return ris;
}

fn is_visible(board: &Vec<Vec<u8>>, col: usize, row: usize) -> bool {
    if col == 0 || col == board.len() - 1 || row == 0 || row == board[0].len() {
        return true;
    }

    let comp = board[col][row];
    let mut vis = true;
    for x in 0..col {
        if board[x][row] >= comp {
            vis = false;
            break;
        }
    }
    if vis {
        return true;
    }

    vis = true;
    for y in 0..row {
        if board[col][y] >= comp {
            vis = false;
            break;
        }
    }
    if vis {
        return true;
    }

    vis = true;
    for x in (col + 1)..board.len() {
        if board[x][row] >= comp {
            vis = false;
            break;
        }
    }
    if vis {
        return true;
    }

    for y in (row + 1)..board[0].len() {
        if board[col][y] >= comp {
            return false;
        }
    }
    return true;
}

fn part1(board: &Vec<Vec<u8>>) {
    let mut counter = 0;
    for x in 0..board.len() {
        for y in 0..board[0].len() {
            if is_visible(&board, x, y) {
                counter += 1;
            }
        }
    }
    println!("{counter}");
}

fn count_vec(trees: &Vec<u8>, max: u8) -> u32 {
    if trees.is_empty() {
        return 0;
    }

    let mut counter = 0;
    for x in 0..trees.len() {
        counter += 1;
        if trees[x] >= max {
            return counter;
        }
    }
    return counter;
}

fn total_view(board: &Vec<Vec<u8>>, col: usize, row: usize) -> u32 {
    let max = board[col][row];
    let mut n = vec![];
    for y in 0..row {
        n.push(board[col][y]);
    }
    n.reverse();
    let n = count_vec(&n, max);

    let mut s = vec![];
    for y in (row + 1)..board[0].len() {
        s.push(board[col][y]);
    }
    let s = count_vec(&s, max);

    let mut e = vec![];
    for x in (col + 1)..board.len() {
        e.push(board[x][row]);
    }
    let e = count_vec(&e, max);

    let mut w = vec![];
    for x in 0..col {
        w.push(board[x][row]);
    }
    w.reverse();
    let w = count_vec(&w, max);

    return n * e * s * w;
}

fn part2(board: &mut Vec<Vec<u8>>) {
    let mut max = 0;
    for x in 0..board.len() {
        for y in 0..board[x].len() {
            let tot = total_view(board, x, y);
            if tot > max {
                max = tot;
            }
        }
    }
    println!("{max}");
}

fn main() {
    let mut board = parse_input();
    part1(&board);
    part2(&mut board);
}
