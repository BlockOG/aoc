aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let input = input.as_lines();

    let mut sum = 0;
    for i in input.split(|i| i.is_empty()) {
        let l = calc(
            &i.into_iter()
                .map(|i| i.bytes().map(|i| i == b'#').collect())
                .collect(),
            0,
            0,
            false,
        );
        sum += l.0 * 100 + l.1;
    }

    sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    let input = input.as_lines();

    let mut sum = 0;
    'outer: for i in input.split(|i| i.is_empty()) {
        let mut i: Vec<Vec<bool>> = i
            .into_iter()
            .map(|i| i.bytes().map(|i| i == b'#').collect())
            .collect();

        let z = calc(&i, 0, 0, false);

        for j in 0..i.len() {
            for k in 0..i[j].len() {
                i[j][k] = !i[j][k];

                let l = calc(&i, z.0, z.1, true);
                if l.0 != 0 || l.1 != 0 {
                    sum += l.0 * 100 + l.1;
                    continue 'outer;
                }

                i[j][k] = !i[j][k];
            }
        }
    }

    sum
}

fn calc(i: &Vec<Vec<bool>>, horiz: usize, vert: usize, two: bool) -> (usize, usize) {
    let mut first = 0;
    let mut second = 0;

    'outer: for j in 0..i[0].len() - 1 {
        for (l, j) in (0..=j).rev().enumerate() {
            if j + l * 2 + 1 >= i[0].len() {
                break;
            }
            for k in 0..i.len() {
                if i[k][j] != i[k][j + l * 2 + 1] {
                    continue 'outer;
                }
            }
        }

        if !two || j + 1 != vert {
            second = j + 1;
            break;
        }
    }

    'outer: for j in 0..i.len() - 1 {
        for (l, j) in (0..=j).rev().enumerate() {
            if j + l * 2 + 1 >= i.len() {
                break;
            }
            if i[j] != i[j + l * 2 + 1] {
                continue 'outer;
            }
        }

        if !two || j + 1 != horiz {
            first = j + 1;
            break;
        }
    }

    (first, second)
}
