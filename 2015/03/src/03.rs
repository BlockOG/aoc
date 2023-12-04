use std::collections::HashSet;

use aoc::Input;

aoc::parts!(1, 2);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part_1(input: Input) -> impl ToString {
    let mut houses = HashSet::new();
    houses.insert((0, 0));
    let mut curr = (0, 0);
    for i in parse(input) {
        match i {
            Direction::Up => curr.1 += 1,
            Direction::Down => curr.1 -= 1,
            Direction::Left => curr.0 -= 1,
            Direction::Right => curr.0 += 1,
        }
        houses.insert(curr);
    }

    houses.len()
}

fn part_2(input: Input) -> impl ToString {
    let mut houses = HashSet::new();
    houses.insert((0, 0));

    let mut curr = (0, 0);
    let mut curr2 = (0, 0);
    let mut turn = false;

    for i in parse(input) {
        match (i, turn) {
            (Direction::Up, false) => {
                curr.1 += 1;
                turn = true;
            }
            (Direction::Up, true) => {
                curr2.1 += 1;
                turn = false;
            }
            (Direction::Down, false) => {
                curr.1 -= 1;
                turn = true;
            }
            (Direction::Down, true) => {
                curr2.1 -= 1;
                turn = false;
            }
            (Direction::Left, false) => {
                curr.0 -= 1;
                turn = true;
            }
            (Direction::Left, true) => {
                curr2.0 -= 1;
                turn = false;
            }
            (Direction::Right, false) => {
                curr.0 += 1;
                turn = true;
            }
            (Direction::Right, true) => {
                curr2.0 += 1;
                turn = false;
            }
        }
        if turn {
            houses.insert(curr2);
        } else {
            houses.insert(curr);
        }
    }

    houses.len()
}

fn parse(input: Input) -> impl Iterator<Item = Direction> + '_ {
    input.raw().chars().map(|i| match i {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => unreachable!(),
    })
}
