use aoc::Parse;

aoc::parts!(1, 2);

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

fn part_1(input: aoc::Input) -> impl ToString {
    let instructions: Vec<_> = input[0]
        .bytes()
        .map(|i| match i {
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();

    let mut nodes = [(0, 0); 26 * 26 * 26];
    for node in input.lines().skip(2) {
        nodes[(node.idx(0) - b'A') as usize * 26 * 26
            + (node.idx(1) - b'A') as usize * 26
            + (node.idx(2) - b'A') as usize] = (
            (node.idx(7) - b'A') as usize * 26 * 26
                + (node.idx(8) - b'A') as usize * 26
                + (node.idx(9) - b'A') as usize,
            (node.idx(12) - b'A') as usize * 26 * 26
                + (node.idx(13) - b'A') as usize * 26
                + (node.idx(14) - b'A') as usize,
        );
    }

    let mut node = 0;
    for (i, instruction) in instructions.into_iter().cycle().enumerate() {
        node = match instruction {
            Direction::Left => nodes[node].0,
            Direction::Right => nodes[node].1,
        };
        if node == 26 * 26 * 26 - 1 {
            return i + 1;
        }
    }

    unreachable!()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let instructions: Vec<_> = input[0]
        .bytes()
        .map(|i| match i {
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();
    let n = instructions.len();

    let mut curr_nodes = vec![];
    let mut nodes = [(0, 0); 26 * 26 * 26];
    for node in input.lines().skip(2) {
        let name = (node.idx(0) - b'A') as usize * 26 * 26
            + (node.idx(1) - b'A') as usize * 26
            + (node.idx(2) - b'A') as usize;
        if node.idx(2) == b'A' {
            curr_nodes.push(name);
        }
        nodes[name] = (
            (node.idx(7) - b'A') as usize * 26 * 26
                + (node.idx(8) - b'A') as usize * 26
                + (node.idx(9) - b'A') as usize,
            (node.idx(12) - b'A') as usize * 26 * 26
                + (node.idx(13) - b'A') as usize * 26
                + (node.idx(14) - b'A') as usize,
        );
    }

    let mut rep_start = vec![None; curr_nodes.len()];

    for (i, instruction) in instructions.into_iter().cycle().enumerate() {
        let mut all_some = true;
        for (j, node) in curr_nodes.iter_mut().enumerate() {
            *node = match instruction {
                Direction::Left => nodes[*node].0,
                Direction::Right => nodes[*node].1,
            };

            if *node % 26 == 25 {
                rep_start[j] = Some(i);
            }

            if rep_start[j].is_none() {
                all_some = false;
            }
        }

        if all_some {
            return rep_start
                .into_iter()
                .map(|i| (i.unwrap() + 1) / n)
                .product::<usize>()
                * n;
        }
    }

    unreachable!()
}
