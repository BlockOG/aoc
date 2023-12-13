aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    calc_n(input, calc_1)
}

fn part_2(input: aoc::Input) -> impl ToString {
    calc_n(input, calc_2)
}

fn calc_n(input: aoc::Input, calc: fn(Vec<u32>) -> Option<usize>) -> usize {
    input
        .as_lines()
        .split(|i| i.is_empty())
        .map(parse)
        .map(|(rows, cols)| calc(rows).map(|i| i * 100).or_else(|| calc(cols)).unwrap())
        .sum()
}

fn parse(input: &[&str]) -> (Vec<u32>, Vec<u32>) {
    let mut rows = Vec::with_capacity(input.len());
    let mut cols = vec![0; input[0].len()];

    for line in input.iter() {
        let mut row = 0;
        for (x, byte) in line.bytes().enumerate() {
            row *= 2;
            cols[x] *= 2;
            if byte == b'#' {
                row += 1;
                cols[x] += 1;
            }
        }

        rows.push(row);
    }

    (rows, cols)
}

fn calc_1(lines: Vec<u32>) -> Option<usize> {
    'outer: for i in 1..lines.len() {
        for j in 0..i.min(lines.len() - i) {
            if lines[i - j - 1] != lines[i + j] {
                continue 'outer;
            }
        }

        return Some(i);
    }

    None
}

fn calc_2(lines: Vec<u32>) -> Option<usize> {
    'outer: for i in 1..lines.len() {
        let mut one = false;
        for j in 0..i.min(lines.len() - i) {
            if (lines[i - j - 1] ^ lines[i + j]).count_ones() == 1 && !one {
                one = true;
            } else if lines[i - j - 1] != lines[i + j] {
                continue 'outer;
            }
        }

        if one {
            return Some(i);
        }
    }
    None
}
