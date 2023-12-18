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

const N: usize = 110;

fn part_1(input: aoc::Input) -> impl ToString {
    let grid: Vec<Tile> = input
        .lines()
        .flat_map(|i| {
            i.bytes().map(|i| match i {
                b'.' => Tile::None,
                b'-' => Tile::Horizontal,
                b'|' => Tile::Vertical,
                b'/' => Tile::DiagonalUp,
                b'\\' => Tile::DiagonalDown,
                _ => unreachable!(),
            })
        })
        .collect();

    let mut been = [[false; 4]; N * N];
    calc(
        &grid,
        &mut been,
        &mut [[true; N]; 4],
        (0, 0),
        Direction::Right,
    );
    been.into_iter()
        .filter(|i| i[0] || i[1] || i[2] || i[3])
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let grid: Vec<Tile> = input
        .lines()
        .flat_map(|i| {
            i.bytes().map(|i| match i {
                b'.' => Tile::None,
                b'-' => Tile::Horizontal,
                b'|' => Tile::Vertical,
                b'/' => Tile::DiagonalUp,
                b'\\' => Tile::DiagonalDown,
                _ => unreachable!(),
            })
        })
        .collect();

    let mut start = [[true; N]; 4];

    let mut max = 0;
    for i in 0..N {
        if start[Direction::Left as usize][i] {
            let mut been = [[false; 4]; N * N];
            calc(&grid, &mut been, &mut start, (0, i), Direction::Right);
            max = max.max(
                been.into_iter()
                    .filter(|i| i[0] || i[1] || i[2] || i[3])
                    .count(),
            );
        }

        if start[Direction::Right as usize][i] {
            let mut been = [[false; 4]; N * N];
            calc(&grid, &mut been, &mut start, (N - 1, i), Direction::Left);
            max = max.max(
                been.into_iter()
                    .filter(|i| i[0] || i[1] || i[2] || i[3])
                    .count(),
            );
        }
    }
    for i in 0..N {
        if start[Direction::Up as usize][i] {
            let mut been = [[false; 4]; N * N];
            calc(&grid, &mut been, &mut start, (i, 0), Direction::Down);
            max = max.max(
                been.into_iter()
                    .filter(|i| i[0] || i[1] || i[2] || i[3])
                    .count(),
            );
        }

        if start[Direction::Down as usize][i] {
            let mut been = [[false; 4]; N * N];
            calc(&grid, &mut been, &mut start, (i, N - 1), Direction::Up);
            max = max.max(
                been.into_iter()
                    .filter(|i| i[0] || i[1] || i[2] || i[3])
                    .count(),
            );
        }
    }

    max
}

fn calc(
    grid: &Vec<Tile>,
    been: &mut [[bool; 4]; N * N],
    start: &mut [[bool; N]; 4],
    mut pos: (usize, usize),
    direction: Direction,
) {
    if !been[pos.1 * N + pos.0][direction as usize] {
        been[pos.1 * N + pos.0][direction as usize] = true;

        while let (Tile::None, _)
        | (Tile::Horizontal, Direction::Right | Direction::Left)
        | (Tile::Vertical, Direction::Up | Direction::Down) =
            (grid[pos.1 * N + pos.0], direction)
        {
            match direction {
                Direction::Up if pos.1 > 0 => pos.1 -= 1,
                Direction::Right if pos.0 < N - 1 => pos.0 += 1,
                Direction::Down if pos.1 < N - 1 => pos.1 += 1,
                Direction::Left if pos.0 > 0 => pos.0 -= 1,
                _ => {
                    start[direction as usize][match direction {
                        Direction::Right | Direction::Left => pos.1,
                        Direction::Up | Direction::Down => pos.0,
                    }] = false;
                    return;
                }
            };

            been[pos.1 * N + pos.0][direction as usize] = true;
        }

        match (grid[pos.1 * N + pos.0], direction) {
            (Tile::None, _)
            | (Tile::Horizontal, Direction::Right | Direction::Left)
            | (Tile::Vertical, Direction::Up | Direction::Down) => {
                unreachable!()
            }

            (Tile::Horizontal, Direction::Up | Direction::Down) => {
                if pos.0 > 0 {
                    calc(grid, been, start, (pos.0 - 1, pos.1), Direction::Left);
                } else {
                    start[Direction::Left as usize][pos.1] = false;
                }
                if pos.0 < N - 1 {
                    calc(grid, been, start, (pos.0 + 1, pos.1), Direction::Right);
                } else {
                    start[Direction::Right as usize][pos.1] = false;
                }
            }
            (Tile::Vertical, Direction::Right | Direction::Left) => {
                if pos.1 > 0 {
                    calc(grid, been, start, (pos.0, pos.1 - 1), Direction::Up);
                } else {
                    start[Direction::Up as usize][pos.0] = false;
                }
                if pos.1 < N - 1 {
                    calc(grid, been, start, (pos.0, pos.1 + 1), Direction::Down);
                } else {
                    start[Direction::Down as usize][pos.0] = false;
                }
            }

            (Tile::DiagonalUp, Direction::Up) | (Tile::DiagonalDown, Direction::Down) => {
                if pos.0 < N - 1 {
                    calc(grid, been, start, (pos.0 + 1, pos.1), Direction::Right);
                } else {
                    start[Direction::Right as usize][pos.1] = false;
                }
            }
            (Tile::DiagonalUp, Direction::Right) | (Tile::DiagonalDown, Direction::Left) => {
                if pos.1 > 0 {
                    calc(grid, been, start, (pos.0, pos.1 - 1), Direction::Up);
                } else {
                    start[Direction::Up as usize][pos.0] = false;
                }
            }
            (Tile::DiagonalUp, Direction::Down) | (Tile::DiagonalDown, Direction::Up) => {
                if pos.0 > 0 {
                    calc(grid, been, start, (pos.0 - 1, pos.1), Direction::Left);
                } else {
                    start[Direction::Left as usize][pos.1] = false;
                }
            }
            (Tile::DiagonalUp, Direction::Left) | (Tile::DiagonalDown, Direction::Right) => {
                if pos.1 < N - 1 {
                    calc(grid, been, start, (pos.0, pos.1 + 1), Direction::Down);
                } else {
                    start[Direction::Down as usize][pos.0] = false;
                }
            }
        };
    }
}
