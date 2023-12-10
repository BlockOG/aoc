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
    let mut grid = vec![vec![Tile::Ground; input[0].len()]; input.len()];
    let mut start = (0, 0);
    for (y, i) in input.lines().enumerate() {
        for (x, i) in i.bytes().enumerate() {
            if i == b'S' {
                start = (x, y);
            }

            grid[y][x] = match i {
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
            grid[curr.1 - 1][curr.0],
            Tile::BottomLeft | Tile::Vertical | Tile::BottomRight
        )
    {
        curr.1 -= 1;
        direction = Direction::Up;
    } else if curr.1 < grid.len() - 1
        && matches!(
            grid[curr.1 + 1][curr.0],
            Tile::TopLeft | Tile::Vertical | Tile::TopRight
        )
    {
        curr.1 += 1;
        direction = Direction::Down;
    } else if curr.0 > 0
        && matches!(
            grid[curr.1][curr.0 - 1],
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
        (curr, direction) = match (grid[curr.1][curr.0], direction) {
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
    let mut grid = vec![vec![Tile::Ground; input[0].len()]; input.len()];
    let mut start = (0, 0);
    for (y, i) in input.lines().enumerate() {
        for (x, i) in i.bytes().enumerate() {
            if i == b'S' {
                start = (x, y);
            }

            grid[y][x] = match i {
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
            grid[curr.1 - 1][curr.0],
            Tile::BottomLeft | Tile::Vertical | Tile::BottomRight
        )
    {
        curr.1 -= 1;
        direction = Direction::Up;
    } else if curr.1 < grid.len() - 1
        && matches!(
            grid[curr.1 + 1][curr.0],
            Tile::TopLeft | Tile::Vertical | Tile::TopRight
        )
    {
        curr.1 += 1;
        direction = Direction::Down;
    } else if curr.0 > 0
        && matches!(
            grid[curr.1][curr.0 - 1],
            Tile::BottomRight | Tile::Horizontal | Tile::TopRight
        )
    {
        curr.0 -= 1;
        direction = Direction::Left;
    } else {
        curr.0 += 1;
        direction = Direction::Right;
    }

    let mut in_loop = vec![vec![false; grid[0].len()]; grid.len()];
    in_loop[start.1][start.0] = true;
    while curr != start {
        in_loop[curr.1][curr.0] = true;
        (curr, direction) = match (grid[curr.1][curr.0], direction) {
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
    for y in 0..grid.len() {
        let mut evenodd = 0;
        for x in 0..grid[y].len() {
            if in_loop[y][x] {
                if matches!(grid[y][x], Tile::TopLeft | Tile::Vertical | Tile::TopRight) {
                    evenodd += 1;
                }
            } else {
                res += evenodd & 1;
            }
        }
    }

    res
}
