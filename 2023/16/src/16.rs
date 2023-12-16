use rustc_hash::FxHashSet;

aoc::parts!(1, 2);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
enum Tile {
    None,
    Horizontal,
    Vertical,
    DiagonalUp,
    DiagonalDown,
}

fn part_1(input: aoc::Input) -> impl ToString {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|i| {
            i.bytes()
                .map(|i| match i {
                    b'.' => Tile::None,
                    b'-' => Tile::Horizontal,
                    b'|' => Tile::Vertical,
                    b'/' => Tile::DiagonalUp,
                    b'\\' => Tile::DiagonalDown,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    calc(&grid, vec![((0, 0), Direction::Right)])
}

fn part_2(input: aoc::Input) -> impl ToString {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|i| {
            i.bytes()
                .map(|i| match i {
                    b'.' => Tile::None,
                    b'-' => Tile::Horizontal,
                    b'|' => Tile::Vertical,
                    b'/' => Tile::DiagonalUp,
                    b'\\' => Tile::DiagonalDown,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut max = 0;
    for i in 0..grid.len() {
        max = max
            .max(calc(&grid, vec![((0, i), Direction::Right)]))
            .max(calc(&grid, vec![((grid[0].len() - 1, i), Direction::Left)]));
    }
    for i in 0..grid[0].len() {
        max = max
            .max(calc(&grid, vec![((i, 0), Direction::Down)]))
            .max(calc(&grid, vec![((i, grid.len() - 1), Direction::Up)]));
    }

    max
}

fn calc(grid: &Vec<Vec<Tile>>, mut lights: Vec<((usize, usize), Direction)>) -> usize {
    let mut hs = FxHashSet::default();
    while !lights.is_empty() {
        let mut new_lights = vec![];
        for (mut pos, direction) in lights {
            if !hs.contains(&(pos, direction)) {
                hs.insert((pos, direction));
                match (grid[pos.1][pos.0], direction) {
                    (Tile::None, _)
                    | (Tile::Horizontal, Direction::Right | Direction::Left)
                    | (Tile::Vertical, Direction::Up | Direction::Down) => {
                        match direction {
                            Direction::Up if pos.1 > 0 => pos.1 -= 1,
                            Direction::Right if pos.0 < grid[0].len() - 1 => pos.0 += 1,
                            Direction::Down if pos.1 < grid.len() - 1 => pos.1 += 1,
                            Direction::Left if pos.0 > 0 => pos.0 -= 1,
                            _ => continue,
                        };
                        new_lights.push((pos, direction));
                    }

                    (Tile::Horizontal, Direction::Up | Direction::Down) => {
                        if pos.0 > 0 {
                            new_lights.push(((pos.0 - 1, pos.1), Direction::Left));
                        }
                        if pos.0 < grid[0].len() - 1 {
                            new_lights.push(((pos.0 + 1, pos.1), Direction::Right));
                        }
                    }
                    (Tile::Vertical, Direction::Right | Direction::Left) => {
                        if pos.1 > 0 {
                            new_lights.push(((pos.0, pos.1 - 1), Direction::Up));
                        }
                        if pos.1 < grid.len() - 1 {
                            new_lights.push(((pos.0, pos.1 + 1), Direction::Down));
                        }
                    }

                    (Tile::DiagonalUp, Direction::Up) => {
                        if pos.0 < grid[0].len() - 1 {
                            new_lights.push(((pos.0 + 1, pos.1), Direction::Right));
                        }
                    }
                    (Tile::DiagonalUp, Direction::Right) => {
                        if pos.1 > 0 {
                            new_lights.push(((pos.0, pos.1 - 1), Direction::Up));
                        }
                    }
                    (Tile::DiagonalUp, Direction::Down) => {
                        if pos.0 > 0 {
                            new_lights.push(((pos.0 - 1, pos.1), Direction::Left));
                        }
                    }
                    (Tile::DiagonalUp, Direction::Left) => {
                        if pos.1 < grid.len() - 1 {
                            new_lights.push(((pos.0, pos.1 + 1), Direction::Down));
                        }
                    }

                    (Tile::DiagonalDown, Direction::Up) => {
                        if pos.0 > 0 {
                            new_lights.push(((pos.0 - 1, pos.1), Direction::Left));
                        }
                    }
                    (Tile::DiagonalDown, Direction::Right) => {
                        if pos.1 < grid.len() - 1 {
                            new_lights.push(((pos.0, pos.1 + 1), Direction::Down));
                        }
                    }
                    (Tile::DiagonalDown, Direction::Down) => {
                        if pos.0 < grid[0].len() - 1 {
                            new_lights.push(((pos.0 + 1, pos.1), Direction::Right));
                        }
                    }
                    (Tile::DiagonalDown, Direction::Left) => {
                        if pos.1 > 0 {
                            new_lights.push(((pos.0, pos.1 - 1), Direction::Up));
                        }
                    }
                };
            }
        }

        lights = new_lights;
    }

    let mut hs2 = FxHashSet::default();
    for (pos, _) in hs {
        hs2.insert(pos);
    }

    hs2.len()
}
