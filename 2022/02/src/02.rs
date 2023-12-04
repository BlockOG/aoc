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

fn parse<'a>(input: Input<'a>) -> impl Iterator<Item = (u32, u32)> + 'a {
    input.lines().map(|round| {
        (
            (round.bytes().nth(0).unwrap() - b'A') as u32,
            (round.bytes().nth(2).unwrap() - b'X') as u32,
        )
    })
}
