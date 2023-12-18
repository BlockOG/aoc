use aoc::Parse;
use parse_display::FromStr;
use rustc_hash::{FxHashMap, FxHashSet};

aoc::parts!(1, 2);

#[derive(FromStr)]
enum Instruction {
    #[display("R{0}")]
    Right(i32),
    #[display("D{0}")]
    Down(i32),
    #[display("L{0}")]
    Left(i32),
    #[display("U{0}")]
    Up(i32),
}

impl Instruction {
    fn amount(&self) -> i32 {
        match self {
            Instruction::Right(amount)
            | Instruction::Down(amount)
            | Instruction::Left(amount)
            | Instruction::Up(amount) => *amount,
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let a = input[0]
        .split(',')
        .map(|i| i.parse_uw())
        .collect::<Vec<Instruction>>();
    let b = input[1]
        .split(',')
        .map(|i| i.parse_uw())
        .collect::<Vec<Instruction>>();

    let mut pos = (0, 0);
    let mut poss = FxHashSet::default();
    for instr in a {
        for _ in 0..instr.amount() {
            match instr {
                Instruction::Right(_) => pos.0 += 1,
                Instruction::Down(_) => pos.1 += 1,
                Instruction::Left(_) => pos.0 -= 1,
                Instruction::Up(_) => pos.1 -= 1,
            }
            poss.insert(pos);
        }
    }

    let mut pos = (0i32, 0i32);
    let mut dist = i32::MAX;
    for instr in b {
        for _ in 0..instr.amount() {
            match instr {
                Instruction::Right(_) => pos.0 += 1,
                Instruction::Down(_) => pos.1 += 1,
                Instruction::Left(_) => pos.0 -= 1,
                Instruction::Up(_) => pos.1 -= 1,
            }
            if poss.contains(&pos) {
                let curr = pos.0.abs() + pos.1.abs();
                if curr < dist {
                    dist = curr;
                }
            }
        }
    }

    dist
}

fn part_2(input: aoc::Input) -> impl ToString {
    let a = input[0]
        .split(',')
        .map(|i| i.parse_uw())
        .collect::<Vec<Instruction>>();
    let b = input[1]
        .split(',')
        .map(|i| i.parse_uw())
        .collect::<Vec<Instruction>>();

    let mut pos = (0, 0);
    let mut poss = FxHashMap::default();
    let mut step = 0;
    for instr in a {
        for _ in 0..instr.amount() {
            step += 1;
            match instr {
                Instruction::Right(_) => pos.0 += 1,
                Instruction::Down(_) => pos.1 += 1,
                Instruction::Left(_) => pos.0 -= 1,
                Instruction::Up(_) => pos.1 -= 1,
            }
            poss.entry(pos).or_insert(step);
        }
    }

    let mut pos = (0i32, 0i32);
    let mut dist = i32::MAX;
    let mut step = 0;
    for instr in b {
        for _ in 0..instr.amount() {
            step += 1;
            match instr {
                Instruction::Right(_) => pos.0 += 1,
                Instruction::Down(_) => pos.1 += 1,
                Instruction::Left(_) => pos.0 -= 1,
                Instruction::Up(_) => pos.1 -= 1,
            }
            if poss.contains_key(&pos) {
                let curr = step + poss[&pos];
                if curr < dist {
                    dist = curr;
                }
            }
        }
    }

    dist
}
