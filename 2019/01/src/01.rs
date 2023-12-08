use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|i| i.parse_uw::<u32>() / 3 - 2)
        .sum::<u32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|i| {
            let mut i = i.parse_uw::<i32>() / 3 - 2;
            let mut sum = i;
            while i > 0 {
                i = i / 3 - 2;
                sum += i.max(0);
            }
            sum
        })
        .sum::<i32>()
}
