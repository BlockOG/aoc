use std::slice;

aoc::parts!(1, 2);

struct U64s<'a> {
    s: slice::Iter<'a, u8>,
}

impl<'a> U64s<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            s: s.as_bytes().iter(),
        }
    }
}

impl<'a> Iterator for U64s<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let s = &mut self.s;
        let mut res = 0;
        while let Some(byte) = s.next() {
            if byte.is_ascii_digit() {
                res = (byte - b'0') as u64;
                break;
            }
        }

        if let Some(byte) = s.next() {
            if !byte.is_ascii_digit() {
                return Some(res);
            }
            res = res * 10 + (byte - b'0') as u64;
        } else {
            return None;
        }

        while let Some(byte) = s.next() {
            if !byte.is_ascii_digit() {
                break;
            }
            res = res * 10 + (byte - b'0') as u64;
        }
        Some(res)
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    U64s::new(input[0])
        .zip(U64s::new(input[1]))
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
