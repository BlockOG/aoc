use aoc::Input;

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut i = 0;
    let mut s = String::new();
    loop {
        let m = md5::compute(format!("{}{}", input.raw(), i));
        if m.starts_with(&[0, 0]) && m[2] < 0x10 {
            s.push(
                [
                    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
                    'g',
                ][m[2] as usize],
            );
            if s.len() == 8 {
                return s;
            }
        }

        i += 1;
    }
}

fn part_2(input: Input) -> impl ToString {
    let mut i = 0;
    let mut s = [0u8; 8];
    loop {
        let m = md5::compute(format!("{}{}", input.raw(), i));
        if m.starts_with(&[0, 0]) && m[2] < 8 && s[m[2] as usize] == 0 {
            s[m[2] as usize] = [
                b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd',
                b'e', b'f', b'g',
            ][(m[3] >> 4) as usize];
            if s.iter().filter(|&&i| i == 0).count() == 0 {
                return s.into_iter().map(|i| i as char).collect::<String>();
            }
        }

        i += 1;
    }
}
