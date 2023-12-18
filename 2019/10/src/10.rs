use std::f32::consts::PI;

use gcd::Gcd;
use rustc_hash::FxHashSet;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let asteroids: Vec<Vec<bool>> = input
        .lines()
        .map(|i| i.bytes().map(|j| j == b'#').collect())
        .collect();

    let mut res = 0;
    for y in 0..asteroids.len() {
        for x in 0..asteroids[y].len() {
            if !asteroids[y][x] {
                continue;
            }

            let mut curr = 0;
            let mut hs = FxHashSet::default();
            for y1 in 0..asteroids.len() {
                for x1 in 0..asteroids[y].len() {
                    if y1 == y && x1 == x {
                        continue;
                    }

                    let x1 = x1 as isize - x as isize;
                    let y1 = y1 as isize - y as isize;
                    let g = x1.unsigned_abs().gcd(y1.unsigned_abs()) as isize;
                    hs.insert((x1 / g, y1 / g));
                }
            }

            for (x1, y1) in hs {
                let mut x2 = x.wrapping_add_signed(x1);
                let mut y2 = y.wrapping_add_signed(y1);

                while x2 < asteroids[y].len() && y2 < asteroids.len() {
                    if asteroids[y2][x2] {
                        curr += 1;
                        break;
                    }

                    x2 = x2.wrapping_add_signed(x1);
                    y2 = y2.wrapping_add_signed(y1);
                }
            }

            res = res.max(curr);
        }
    }

    res
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut asteroids: Vec<Vec<bool>> = input
        .lines()
        .map(|i| i.bytes().map(|j| j == b'#').collect())
        .collect();

    let mut res = 0;
    let mut pos = (0, 0);
    let mut angles = FxHashSet::default();
    for y in 0..asteroids.len() {
        for x in 0..asteroids[y].len() {
            if !asteroids[y][x] {
                continue;
            }

            let mut curr = 0;
            let mut hs = FxHashSet::default();
            for y1 in 0..asteroids.len() {
                for x1 in 0..asteroids[y].len() {
                    if y1 == y && x1 == x {
                        continue;
                    }

                    let x1 = x1 as isize - x as isize;
                    let y1 = y1 as isize - y as isize;
                    let g = x1.unsigned_abs().gcd(y1.unsigned_abs()) as isize;
                    hs.insert((x1 / g, y1 / g));
                }
            }

            for (x1, y1) in hs.iter() {
                let mut x2 = x.wrapping_add_signed(*x1);
                let mut y2 = y.wrapping_add_signed(*y1);

                while x2 < asteroids[y].len() && y2 < asteroids.len() {
                    if asteroids[y2][x2] {
                        curr += 1;
                        break;
                    }

                    x2 = x2.wrapping_add_signed(*x1);
                    y2 = y2.wrapping_add_signed(*y1);
                }
            }

            if curr > res {
                res = curr;
                pos = (x, y);
                angles = hs;
            }
        }
    }

    let mut angles: Vec<_> = angles.into_iter().collect();
    angles.sort_by_key(|(x, y)| {
        (((*x as f32).atan2(-y as f32).rem_euclid(2.0 * PI)) * 180.0) as isize
    });

    let mut i = 1;
    for (x1, y1) in angles.into_iter().cycle() {
        let mut x2 = pos.0.wrapping_add_signed(x1);
        let mut y2 = pos.1.wrapping_add_signed(y1);

        while x2 < asteroids[pos.1].len() && y2 < asteroids.len() {
            if asteroids[y2][x2] {
                if i == 200 {
                    return x2 * 100 + y2;
                }
                i += 1;
                asteroids[y2][x2] = false;
                break;
            }

            x2 = x2.wrapping_add_signed(x1);
            y2 = y2.wrapping_add_signed(y1);
        }
    }

    unreachable!()
}
