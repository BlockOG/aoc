aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .raw()
        .as_bytes()
        .chunks(25 * 6)
        .map(|i| {
            (
                i.iter().filter(|i| i == &&b'1').count() * i.iter().filter(|i| i == &&b'2').count(),
                i.into_iter().filter(|i| i == &&b'0').count(),
            )
        })
        .min_by_key(|i| i.1)
        .unwrap()
        .0
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut res = [[2; 25]; 6];
    for i in input.raw().as_bytes().chunks(25 * 6) {
        let mut x = 0;
        let mut y = 0;
        for i in i {
            if res[y][x] == 2 {
                res[y][x] = i - b'0';
            }

            x += 1;
            if x >= 25 {
                x = 0;
                y += 1;
            }
        }
    }

    let mut pass = String::with_capacity(26 * 6);
    for i in res {
        for i in i {
            pass.push_str(["  ", "██"][[0, 1, 0][i as usize]]);
        }
        pass.push('\n');
    }

    pass
}
