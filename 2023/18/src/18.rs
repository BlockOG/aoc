use aoc::Parse;

aoc::parts!(1, 2);

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn part_1(input: aoc::Input) -> impl ToString {
    calc(input.lines().map(|i| {
        (
            match i.idx(0) {
                b'U' => Direction::Up,
                b'R' => Direction::Right,
                b'D' => Direction::Down,
                b'L' => Direction::Left,
                _ => unreachable!(),
            },
            i.ints::<1, isize>()[0],
        )
    }))
}

fn part_2(input: aoc::Input) -> impl ToString {
    calc(input.lines().map(|i| {
        let pos = i.bytes().position(|i| i == b'#').unwrap();

        (
            match i.idx(pos + 6) {
                b'0' => Direction::Right,
                b'1' => Direction::Down,
                b'2' => Direction::Left,
                b'3' => Direction::Up,
                _ => unreachable!(),
            },
            from_hexdigit(i.idx(pos + 1)) * 16 * 16 * 16 * 16
                + from_hexdigit(i.idx(pos + 2)) * 16 * 16 * 16
                + from_hexdigit(i.idx(pos + 3)) * 16 * 16
                + from_hexdigit(i.idx(pos + 4)) * 16
                + from_hexdigit(i.idx(pos + 5)),
        )
    }))
}

fn calc(iterator: impl Iterator<Item = (Direction, isize)>) -> isize {
    let mut shoelace = 0;
    let mut last = (0, 0);
    let mut pos = (0, 0);
    let mut len = 0;
    for (dir, amount) in iterator {
        match dir {
            Direction::Up => pos.1 += amount,
            Direction::Right => pos.0 += amount,
            Direction::Down => pos.1 -= amount,
            Direction::Left => pos.0 -= amount,
        };

        shoelace += last.0 * pos.1 - last.1 * pos.0;
        last = pos;
        len += amount;
    }

    shoelace.abs() / 2 + len / 2 + 1
}

fn from_hexdigit(i: u8) -> isize {
    match i {
        b'0'..=b'9' => (i - b'0') as isize,
        b'a'..=b'f' => (i - b'a' + 10) as isize,
        _ => unreachable!(),
    }
}
