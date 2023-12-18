use std::collections::hash_map::Entry;

use aoc::Parse;
use rustc_hash::FxHashMap;

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
    // let mut res = [[0; 64]; 256];
    let mut res = [[0; 14]; 256];
    let mut ends = [0; 256];
    let mut hm = FxHashMap::default();

    for i in input.raw().split(',') {
        let cur = if i.idx(i.len() - 1) == b'-' {
            &i[0..i.len() - 1]
        } else {
            &i[0..i.len() - 2]
        };

        let curr = cur.bytes().fold(0, |acc, i| (acc + i) * 17);

        if i.idx(i.len() - 1) == b'-' {
            if let Some(index) = hm.remove(&cur) {
                res[curr as usize][index] = 0;
            }
        } else {
            match hm.entry(cur) {
                Entry::Occupied(entry) => {
                    res[curr as usize][*entry.get()] = i.idx(i.len() - 1) - b'0';
                }
                Entry::Vacant(entry) => {
                    entry.insert(ends[curr as usize]);
                    res[curr as usize][ends[curr as usize]] = i.idx(i.len() - 1) - b'0';
                    ends[curr as usize] += 1;
                }
            }
        }
    }

    let mut sum = 0;
    for (j, i) in res.into_iter().enumerate() {
        let mut curr = 1;
        for i in i.into_iter() {
            if i != 0 {
                sum += i as usize * curr * (j + 1);
                curr += 1;
            }
        }
    }
    sum
}
