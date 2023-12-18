use aoc::Parse;
use itertools::Itertools;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|line| {
            let (min, max) = line
                .split('\t')
                .map(|i| i.parse_uw::<i32>())
                .fold((i32::MAX, i32::MIN), |acc, i| (acc.0.min(i), acc.1.max(i)));
            max - min
        })
        .sum::<i32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|line| {
            for i in line
                .split('\t')
                .map(|i| i.parse_uw::<i32>())
                .combinations(2)
            {
                if i[0].max(i[1]) % i[1].min(i[0]) == 0 {
                    return i[0].max(i[1]) / i[1].min(i[0]);
                }
            }
            unreachable!()
        })
        .sum::<i32>()
}
