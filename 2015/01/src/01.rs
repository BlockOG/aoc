use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    input
        .raw()
        .bytes()
        .map(|c| (c - b'(') as i32 * -2 + 1)
        .sum::<i32>()
}

fn part_2(input: Input) -> impl ToString {
    let mut sum = 0;
    for (i, j) in input
        .raw()
        .bytes()
        .map(|c| (c - b'(') as i32 * -2 + 1)
        .enumerate()
    {
        sum += j;
        if sum < 0 {
            return i + 1;
        }
    }
    0
}
