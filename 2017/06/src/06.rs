use std::collections::HashSet;

use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut input: Vec<u32> = input.raw().split('\t').map(|i| i.parse_uw()).collect();
    let mut hs = HashSet::new();
    hs.insert(
        input
            .iter()
            .enumerate()
            .map(|(i, v)| *v as u128 * 250u128.pow(i as u32))
            .sum::<u128>(),
    );

    for i in 1.. {
        let (j, &v) = input.iter().enumerate().rev().max_by_key(|i| i.1).unwrap();
        input[j] = 0;

        for v in (0..input.len()).cycle().skip(j + 1).take(v as usize) {
            input[v] += 1;
        }

        let input = input
            .iter()
            .enumerate()
            .map(|(i, v)| *v as u128 * 250u128.pow(i as u32))
            .sum::<u128>();
        if hs.contains(&input) {
            return i;
        } else {
            hs.insert(input);
        }
    }

    unreachable!()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut input: Vec<u32> = input.raw().split('\t').map(|i| i.parse_uw()).collect();
    let mut hs = HashSet::new();
    hs.insert(
        input
            .iter()
            .enumerate()
            .map(|(i, v)| *v as u128 * 250u128.pow(i as u32))
            .sum::<u128>(),
    );

    let mut seen: Option<(i32, u128)> = None;
    for i in 1.. {
        let (j, &v) = input.iter().enumerate().rev().max_by_key(|i| i.1).unwrap();
        input[j] = 0;

        for v in (0..input.len()).cycle().skip(j + 1).take(v as usize) {
            input[v] += 1;
        }

        let input = input
            .iter()
            .enumerate()
            .map(|(i, v)| *v as u128 * 250u128.pow(i as u32))
            .sum::<u128>();
        if let Some(seen) = seen {
            if input == seen.1 {
                return i - seen.0;
            }
        } else if hs.contains(&input) {
            seen = Some((i, input));
        } else {
            hs.insert(input);
        }
    }

    unreachable!()
}
