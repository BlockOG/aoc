use std::collections::VecDeque;

aoc::parts!(1, 2);

const N: usize = 131;

fn part_1(input: aoc::Input) -> impl ToString {
    let mut grid = parse(input);

    let mut go_to = VecDeque::new();
    go_to.push_back((N / 2, N / 2));

    let mut even = 0;
    for i in 1..64 + 1 {
        for _ in 0..go_to.len() {
            let (x, y) = go_to.pop_front().unwrap();
            if let Some(v @ true) = grid.get_mut(y * N + x - 1) {
                *v = false;
                go_to.push_back((x - 1, y));
            }
            if let Some(v @ true) = grid.get_mut((y - 1) * N + x) {
                *v = false;
                go_to.push_back((x, y - 1));
            }
            if let Some(v @ true) = grid.get_mut(y * N + x + 1) {
                *v = false;
                go_to.push_back((x + 1, y));
            }
            if let Some(v @ true) = grid.get_mut((y + 1) * N + x) {
                *v = false;
                go_to.push_back((x, y + 1));
            }
        }
        if i & 1 == 0 {
            even += go_to.len();
        }
    }

    even
}

fn part_2(input: aoc::Input) -> impl ToString {
    const V: usize = 26501365;

    let mut grid = parse(input);

    let mut go_to = VecDeque::new();
    go_to.push_back((N / 2, N / 2));

    let mut even_center = 0;
    let mut odd_center = 0;

    let mut up = 0;
    let mut left = 0;
    let mut down = 0;
    let mut right = 0;

    let mut up_left_small = 0;
    let mut up_right_small = 0;
    let mut down_left_small = 0;
    let mut down_right_small = 0;

    let mut up_left_big = 0;
    let mut up_right_big = 0;
    let mut down_left_big = 0;
    let mut down_right_big = 0;

    let mut go_to = VecDeque::new();
    go_to.push_back((N / 2, N / 2));
    grid[N / 2 * N + N / 2] = false;
    while !go_to.is_empty() {
        for _ in 0..go_to.len() {
            let (x, y) = go_to.pop_front().unwrap();
            if x > 0 && grid[y * N + x - 1] {
                grid[y * N + x - 1] = false;
                go_to.push_back((x - 1, y));
            }
            if y > 0 && grid[(y - 1) * N + x] {
                grid[(y - 1) * N + x] = false;
                go_to.push_back((x, y - 1));
            }
            if x < N - 1 && grid[y * N + x + 1] {
                grid[y * N + x + 1] = false;
                go_to.push_back((x + 1, y));
            }
            if y < N - 1 && grid[(y + 1) * N + x] {
                grid[(y + 1) * N + x] = false;
                go_to.push_back((x, y + 1));
            }

            odd_center += (x.abs_diff(N / 2) + y.abs_diff(N / 2)) & 1;
            even_center += 1 - ((x.abs_diff(N / 2) + y.abs_diff(N / 2)) & 1);

            if x.abs_diff(N / 2) + y <= N {
                down += 1 - ((x.abs_diff(N / 2) + y) & 1);
            }
            if x + y.abs_diff(N / 2) <= N {
                right += 1 - ((x + y.abs_diff(N / 2)) & 1);
            }
            if x.abs_diff(N / 2) + N - 1 - y <= N {
                up += 1 - ((x.abs_diff(N / 2) + N - 1 - y) & 1);
            }
            if N - 1 - x + y.abs_diff(N / 2) <= N {
                left += 1 - ((N - 1 - x + y.abs_diff(N / 2)) & 1);
            }

            if N * 2 - 2 - x - y < N / 2 {
                up_left_small += 1 - ((N * 2 - 2 - x - y) & 1);
            }
            if x + N - 1 - y < N / 2 {
                up_right_small += 1 - ((x + N - 1 - y) & 1);
            }
            if N - 1 - x + y < N / 2 {
                down_left_small += 1 - ((N - 1 - x + y) & 1);
            }
            if x + y < N / 2 {
                down_right_small += 1 - ((x + y) & 1);
            }

            if N * 2 - 2 - x - y <= N + N / 2 {
                up_left_big += (N * 2 - 2 - x - y) & 1;
            }
            if x + (N - y - 1) <= N + N / 2 {
                up_right_big += (x + (N - y - 1)) & 1;
            }
            if N - 1 - x + y <= N + N / 2 {
                down_left_big += (N - 1 - x + y) & 1;
            }
            if x + y <= N + N / 2 {
                down_right_big += (x + y) & 1;
            }
        }
    }

    (up + down + left + right)
        + odd_center * ((2..V / N).step_by(2).sum::<usize>() * 4 + 1)
        + even_center * (1..V / N).step_by(2).sum::<usize>() * 4
        + (up_left_small + up_right_small + down_left_small + down_right_small) * (V / N)
        + (up_left_big + up_right_big + down_left_big + down_right_big) * (V / N - 1)
}

fn parse(input: aoc::Input) -> Vec<bool> {
    input
        .lines()
        .flat_map(|i| i.bytes().map(|j| j != b'#'))
        .collect()
}
