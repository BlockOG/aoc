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

    let mut been = [[false; 4]; N * N];
    calc(&grid, &mut been, (0, 0), Direction::Right);
    been.into_iter()
        .filter(|i| i[0] || i[1] || i[2] || i[3])
        .count()
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
    for i in 0..N {
        let mut been = [[false; 4]; N * N];
        calc(&grid, &mut been, (0, i), Direction::Right);
        max = max.max(
            been.into_iter()
                .filter(|i| i[0] || i[1] || i[2] || i[3])
                .count(),
        );

        let mut been = [[false; 4]; N * N];
        calc(&grid, &mut been, (N - 1, i), Direction::Left);
        max = max.max(
            been.into_iter()
                .filter(|i| i[0] || i[1] || i[2] || i[3])
                .count(),
        );
    }
    for i in 0..N {
        let mut been = [[false; 4]; N * N];
        calc(&grid, &mut been, (i, 0), Direction::Down);
        max = max.max(
            been.into_iter()
                .filter(|i| i[0] || i[1] || i[2] || i[3])
                .count(),
        );

        let mut been = [[false; 4]; N * N];
        calc(&grid, &mut been, (i, N - 1), Direction::Up);
        max = max.max(
            been.into_iter()
                .filter(|i| i[0] || i[1] || i[2] || i[3])
                .count(),
        );
    }

    max
}

fn calc(
    grid: &Vec<Vec<Tile>>,
    been: &mut [[bool; 4]; N * N],
    mut pos: (usize, usize),
    direction: Direction,
) {
    if !been[pos.1 * N + pos.0][direction as usize] {
        been[pos.1 * N + pos.0][direction as usize] = true;

        while let (Tile::None, _)
        | (Tile::Horizontal, Direction::Right | Direction::Left)
        | (Tile::Vertical, Direction::Up | Direction::Down) = (grid[pos.1][pos.0], direction)
        {
            match direction {
                Direction::Up if pos.1 > 0 => pos.1 -= 1,
                Direction::Right if pos.0 < N - 1 => pos.0 += 1,
                Direction::Down if pos.1 < N - 1 => pos.1 += 1,
                Direction::Left if pos.0 > 0 => pos.0 -= 1,
                _ => return,
            };

            been[pos.1 * N + pos.0][direction as usize] = true;
        }

        match (grid[pos.1][pos.0], direction) {
            (Tile::None, _)
            | (Tile::Horizontal, Direction::Right | Direction::Left)
            | (Tile::Vertical, Direction::Up | Direction::Down) => {
                unreachable!()
            }

            (Tile::Horizontal, Direction::Up | Direction::Down) => {
                if pos.0 > 0 {
                    calc(grid, been, (pos.0 - 1, pos.1), Direction::Left);
                }
                if pos.0 < N - 1 {
                    calc(grid, been, (pos.0 + 1, pos.1), Direction::Right);
                }
            }
            (Tile::Vertical, Direction::Right | Direction::Left) => {
                if pos.1 > 0 {
                    calc(grid, been, (pos.0, pos.1 - 1), Direction::Up);
                }
                if pos.1 < N - 1 {
                    calc(grid, been, (pos.0, pos.1 + 1), Direction::Down);
                }
            }

            (Tile::DiagonalUp, Direction::Up) | (Tile::DiagonalDown, Direction::Down) => {
                if pos.0 < N - 1 {
                    calc(grid, been, (pos.0 + 1, pos.1), Direction::Right);
                }
            }
            (Tile::DiagonalUp, Direction::Right) | (Tile::DiagonalDown, Direction::Left) => {
                if pos.1 > 0 {
                    calc(grid, been, (pos.0, pos.1 - 1), Direction::Up);
                }
            }
            (Tile::DiagonalUp, Direction::Down) | (Tile::DiagonalDown, Direction::Up) => {
                if pos.0 > 0 {
                    calc(grid, been, (pos.0 - 1, pos.1), Direction::Left);
                }
            }
            (Tile::DiagonalUp, Direction::Left) | (Tile::DiagonalDown, Direction::Right) => {
                if pos.1 < N - 1 {
                    calc(grid, been, (pos.0, pos.1 + 1), Direction::Down);
                }
            }
        };
    }
}
