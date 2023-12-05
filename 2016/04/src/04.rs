aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .filter_map(|i| {
            let mut chars = [0; 26];
            let mut sorted: Vec<(u8, u32)> = vec![];
            let mut j = 0;

            let mut num = 0;
            let mut cs = false;
            for i in i.bytes() {
                if cs {
                    if i == b']' {
                        return Some(num);
                    }
                    if i - b'a' != sorted[j].0 {
                        return None;
                    }
                    j += 1;
                } else if i.is_ascii_digit() {
                    num = num * 10 + (i - b'0') as u32;
                } else if i == b'[' {
                    cs = true;
                    sorted = chars
                        .into_iter()
                        .enumerate()
                        .map(|i| (i.0 as u8, i.1))
                        .collect();
                    sorted.sort_by_key(|i| 24 - i.1);
                } else if i != b'-' {
                    chars[(i - b'a') as usize] += 1;
                }
            }

            unreachable!()
        })
        .sum::<u32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    for i in input.lines() {
        let mut num = 0;
        let mut s = String::new();
        for i in i.bytes() {
            if i == b'[' {
                break;
            } else if i.is_ascii_digit() {
                num = num * 10 + (i - b'0') as u32;
            } else if i == b'-' {
                s.push(' ');
            } else {
                s.push(i as char);
            }
        }

        for i in s.bytes() {
            if i == b' ' {
                print!(" ");
            } else {
                print!(
                    "{}",
                    ((((i - b'a') as u32 + num) % 26) as u8 + b'a') as char
                );
            }
        }

        println!(" {}", num);
    }
    0
}
