use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    parse(input)
        .map(|[l, w, h]| 2 * l * w + 2 * w * h + 2 * h * l + (l * w).min(w * h).min(h * l))
        .sum::<i32>()
}

fn part_2(input: Input) -> impl ToString {
    parse(input)
        .map(|[l, w, h]| (2 * (l + w)).min(2 * (w + h)).min(2 * (h + l)) + l * w * h)
        .sum::<i32>()
}

fn parse(input: Input) -> impl Iterator<Item = [i32; 3]> + '_ {
    input.lines().map(|i| i.ints())
}
