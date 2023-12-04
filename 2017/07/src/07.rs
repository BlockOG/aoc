use aoc_parse::{parser, prelude::*};
use std::collections::{hash_map::Entry, HashMap};

aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut pr = HashMap::new();
    for i in input.lines() {
        for v in lower_strings(i) {
            match pr.entry(v) {
                Entry::Occupied(mut entry) => *entry.get_mut() += 1,
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
        }
    }

    pr.into_iter()
        .find_map(|i| (i.1 == 1).then_some(i.0))
        .unwrap()
        .to_owned()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let p = parser!(string(lower+) " (" u32 ")" (" -> " repeat_sep(string(lower+), ", "))?);

    // let mut
    // for i in input.lines() {

    // }
    0
}

fn lower_strings(string: &str) -> Vec<&str> {
    let mut res = vec![];
    let mut start = None;
    for (i, c) in string.bytes().enumerate() {
        if c.is_ascii_lowercase() {
            if start.is_none() {
                start = Some(i);
            }
        } else {
            if let Some(start) = start.take() {
                res.push(&string[start..i]);
            }
        }
    }
    if let Some(start) = start.take() {
        res.push(&string[start..]);
    }
    res
}
