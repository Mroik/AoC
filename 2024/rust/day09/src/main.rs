use std::fs::read_to_string;

#[derive(Clone)]
enum Block {
    Empty,
    Filled(u64),
}

fn part1() {
    let mut memory: Vec<Block> = Vec::new();
    let mut block_id = 0;
    read_to_string("input")
        .unwrap()
        .trim()
        .chars()
        .enumerate()
        .for_each(|(i, size)| {
            let data = if i % 2 == 1 {
                Block::Empty
            } else {
                let d = Block::Filled(block_id);
                block_id += 1;
                d
            };
            let size = size.to_digit(10).unwrap();
            memory.extend((0..size).map(|_| data.clone()));
        });
    let mut left = 0;
    let mut right = memory.len() - 1;
    while left < right {
        match (memory.get(left).unwrap(), memory.get(right).unwrap()) {
            (Block::Empty, Block::Empty) => right -= 1,
            (Block::Empty, Block::Filled(_)) => memory.swap(left, right),
            (Block::Filled(_), Block::Empty) => {
                left += 1;
                right -= 1;
            }
            (Block::Filled(_), Block::Filled(_)) => left += 1,
        }
    }

    let ris: u64 = memory
        .iter()
        .enumerate()
        .map(|(pos, block)| match block {
            Block::Empty => 0,
            Block::Filled(id) => id * pos as u64,
        })
        .sum();
    println!("{}", ris);
}

enum Block2 {
    Empty(usize),
    Filled(u64, usize),
}

fn part2() {
    let mut memory: Vec<Block2> = Vec::new();
    let mut block_id = 0;
    read_to_string("input")
        .unwrap()
        .trim()
        .chars()
        .enumerate()
        .for_each(|(i, size)| {
            let size = size.to_digit(10).unwrap() as usize;
            if size == 0 {
                return;
            }
            let data = if i % 2 == 1 {
                Block2::Empty(size)
            } else {
                let d = Block2::Filled(block_id, size);
                block_id += 1;
                d
            };
            memory.push(data);
        });

    // This could be faster with a linked list. But it's too much
    // effort for this.
    let mut cur = memory.len() - 1;
    let mut start = 0;
    loop {
        while start < cur {
            match memory.get(start).unwrap() {
                Block2::Empty(n) if *n > 0 => break,
                _ => start += 1,
            }
        }
        for i in start..cur {
            let left = memory.get(i).unwrap();
            let right = memory.get(cur).unwrap();
            let (l, r, should) = match (left, right) {
                (_, Block2::Empty(_)) => (0, 0, false),
                (Block2::Filled(_, _), _) => (0, 0, false),
                (Block2::Empty(e_n), Block2::Filled(_, f_n)) => {
                    if e_n >= f_n {
                        (*e_n, *f_n, true)
                    } else {
                        (0, 0, false)
                    }
                }
            };
            if should {
                std::mem::swap(&mut Block2::Empty(r), memory.get_mut(i).unwrap());
                memory.swap(cur, i);
                let remain = l - r;
                if remain > 0 {
                    memory.insert(i + 1, Block2::Empty(remain));
                    cur += 1;
                }
                break;
            }
        }
        if cur == 0 {
            break;
        }
        cur -= 1;
    }

    let mut base = 0;
    let ris: u64 = memory
        .iter()
        .map(|block| {
            let (r, s) = match block {
                Block2::Empty(s) => (0, *s),
                Block2::Filled(v, s) => ((base..base + s).map(|i| (i as u64) * *v).sum(), *s),
            };
            base += s;
            return r;
        })
        .sum();
    println!("{}", ris);
}

fn main() {
    part1();
    part2();
}
