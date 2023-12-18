aoc::parts!(1, 2);

const N: usize = 140;

fn part_1(input: aoc::Input) -> impl ToString {
    let mut symbols = [[false; N]; N];
    for (y, line) in input.lines().enumerate() {
        for (x, i) in line.bytes().enumerate() {
            if i != b'.' && !i.is_ascii_digit() {
                symbols[x][y] = true;
            }
        }
    }

    let mut sum = 0;
    for (y, line) in input.lines().enumerate() {
        let line = line.as_bytes();
        let mut x = 0;
        while x < N {
            if line[x].is_ascii_digit() {
                let mut num = 0;
                let mut has = false;
                if x > 0
                    && (y > 0 && symbols[x - 1][y - 1]
                        || symbols[x - 1][y]
                        || y < N - 1 && symbols[x - 1][y + 1])
                {
                    has = true
                }
                while x < N && line[x].is_ascii_digit() {
                    num = num * 10 + (line[x] - b'0') as i32;
                    if y > 0 && symbols[x][y - 1] || y < N - 1 && symbols[x][y + 1] {
                        has = true
                    }

                    x += 1;
                }
                if x < N
                    && (y > 0 && symbols[x][y - 1]
                        || symbols[x][y]
                        || y < N - 1 && symbols[x][y + 1])
                {
                    has = true
                }

                if has {
                    sum += num;
                }
            } else {
                x += 1;
            }
        }
    }

    sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut symbols = [[None; N]; N];
    for (y, line) in input.lines().enumerate() {
        for (x, i) in line.bytes().enumerate() {
            if i == b'*' {
                symbols[x][y] = Some(0);
            }
        }
    }

    let mut sum = 0;
    for (y, line) in input.lines().enumerate() {
        let line = line.as_bytes();
        let mut x = 0;
        while x < N {
            if line[x].is_ascii_digit() {
                let mut num = 0;
                let mut hasx = 0;
                let mut hasy = 0;
                if x > 0 {
                    if y > 0 && symbols[x - 1][y - 1].is_some() {
                        hasx = x - 1;
                        hasy = y - 1;
                    }
                    if symbols[x - 1][y].is_some() {
                        hasx = x - 1;
                        hasy = y;
                    }
                    if y < N - 1 && symbols[x - 1][y + 1].is_some() {
                        hasx = x - 1;
                        hasy = y + 1;
                    }
                }
                while x < N && line[x].is_ascii_digit() {
                    num = num * 10 + (line[x] - b'0') as i32;
                    if y > 0 && symbols[x][y - 1].is_some() {
                        hasx = x;
                        hasy = y - 1;
                    }
                    if y < N - 1 && symbols[x][y + 1].is_some() {
                        hasx = x;
                        hasy = y + 1;
                    }

                    x += 1;
                }
                if x < N {
                    if y > 0 && symbols[x][y - 1].is_some() {
                        hasx = x;
                        hasy = y - 1;
                    }
                    if symbols[x][y].is_some() {
                        hasx = x;
                        hasy = y;
                    }
                    if y < N - 1 && symbols[x][y + 1].is_some() {
                        hasx = x;
                        hasy = y + 1;
                    }
                }

                if hasx != 0 || hasy != 0 {
                    let has = symbols[hasx][hasy].as_mut().unwrap();
                    if *has > 0 {
                        sum += *has * num;
                    } else {
                        *has = num;
                    }
                }
            } else {
                x += 1;
            }
        }
    }

    sum
}
