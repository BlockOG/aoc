aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut sum = 0;
    for j in input {
        let mut i = 1;
        while i < j.len() - 1 {
            i += 1;
            sum += 1;
            match j.bytes().nth(i - 1) {
                Some(b'\\') => match j.bytes().nth(i) {
                    Some(b'x') => i += 3,
                    _ => i += 1,
                },
                _ => {}
            }
        }
    }

    input.raw().len() - (input.lines().len() - 1) - sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut sum = 0;
    for j in input {
        let mut i = 0;
        sum += 2;
        while i < j.len() {
            match j.bytes().nth(i) {
                Some(b'\\') | Some(b'"') => sum += 1,
                _ => {}
            }
            i += 1;
            sum += 1;
        }
    }

    sum - (input.raw().len() - (input.lines().len() - 1))
}
