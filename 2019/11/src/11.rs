use intcode::{Intcode, State};
use rustc_hash::FxHashMap;

aoc::parts!(1, 2);

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut intcode = Intcode::from(input);
    intcode.run();
    let mut hm = FxHashMap::default();
    let mut pos = (0, 0);
    let mut dir = Direction::Up;
    while intcode.state != State::Halted {
        if hm.contains_key(&pos) {
            intcode.input(hm[&pos] as isize);
        } else {
            intcode.input(0);
        }

        let output = intcode.run();
        hm.insert(pos, output[0] != 0);
        dir = match (dir, output[1] != 0) {
            (Direction::Up, true) => Direction::Right,
            (Direction::Right, true) => Direction::Down,
            (Direction::Down, true) => Direction::Left,
            (Direction::Left, true) => Direction::Up,
            (Direction::Up, false) => Direction::Left,
            (Direction::Left, false) => Direction::Down,
            (Direction::Down, false) => Direction::Right,
            (Direction::Right, false) => Direction::Up,
        };
        match dir {
            Direction::Up => pos.1 += 1,
            Direction::Right => pos.0 += 1,
            Direction::Down => pos.1 -= 1,
            Direction::Left => pos.0 -= 1,
        };
    }

    hm.len()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut intcode = Intcode::from(input);
    intcode.run();
    let mut hm = FxHashMap::default();
    let mut pos = (0, 0);
    hm.insert(pos, true);
    let mut dir = Direction::Up;
    while intcode.state != State::Halted {
        if hm.contains_key(&pos) {
            intcode.input(hm[&pos] as isize);
        } else {
            intcode.input(0);
        }

        let output = intcode.run();
        hm.insert(pos, output[0] != 0);
        dir = match (dir, output[1] != 0) {
            (Direction::Up, true) => Direction::Right,
            (Direction::Right, true) => Direction::Down,
            (Direction::Down, true) => Direction::Left,
            (Direction::Left, true) => Direction::Up,
            (Direction::Up, false) => Direction::Left,
            (Direction::Left, false) => Direction::Down,
            (Direction::Down, false) => Direction::Right,
            (Direction::Right, false) => Direction::Up,
        };
        match dir {
            Direction::Up => pos.1 += 1,
            Direction::Right => pos.0 += 1,
            Direction::Down => pos.1 -= 1,
            Direction::Left => pos.0 -= 1,
        };
    }

    let mut res = String::with_capacity(81 * 82);
    for j in -5..0 + 1 {
        for i in 1..40 {
            if hm.contains_key(&(i, j)) {
                res.push_str(["  ", "██"][hm[&(i, j)] as usize]);
            } else {
                res.push_str("  ");
            }
        }
        res.push('\n');
    }

    res
}
