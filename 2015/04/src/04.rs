use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut i = 0;
    loop {
        let m = md5::compute(format!("{}{}", input.raw(), i));
        if m.starts_with(&[0, 0]) && m[2] < 0x10 {
            return i;
        }

        i += 1;
    }
}

fn part_2(input: Input) -> impl ToString {
    let mut i = 0;
    loop {
        if md5::compute(format!("{}{}", input.raw(), i)).starts_with(&[0, 0, 0]) {
            return i;
        }

        i += 1;
    }
}
