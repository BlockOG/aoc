use std::iter::once;

use aoc::{IterUnwrap, Parse};

aoc::parts!(1, 2);

const CHAR_TO_TYPE: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|i| {
            let [springs, continuous_groups] = i.split(" ").collect_n();

            calc(
                springs.bytes().map(|i| CHAR_TO_TYPE[i as usize]),
                continuous_groups.uints_iter().collect(),
            )
        })
        .sum::<u64>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|i| {
            let [springs, continuous_groups] = i.split(" ").collect_n();

            calc(
                springs
                    .bytes()
                    .map(|i| CHAR_TO_TYPE[i as usize])
                    .chain(once(3))
                    .cycle()
                    .take(springs.len() * 5 + 4),
                continuous_groups
                    .uints_iter()
                    .cycle()
                    .take((continuous_groups.len() + 1) / 2 * 5)
                    .collect(),
            )
        })
        .sum::<u64>()
}

fn calc(springs: impl Iterator<Item = u8>, continuous_groups: Vec<usize>) -> u64 {
    // let len = *continuous_groups.iter().max().unwrap() + 1;
    let len = 18;
    let mut curr = vec![0; (continuous_groups.len() + 1) * len];

    curr[0] = 1;

    for byte in springs {
        match byte {
            0b10 => {
                for j in 0..continuous_groups.len() {
                    for k in (0..continuous_groups[j]).rev() {
                        curr[j * len + k + 1] = curr[j * len + k];
                    }
                    curr[j * len] = 0;
                }
                curr[continuous_groups.len() * len] = 0;
            }
            0b01 => {
                for j in 0..continuous_groups.len() {
                    curr[(j + 1) * len] += curr[j * len + continuous_groups[j]];
                    for k in 1..=continuous_groups[j] {
                        curr[j * len + k] = 0;
                    }
                }
            }
            0b11 => {
                for j in (0..continuous_groups.len()).rev() {
                    curr[(j + 1) * len] += curr[j * len + continuous_groups[j]];
                    for k in (0..continuous_groups[j]).rev() {
                        curr[j * len + k + 1] = curr[j * len + k];
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    curr[continuous_groups.len() * len]
        + curr[(continuous_groups.len() - 1) * len + continuous_groups.last().unwrap()]
}
