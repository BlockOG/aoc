use aoc::Parse;
use itertools::Itertools;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|i| i.uints::<3, u32>())
        .filter(|&[a, b, c]| a + b > c && a + c > b && b + c > a)
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|i| i.uints::<3, u32>())
        .tuples()
        .map(|([a1, a2, a3], [b1, b2, b3], [c1, c2, c3])| {
            (a1 + b1 > c1 && a1 + c1 > b1 && b1 + c1 > a1) as u32
                + (a2 + b2 > c2 && a2 + c2 > b2 && b2 + c2 > a2) as u32
                + (a3 + b3 > c3 && a3 + c3 > b3 && b3 + c3 > a3) as u32
        })
        .sum::<u32>()
}
