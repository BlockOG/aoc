use std::collections::hash_map::Entry;

use aoc::Input;
use rustc_hash::FxHashMap;

aoc::parts!(1, 2);

const VOWELS: &str = "aeiou";

fn part_1(input: Input) -> impl ToString {
    input
        .lines()
        .filter(|s| {
            let mut vowels = 0;
            let mut repeat = false;

            for i in s.as_bytes().windows(2) {
                match i {
                    b"ab" | b"cd" | b"pq" | b"xy" => return false,
                    _ => {
                        if VOWELS.contains(i[0] as char) {
                            vowels += 1;
                        }

                        if i[0] == i[1] {
                            repeat = true;
                        }
                    }
                }
            }

            if VOWELS.contains(s.chars().last().unwrap()) {
                vowels += 1;
            }

            vowels >= 3 && repeat
        })
        .count()
}

fn part_2(input: Input) -> impl ToString {
    input
        .lines()
        .filter(|s| {
            let mut cond = false;
            for i in s.as_bytes().windows(3) {
                if i[0] == i[2] {
                    cond = true;
                    break;
                }
            }
            if !cond {
                return false;
            }

            let mut hm: FxHashMap<&[u8], Vec<usize>> = FxHashMap::default();
            let mut cond = false;
            for (i, j) in s.as_bytes().windows(2).enumerate() {
                match hm.entry(j) {
                    Entry::Occupied(entry) => {
                        for &k in entry.get() {
                            if i.abs_diff(k) > 1 {
                                cond = true;
                                break;
                            }
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(vec![]);
                    }
                }

                hm.get_mut(j).unwrap().push(i);
            }

            cond
        })
        .count()
}
