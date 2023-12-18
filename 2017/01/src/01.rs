use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    solve(input, 1)
}

fn part_2(input: Input) -> impl ToString {
    solve(input, input.raw().len() / 2)
}

fn solve(input: Input, ahead: usize) -> i32 {
    let mut sum = 0;
    for i in 0..input.raw().len() {
        if input.raw().as_bytes()[i]
            == input
                .raw().as_bytes()[(i + ahead) % input.raw().len()]
        {
            sum += (input.raw().as_bytes()[i] - b'0') as i32;
        }
    }

    sum
}
