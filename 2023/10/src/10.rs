aoc::parts!(1, 2);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    Ground,

    Vertical,
    Horizontal,

    TopRight,
    TopLeft,

    BottomRight,
    BottomLeft,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part_1(input: aoc::Input) -> impl ToString {
    let n = input[0].len();
    let m = input.len();

    let mut grid = vec![Tile::Ground; n * m];
    let mut start = (0, 0);
    for (y, i) in input.lines().enumerate() {
        for (x, i) in i.bytes().enumerate() {
            if i == b'S' {
                start = (x, y);
            }

            grid[x + y * n] = match i {
                b'S' => Tile::Start,
                b'.' => Tile::Ground,

                b'|' => Tile::Vertical,
                b'-' => Tile::Horizontal,

                b'L' => Tile::TopRight,
                b'J' => Tile::TopLeft,

                b'F' => Tile::BottomRight,
                b'7' => Tile::BottomLeft,

                _ => unreachable!(),
            }
        }
    }

    let mut curr = start;
    let mut direction;
    if curr.1 > 0
        && matches!(
            grid[curr.0 + (curr.1 - 1) * n],
            Tile::BottomLeft | Tile::Vertical | Tile::BottomRight
        )
    {
        curr.1 -= 1;
        direction = Direction::Up;
    } else if curr.1 < grid.len() - 1
        && matches!(
            grid[curr.0 + (curr.1 + 1) * n],
            Tile::TopLeft | Tile::Vertical | Tile::TopRight
        )
    {
        curr.1 += 1;
        direction = Direction::Down;
    } else if curr.0 > 0
        && matches!(
            grid[curr.0 - 1 + curr.1 * n],
            Tile::BottomRight | Tile::Horizontal | Tile::TopRight
        )
    {
        curr.0 -= 1;
        direction = Direction::Left;
    } else {
        curr.0 += 1;
        direction = Direction::Right;
    }

    let mut i = 1;
    while curr != start {
        (curr, direction) = match (grid[curr.0 + curr.1 * n], direction) {
            (Tile::Vertical, Direction::Up) => ((curr.0, curr.1 - 1), Direction::Up),
            (Tile::Vertical, Direction::Down) => ((curr.0, curr.1 + 1), Direction::Down),
            (Tile::Horizontal, Direction::Left) => ((curr.0 - 1, curr.1), Direction::Left),
            (Tile::Horizontal, Direction::Right) => ((curr.0 + 1, curr.1), Direction::Right),

            (Tile::TopRight, Direction::Down) => ((curr.0 + 1, curr.1), Direction::Right),
            (Tile::TopRight, Direction::Left) => ((curr.0, curr.1 - 1), Direction::Up),
            (Tile::TopLeft, Direction::Down) => ((curr.0 - 1, curr.1), Direction::Left),
            (Tile::TopLeft, Direction::Right) => ((curr.0, curr.1 - 1), Direction::Up),

            (Tile::BottomRight, Direction::Up) => ((curr.0 + 1, curr.1), Direction::Right),
            (Tile::BottomRight, Direction::Left) => ((curr.0, curr.1 + 1), Direction::Down),
            (Tile::BottomLeft, Direction::Up) => ((curr.0 - 1, curr.1), Direction::Left),
            (Tile::BottomLeft, Direction::Right) => ((curr.0, curr.1 + 1), Direction::Down),

            _ => unreachable!(),
        };

        i += 1;
    }

    i / 2
}

fn part_2(input: aoc::Input) -> impl ToString {
    let n = input[0].len();
    let m = input.len();

    let mut grid = vec![Tile::Ground; n * m];
    let mut start = (0, 0);
    for (y, i) in input.lines().enumerate() {
        for (x, i) in i.bytes().enumerate() {
            if i == b'S' {
                start = (x, y);
            }

            grid[x + y * n] = match i {
                b'S' => Tile::Start,
                b'.' => Tile::Ground,

                b'|' => Tile::Vertical,
                b'-' => Tile::Horizontal,

                b'L' => Tile::TopRight,
                b'J' => Tile::TopLeft,

                b'F' => Tile::BottomRight,
                b'7' => Tile::BottomLeft,

                _ => unreachable!(),
            }
        }
    }

    let mut curr = start;
    let mut direction;
    if curr.1 > 0
        && matches!(
            grid[curr.0 + (curr.1 - 1) * n],
            Tile::BottomLeft | Tile::Vertical | Tile::BottomRight
        )
    {
        curr.1 -= 1;
        direction = Direction::Up;
    } else if curr.1 < grid.len() - 1
        && matches!(
            grid[curr.0 + (curr.1 + 1) * n],
            Tile::TopLeft | Tile::Vertical | Tile::TopRight
        )
    {
        curr.1 += 1;
        direction = Direction::Down;
    } else if curr.0 > 0
        && matches!(
            grid[curr.0 - 1 + curr.1 * n],
            Tile::BottomRight | Tile::Horizontal | Tile::TopRight
        )
    {
        curr.0 -= 1;
        direction = Direction::Left;
    } else {
        curr.0 += 1;
        direction = Direction::Right;
    }

    let mut in_loop = vec![false; n * m];
    in_loop[start.0 + start.1 * n] = true;
    while curr != start {
        in_loop[curr.0 + curr.1 * n] = true;
        (curr, direction) = match (grid[curr.0 + curr.1 * n], direction) {
            (Tile::Vertical, Direction::Up) => ((curr.0, curr.1 - 1), Direction::Up),
            (Tile::Vertical, Direction::Down) => ((curr.0, curr.1 + 1), Direction::Down),
            (Tile::Horizontal, Direction::Left) => ((curr.0 - 1, curr.1), Direction::Left),
            (Tile::Horizontal, Direction::Right) => ((curr.0 + 1, curr.1), Direction::Right),

            (Tile::TopRight, Direction::Down) => ((curr.0 + 1, curr.1), Direction::Right),
            (Tile::TopRight, Direction::Left) => ((curr.0, curr.1 - 1), Direction::Up),
            (Tile::TopLeft, Direction::Down) => ((curr.0 - 1, curr.1), Direction::Left),
            (Tile::TopLeft, Direction::Right) => ((curr.0, curr.1 - 1), Direction::Up),

            (Tile::BottomRight, Direction::Up) => ((curr.0 + 1, curr.1), Direction::Right),
            (Tile::BottomRight, Direction::Left) => ((curr.0, curr.1 + 1), Direction::Down),
            (Tile::BottomLeft, Direction::Up) => ((curr.0 - 1, curr.1), Direction::Left),
            (Tile::BottomLeft, Direction::Right) => ((curr.0, curr.1 + 1), Direction::Down),

            _ => unreachable!(),
        };
    }

    let mut res = 0;
    for y in 0..m {
        let mut evenodd = 0;
        for x in 0..n {
            if in_loop[x + y * n] {
                if matches!(
                    grid[x + y * n],
                    Tile::TopLeft | Tile::Vertical | Tile::TopRight
                ) {
                    evenodd += 1;
                }
            } else {
                res += evenodd & 1;
            }
        }
    }

    res
}
