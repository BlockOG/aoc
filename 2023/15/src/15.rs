aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut res = 0;
    let mut curr = 0;
    for i in input.raw().bytes() {
        if i == b',' {
            res += curr as u32;
            curr = 0;
        } else {
            curr += i;
            curr *= 17;
        }
    }

    res + curr as u32
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut res: Vec<Vec<(String, u8)>> = vec![vec![]; 256];
    let mut curr = 0;
    let mut cur = String::new();
    'outer: for i in input.raw().bytes() {
        match i {
            b',' => {
                curr = 0;
                cur.clear();
            }
            b'-' => {
                for j in res[curr as usize].iter().cloned().enumerate() {
                    if j.1 .0 == cur {
                        res[curr as usize].remove(j.0);
                        break;
                    }
                }
            }
            b'=' => {}
            b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                for j in res[curr as usize].iter().cloned().enumerate() {
                    if j.1 .0 == cur {
                        res[curr as usize][j.0] = (cur.clone(), i - b'0');
                        continue 'outer;
                    }
                }
                res[curr as usize].push((cur.clone(), i - b'0'));
            }
            _ => {
                cur.push(i as char);
                curr += i;
                curr *= 17;
            }
        }
    }

    let mut sum = 0;
    for (j, i) in res.into_iter().enumerate() {
        for i in i.into_iter().enumerate() {
            sum += i.1 .1 as usize * (i.0 + 1) * (j + 1);
        }
    }
    sum
}
