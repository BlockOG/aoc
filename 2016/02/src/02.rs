aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let mut res = 0;
    let mut pos = (1, 1);
    for i in input {
        for i in i.bytes() {
            if i == b'U' {
                if 0 < pos.1 {
                    pos.1 -= 1;
                }
            } else if i == b'D' {
                if pos.1 < 3 - 1 {
                    pos.1 += 1;
                }
            } else if i == b'L' {
                if 0 < pos.0 {
                    pos.0 -= 1;
                }
            } else if pos.0 < 3 - 1 {
                pos.0 += 1;
            }
        }
        res = res * 10 + grid[pos.1][pos.0];
    }

    res
}

fn part_2(input: aoc::Input) -> impl ToString {
    let grid = [
        [0, 0, b'1', 0, 0],
        [0, b'2', b'3', b'4', 0],
        [b'5', b'6', b'7', b'8', b'9'],
        [0, b'A', b'B', b'C', 0],
        [0, 0, b'D', 0, 0],
    ];
    let mut res = String::new();
    let mut pos = (0, 2);
    for i in input {
        for i in i.bytes() {
            if i == b'U' {
                if 0 < pos.1 && grid[pos.1 - 1][pos.0] > 0 {
                    pos.1 -= 1;
                }
            } else if i == b'D' {
                if pos.1 < 5 - 1 && grid[pos.1 + 1][pos.0] > 0 {
                    pos.1 += 1;
                }
            } else if i == b'L' {
                if 0 < pos.0 && grid[pos.1][pos.0 - 1] > 0 {
                    pos.0 -= 1;
                }
            } else if pos.0 < 5 - 1 && grid[pos.1][pos.0 + 1] > 0 {
                pos.0 += 1;
            }
        }
        res.push(grid[pos.1][pos.0] as char);
    }

    res
}
