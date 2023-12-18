aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut sum = 0;
    for j in input {
        let mut i = 1;
        while i < j.len() - 1 {
            i += 1;
            sum += 1;
            if matches!(j.as_bytes().get(i - 1), Some(b'\\')) {
                i += if matches!(j.as_bytes().get(i), Some(b'x')) {
                    3
                } else {
                    1
                };
            }
        }
    }

    input.raw().len() - (input.lines().len() - 1) - sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut sum = input.len() * 2;
    for j in input {
        let mut i = 0;
        while i < j.len() {
            if matches!(j.as_bytes().get(i), Some(b'\\') | Some(b'"')) {
                sum += 1;
            }
            i += 1;
            sum += 1;
        }
    }

    sum - (input.raw().len() - (input.lines().len() - 1))
}
