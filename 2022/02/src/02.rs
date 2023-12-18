use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    parse(input)
        .map(|(o, y)| 1 + y + ((4 + y - o) % 3) * 3)
        .sum::<u32>()
}

fn part_2(input: Input) -> impl ToString {
    parse(input)
        .map(|(o, y)| 1 + (2 + y + o) % 3 + y * 3)
        .sum::<u32>()
}

fn parse(input: Input<'_>) -> impl Iterator<Item = (u32, u32)> + '_ {
    input.lines().map(|round| {
        (
            (round.as_bytes()[0] - b'A') as u32,
            (round.as_bytes()[2] - b'X') as u32,
        )
    })
}
