use rustc_hash::FxHashMap;

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

    let mut nodes_indices = FxHashMap::default();
    let mut nodes = Vec::with_capacity(input.len() - 2);
    let mut curr = 0;
    let mut zzz = 0;
    for (i, node) in input.lines().skip(2).map(str::as_bytes).enumerate() {
        let name = [node[0], node[1], node[2]];
        nodes_indices.insert(name, i);
        nodes.push(([node[7], node[8], node[9]], [node[12], node[13], node[14]]));
        if name == *b"AAA" {
            curr = i;
        } else if name == *b"ZZZ" {
            zzz = i;
        }
    }
    let nodes: Vec<_> = nodes
        .into_iter()
        .map(|i| (nodes_indices[&i.0], nodes_indices[&i.1]))
        .collect();

    for (i, instruction) in instructions.into_iter().cycle().enumerate() {
        curr = match instruction {
            Direction::Left => nodes[curr].0,
            Direction::Right => nodes[curr].1,
        };
        if curr == zzz {
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

    let mut nodes_indices = FxHashMap::default();
    let mut nodes = Vec::with_capacity(input.len() - 2);
    let mut curr_nodes = vec![];
    let mut zzz = vec![false; input.len() - 2];
    for (i, node) in input.lines().skip(2).map(str::as_bytes).enumerate() {
        let name = [node[0], node[1], node[2]];
        nodes_indices.insert(name, i);
        nodes.push(([node[7], node[8], node[9]], [node[12], node[13], node[14]]));
        if name[2] == b'A' {
            curr_nodes.push(i);
        } else if name[2] == b'Z' {
            zzz[i] = true;
        }
    }
    let nodes: Vec<_> = nodes
        .into_iter()
        .map(|i| (nodes_indices[&i.0], nodes_indices[&i.1]))
        .collect();

    let mut rep_start = vec![None; curr_nodes.len()];

    for (i, instruction) in instructions.into_iter().cycle().enumerate() {
        let mut all_some = true;
        for (j, node) in curr_nodes.iter_mut().enumerate() {
            *node = match instruction {
                Direction::Left => nodes[*node].0,
                Direction::Right => nodes[*node].1,
            };

            if zzz[*node] {
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
