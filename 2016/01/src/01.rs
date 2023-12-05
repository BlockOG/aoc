use std::collections::HashSet;

use aoc::Parse;
use parse_display::FromStr;

aoc::parts!(1, 2);

#[derive(FromStr)]
enum Instruction {
    #[display("L{0}")]
    Left(u32),
    #[display("R{0}")]
    Right(u32),
}

impl Instruction {
    fn amount(&self) -> u32 {
        match self {
            Instruction::Left(amount) | Instruction::Right(amount) => *amount,
        }
    }
}

enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn turn(&mut self, instruction: &Instruction) {
        *self = match instruction {
            Instruction::Left(_) => match self {
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
            },
            Instruction::Right(_) => match self {
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
            },
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut pos = (0, 0);
    let mut dir = Direction::Up;
    for instruction in input.raw().split(", ").map(|i| i.parse_uw::<Instruction>()) {
        dir.turn(&instruction);
        match dir {
            Direction::Left => pos.0 -= instruction.amount() as i32,
            Direction::Up => pos.1 += instruction.amount() as i32,
            Direction::Right => pos.0 += instruction.amount() as i32,
            Direction::Down => pos.1 -= instruction.amount() as i32,
        }
    }

    pos.0.abs() + pos.1.abs()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut hs = HashSet::new();
    hs.insert((0, 0));

    let mut pos: (i32, i32) = (0, 0);
    let mut dir = Direction::Up;
    for instruction in input.raw().split(", ").map(|i| i.parse_uw::<Instruction>()) {
        dir.turn(&instruction);
        for _ in 0..instruction.amount() {
            match dir {
                Direction::Left => pos.0 -= 1,
                Direction::Up => pos.1 += 1,
                Direction::Right => pos.0 += 1,
                Direction::Down => pos.1 -= 1,
            }
            if hs.contains(&pos) {
                return pos.0.abs() + pos.1.abs();
            }
            hs.insert(pos);
        }
    }

    unreachable!()
}
