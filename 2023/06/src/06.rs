use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    input[0]
        .uints_iter::<u64>()
        .zip(input[1].uints_iter::<u64>())
        .map(|(time, distance)| calc(time, distance))
        .product::<u64>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut time = 0;
    for i in input[0].bytes() {
        if i.is_ascii_digit() {
            time = time * 10 + (i - b'0') as u64;
        }
    }
    let mut distance = 0;
    for i in input[1].bytes() {
        if i.is_ascii_digit() {
            distance = distance * 10 + (i - b'0') as u64;
        }
    }

    calc(time, distance)
}

fn calc(time: u64, distance: u64) -> u64 {
    let mut l = 1;
    let mut r = time / 2;
    while l < r {
        let mid = (l + r) / 2;
        if mid * (time - mid) > distance {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    time - l * 2 + 1
}
