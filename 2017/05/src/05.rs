use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut input: Vec<isize> = input.lines().map(|i| i.parse_uw()).collect();

    let mut steps = 0;
    let mut i = 0;
    while i < input.len() {
        input[i] += 1;
        i = i.checked_add_signed(input[i] - 1).unwrap();
        steps += 1;
    }

    steps
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut input: Vec<isize> = input.lines().map(|i| i.parse_uw()).collect();

    let mut steps = 0;
    let mut i = 0;
    while i < input.len() {
        if input[i] < 3 {
            input[i] += 1;
            i = i.checked_add_signed(input[i] - 1).unwrap();
        } else {
            input[i] -= 1;
            i = i.checked_add_signed(input[i] + 1).unwrap();
        }
        steps += 1;
    }

    steps
}
