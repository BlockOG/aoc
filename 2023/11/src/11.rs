aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    calc::<1>(input)
}

fn part_2(input: aoc::Input) -> impl ToString {
    calc::<999999>(input)
}

fn calc<const N: usize>(input: aoc::Input) -> isize {
    let mut galaxies_x = vec![];
    let mut galaxies_y = vec![];
    let mut rows = 0;
    for (y, i) in input.lines().enumerate() {
        let mut curr = true;
        for (x, i) in i.bytes().enumerate() {
            if i == b'#' {
                galaxies_x.push(x);
                galaxies_y.push(y + rows);
                curr = false;
            }
        }

        if curr {
            rows += N;
        }
    }

    galaxies_x.sort_unstable();

    let mut m = 1 - galaxies_x.len() as isize;
    let mut cols = 0;
    let mut last_x = galaxies_x[0];

    let mut res = 0;
    for (mut x, y) in galaxies_x.into_iter().zip(galaxies_y.into_iter()) {
        x += cols;
        if last_x + 1 < x {
            let increase = N * (x - last_x - 1);
            x += increase;
            cols += increase;
        }
        res += x as isize * m + y as isize * m;
        m += 2;
        last_x = x;
    }

    res
}
