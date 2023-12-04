use aoc::Parse;

aoc::parts!(1, 2);

const VALUES: [i32; 49] = [
    1968, 2105, 2275, 2391, 2450, 5022, 5336, 5733, 6155, 6444, 6591, 13486, 14267, 15252, 16295,
    17008, 17370, 35487, 37402, 39835, 42452, 45220, 47108, 48065, 98098, 103128, 109476, 116247,
    123363, 128204, 130654, 266330, 279138, 295229, 312453, 330785, 349975, 363010, 369601, 752688,
    787032, 830037, 875851, 924406, 975079, 1009457, 1026827, 2089141, 2179400,
];

enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    #[inline]
    fn next(&mut self) {
        *self = match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        };
    }

    #[inline]
    fn step(&self, pos: &mut (i32, i32), amount: i32) {
        match self {
            Direction::Right => pos.0 += amount,
            Direction::Up => pos.1 += amount,
            Direction::Left => pos.0 -= amount,
            Direction::Down => pos.1 -= amount,
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut input = input.raw().parse_uw::<i32>() - 1;
    let mut pos = (0, 0);
    let mut step = 1;
    let mut turn = false;
    let mut direction = Direction::Right;

    loop {
        if step >= input {
            direction.step(&mut pos, input);
            break pos.0.abs() + pos.1.abs();
        } else {
            direction.step(&mut pos, step);
            input -= step;
        }

        if turn {
            step += 1;
        }
        turn = !turn;
        direction.next();
    }
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = input.raw().parse_uw::<i32>() - 1;
    VALUES[VALUES.partition_point(|&x| x < input)]
}
