use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let time = input.lines().nth(0).unwrap().uints_iter::<u32>();
    let distance = input.lines().nth(1).unwrap().uints_iter::<u32>();

    time.zip(distance)
        .map(|(time, distance)| {
            let mut beats = 0;
            for i in 0..=time {
                if i * (time - i) > distance {
                    beats += 1;
                }
            }
            beats
        })
        .product::<u32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut time = 0;
    for i in input.lines().nth(0).unwrap().bytes() {
        if i.is_ascii_digit() {
            time = time * 10 + (i - b'0') as u128;
        }
    }
    let mut distance = 0;
    for i in input.lines().nth(1).unwrap().bytes() {
        if i.is_ascii_digit() {
            distance = distance * 10 + (i - b'0') as u128;
        }
    }

    let mut start = 0;
    for i in 1..time / 2 {
        if i * (time - i) > distance {
            start = i;
            break;
        }
    }
    time - start * 2 + 1
}
