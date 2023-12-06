aoc::parts!(1, 2);

const ROOM: [&str; 3] = ["northpole", "object", "storage"];

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .filter_map(|line| {
            let mut chars = [0; 26];
            let mut sorted: Vec<(u8, u32)> = vec![];
            let mut i = 0;

            let mut num = 0;
            let mut cs = false;
            for byte in line.bytes() {
                if cs {
                    if byte == b']' {
                        return Some(num);
                    }
                    if byte - b'a' != sorted[i].0 {
                        return None;
                    }
                    i += 1;
                } else if byte.is_ascii_digit() {
                    num = num * 10 + (byte - b'0') as u32;
                } else if byte == b'[' {
                    cs = true;
                    sorted = chars
                        .into_iter()
                        .enumerate()
                        .map(|i| (i.0 as u8, i.1))
                        .collect();
                    sorted.sort_by_key(|i| 24 - i.1);
                } else if byte != b'-' {
                    chars[(byte - b'a') as usize] += 1;
                }
            }

            unreachable!()
        })
        .sum::<u32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    'outer: for line in input.lines() {
        let mut num = 0;
        let mut i = 0;
        let mut j = 0;
        for byte in line.bytes() {
            if byte == b'[' {
                break;
            } else if byte.is_ascii_digit() {
                num = num * 10 + (byte - b'0') as u32;
            } else if byte == b'-' {
                if i != ROOM[j].len() {
                    continue 'outer;
                }

                j += 1;
                i = 0;
            } else {
                i += 1;
            }
        }

        return num;
    }

    unreachable!()
}
