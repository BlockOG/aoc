use std::{collections::hash_map::Entry, mem};

use aoc::{IterUnwrap, Parse};
use parse_display::FromStr;
use rustc_hash::FxHashMap;

aoc::parts!(1, 2);

#[derive(FromStr, Default)]
#[from_str(regex = r"(?<name>[a-z]+) \((?<weight>\d+)\)( -> (?<disk>[a-z]+(, [a-z]+)*))?")]
#[from_str(new = Self::new(name, weight, disk))]
struct ParsedProgram {
    name: String,
    weight: u32,
    disk: Vec<String>,
}

impl ParsedProgram {
    fn new(name: String, weight: u32, disk: String) -> Self {
        Self {
            name,
            weight,
            disk: if disk.is_empty() {
                vec![]
            } else {
                disk.split(", ").map(|i| i.to_owned()).collect()
            },
        }
    }
}

struct Program {
    weight: u32,
    total_weight: u32,
    disk: Vec<Program>,
}

impl Program {
    fn new(weight: u32, disk: Vec<Program>) -> Self {
        Self {
            weight,
            total_weight: weight + disk.iter().map(|i| i.total_weight).sum::<u32>(),
            disk,
        }
    }

    fn recurse(&self, value: u32) -> u32 {
        if self.disk.len() == 1 {
            self.disk[0].recurse(0);
        }

        let mut first_value: Option<(u32, bool, &Program)> = None;
        let mut second_value: Option<(u32, &Program)> = None;
        for i in self.disk.iter() {
            match first_value {
                Some((value, ref mut multiple, _)) => {
                    if i.total_weight == value {
                        if let Some((_, program)) = second_value {
                            return program.recurse(value);
                        }
                        *multiple = true;
                    } else {
                        match second_value {
                            Some((value, _)) => {
                                if let Some((_, _, program)) = first_value {
                                    return program.recurse(value);
                                }
                            }
                            None => {
                                if *multiple {
                                    return i.recurse(value);
                                }
                                second_value = Some((i.total_weight, i))
                            }
                        }
                    }
                }
                None => {
                    first_value = Some((i.total_weight, false, i));
                }
            }
        }

        value - (self.total_weight - self.weight)
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut pr = FxHashMap::default();
    for i in input {
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
    let mut programs: Vec<ParsedProgram> = input.lines().map(|i| i.parse_uw()).collect();

    let mut hm = FxHashMap::default();
    while !programs.is_empty() {
        programs.retain_mut(|program| {
            for i in program.disk.iter() {
                if !hm.contains_key(i) {
                    return true;
                }
            }

            let ParsedProgram { name, weight, disk } = mem::take(program);

            let disk = disk.into_iter().map(|i| hm.remove(&i).unwrap()).collect();
            hm.insert(name, Program::new(weight, disk));

            false
        });
    }

    hm.into_iter().next_uw().1.recurse(0)
}

fn lower_strings(string: &str) -> Vec<&str> {
    let mut res = vec![];
    let mut start = None;
    for (i, c) in string.bytes().enumerate() {
        if c.is_ascii_lowercase() {
            if start.is_none() {
                start = Some(i);
            }
        } else if let Some(start) = start.take() {
            res.push(&string[start..i]);
        }
    }
    if let Some(start) = start.take() {
        res.push(&string[start..]);
    }
    res
}
