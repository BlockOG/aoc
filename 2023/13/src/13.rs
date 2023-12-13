aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let input = input.as_lines();

    let mut sum = 0;
    for grid in input.split(|i| i.is_empty()) {
        let l = calc(
            &grid
                .into_iter()
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
    'outer: for grid in input.split(|i| i.is_empty()) {
        let mut grid: Vec<Vec<bool>> = grid
            .into_iter()
            .map(|i| i.bytes().map(|i| i == b'#').collect())
            .collect();

        let z = calc(&grid, 0, 0, false);

        for j in 0..grid.len() {
            for k in 0..grid[j].len() {
                grid[j][k] = !grid[j][k];

                let l = calc(&grid, z.0, z.1, true);
                if l.0 != 0 || l.1 != 0 {
                    sum += l.0 * 100 + l.1;
                    continue 'outer;
                }

                grid[j][k] = !grid[j][k];
            }
        }
    }

    sum
}

fn calc(grid: &Vec<Vec<bool>>, horiz: usize, vert: usize, two: bool) -> (usize, usize) {
    let mut first = 0;
    let mut second = 0;

    'outer: for i in 0..grid[0].len() - 1 {
        for (j, i) in (0..=i).rev().enumerate() {
            if i + j * 2 + 1 >= grid[0].len() {
                break;
            }
            for k in 0..grid.len() {
                if grid[k][i] != grid[k][i + j * 2 + 1] {
                    continue 'outer;
                }
            }
        }

        if !two || i + 1 != vert {
            second = i + 1;
            break;
        }
    }

    'outer: for i in 0..grid.len() - 1 {
        for (j, i) in (0..=i).rev().enumerate() {
            if i + j * 2 + 1 >= grid.len() {
                break;
            }
            if grid[i] != grid[i + j * 2 + 1] {
                continue 'outer;
            }
        }

        if !two || i + 1 != horiz {
            first = i + 1;
            break;
        }
    }

    (first, second)
}
