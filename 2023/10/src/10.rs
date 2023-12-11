aoc::parts!(1, 2);

const UP: u8 = 0b0001;
const DOWN: u8 = 0b0010;
const LEFT: u8 = 0b0100;
const RIGHT: u8 = 0b1000;

const CHAR_TO_PIPE: [u8; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    LEFT | RIGHT,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    DOWN | LEFT,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    DOWN | RIGHT,
    0,
    0,
    0,
    UP | LEFT,
    0,
    UP | RIGHT,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    UP | DOWN,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];

fn part_1(input: aoc::Input) -> impl ToString {
    let n = input[0].len();
    let m = input.len();

    let direction_to_offset = [0, -(n as isize), n as isize, 0, -1, 0, 0, 0, 1];

    let (grid, start, mut curr, mut direction) = parse(n, m, input);
    curr = curr.wrapping_add_signed(direction_to_offset[direction as usize]);

    let mut i = 1;
    while curr != start {
        direction = grid[curr] ^ swap_odd_even_u4(direction);
        curr = curr.wrapping_add_signed(direction_to_offset[direction as usize]);
        i += 1;
    }

    i / 2
}

fn part_2(input: aoc::Input) -> impl ToString {
    let n = input[0].len();
    let m = input.len();

    let direction_to_offset = [0, -(n as isize), n as isize, 0, -1, 0, 0, 0, 1];

    let (grid, start, mut curr, mut direction) = parse(n, m, input);
    curr = curr.wrapping_add_signed(direction_to_offset[direction as usize]);

    let mut in_loop = vec![false; n * m];
    in_loop[start] = true;
    while curr != start {
        in_loop[curr] = true;
        direction = grid[curr] ^ swap_odd_even_u4(direction);
        curr = curr.wrapping_add_signed(direction_to_offset[direction as usize]);
    }

    let mut res = 0;
    let mut evenodd = 0;
    for i in 0..grid.len() {
        if in_loop[i] {
            if grid[i] & UP != 0 {
                evenodd += 1;
            }
        } else {
            res += evenodd & 1;
        }
    }

    res
}

fn parse(n: usize, m: usize, input: aoc::Input) -> (Vec<u8>, usize, usize, u8) {
    let mut grid = vec![0; n * m];
    let mut start = 0;
    for (y, i) in input.lines().enumerate() {
        for (x, i) in i.bytes().enumerate() {
            if i == b'S' {
                start = x + y * n;
            }

            grid[x + y * n] = CHAR_TO_PIPE[i as usize];
        }
    }

    let mut direction = 0;
    if start >= n && grid[start - n] & DOWN != 0 {
        direction = UP;
        grid[start] |= direction;
    }
    if start < n * (m - 1) && grid[start + n] & UP != 0 {
        direction = DOWN;
        grid[start] |= direction;
    }
    if start > 0 && grid[start - 1] & RIGHT != 0 {
        direction = LEFT;
        grid[start] |= direction;
    }
    if start < n * m - 1 && grid[start + 1] & LEFT != 0 {
        direction = RIGHT;
        grid[start] |= direction;
    }

    (grid, start, start, direction)
}

fn swap_odd_even_u4(a: u8) -> u8 {
    ((a & 0b0101) << 1) | ((a & 0b1010) >> 1)
}
