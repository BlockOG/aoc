use std::{
    cmp::Ordering, collections::hash_map::Entry, hash::BuildHasherDefault,
    hint::unreachable_unchecked,
};

use aoc::{IterUnwrap, Parse};

use indexmap::IndexSet;
use rustc_hash::{FxHashMap, FxHasher};

type FxIndexSet<T> = IndexSet<T, BuildHasherDefault<FxHasher>>;

aoc::parts!(1, 2);

#[derive(Debug)]
struct StringToIndex {
    hm: FxHashMap<String, usize>,
    i: usize,
}

impl StringToIndex {
    fn new() -> Self {
        Self {
            hm: FxHashMap::default(),
            i: usize::MAX,
        }
    }

    fn get(&mut self, s: &str) -> usize {
        match self.hm.entry(s.to_owned()) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                self.i = self.i.wrapping_add(1);
                *entry.insert(self.i)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum SentTo {
    Accepted,
    Rejected,
    Label(usize),
}

impl SentTo {
    fn parse(s: &str, hm: &mut StringToIndex) -> Self {
        match s {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            _ => Self::Label(hm.get(s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Category {
    CoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    category: Category,
    greater: bool,
    value: u64,

    sent_to: SentTo,
}

impl Rule {
    fn parse(s: &str, hm: &mut StringToIndex) -> Self {
        let mut i = 2;
        let mut value = 0;
        while s.idx(i) != b':' {
            value = value * 10 + (s.idx(i) - b'0') as u64;
            i += 1;
        }

        Self {
            category: match s.idx(0) {
                b'x' => Category::CoolLooking,
                b'm' => Category::Musical,
                b'a' => Category::Aerodynamic,
                b's' => Category::Shiny,
                _ => unsafe { unreachable_unchecked() },
            },
            greater: s.idx(1) == b'>',
            value,

            sent_to: SentTo::parse(&s[i + 1..], hm),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
    last: SentTo,
}

impl Workflow {
    fn new(line: &str, hm: &mut StringToIndex) -> (usize, Self) {
        let rule_start = line.find("{").unwrap();

        let mut split = line[rule_start + 1..line.len() - 1].split(',');
        let mut last = split.next_uw();

        let mut rules = vec![];
        for i in split {
            rules.push(Rule::parse(last, hm));
            last = i;
        }

        (
            hm.get(&line[..rule_start]),
            Self {
                rules,
                last: SentTo::parse(last, hm),
            },
        )
    }

    fn apply(&self, values: [u64; 4]) -> SentTo {
        for &Rule {
            category,
            greater,
            value,
            sent_to,
        } in self.rules.iter()
        {
            if matches!(
                (values[category as usize].cmp(&value), greater,),
                (Ordering::Less, false) | (Ordering::Greater, true)
            ) {
                return sent_to;
            }
        }

        self.last
    }

    fn apply_range(
        &self,
        mut values: [Range; 4],
        ranges: &mut Vec<Vec<[Range; 4]>>,
        check: &mut FxIndexSet<usize>,
    ) -> u64 {
        let mut sum = 0;
        for &Rule {
            category,
            greater,
            value,
            sent_to,
        } in self.rules.iter()
        {
            let Range { start, end } = values[category as usize];
            if greater {
                values[category as usize].start = value + 1;
            } else {
                values[category as usize].end = value;
            }

            if values[category as usize].start < values[category as usize].end {
                match sent_to {
                    SentTo::Accepted => {
                        sum += values.iter().map(|i| i.end - i.start).product::<u64>()
                    }
                    SentTo::Rejected => {}
                    SentTo::Label(sent_to) => {
                        ranges[sent_to].push(values);
                        check.insert(sent_to);
                    }
                }
            }

            if greater {
                values[category as usize] = Range::new(start, value + 1);
            } else {
                values[category as usize] = Range::new(value, end);
            }

            if values[category as usize].start >= values[category as usize].end {
                return sum;
            }
        }

        match self.last {
            SentTo::Accepted => sum += values.iter().map(|i| i.end - i.start).product::<u64>(),
            SentTo::Rejected => {}
            SentTo::Label(sent_to) => {
                ranges[sent_to].push(values);
                check.insert(sent_to);
            }
        }

        sum
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut workflows = Vec::with_capacity(input.len());

    let mut input = input.as_lines().split(|i| i.is_empty());
    let mut hm = StringToIndex::new();
    for i in input.next_uw().into_iter() {
        let (position, workflow) = Workflow::new(i, &mut hm);

        while workflows.len() <= position {
            workflows.push(None);
        }

        workflows[position] = Some(workflow);
    }

    let workflows: Vec<_> = workflows.into_iter().map(|i| i.unwrap()).collect();
    let start = SentTo::Label(hm.get("in"));

    let mut sum = 0;
    for i in input.next_uw() {
        let part = i.uints();

        let mut curr = start;
        while let SentTo::Label(label) = curr {
            curr = workflows[label].apply(part);
        }

        if let SentTo::Accepted = curr {
            sum += part.into_iter().sum::<u64>();
        }
    }

    sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut workflows = Vec::with_capacity(input.len());

    let mut input = input.as_lines().split(|i| i.is_empty());
    let mut hm = StringToIndex::new();
    for i in input.next_uw().into_iter() {
        let (position, workflow) = Workflow::new(i, &mut hm);

        while workflows.len() <= position {
            workflows.push(None);
        }

        workflows[position] = Some(workflow);
    }

    let workflows: Vec<_> = workflows.into_iter().map(|i| i.unwrap()).collect();
    let mut ranges = vec![vec![]; workflows.len()];
    let mut check = FxIndexSet::default();

    ranges[hm.get("in")].push([Range::new(1, 4001); 4]);
    check.insert(hm.get("in"));

    let mut sum = 0;
    while let Some(curr) = check.pop() {
        let curr_ranges: Vec<_> = ranges[curr].drain(..).collect();
        for values in curr_ranges {
            sum += workflows[curr].apply_range(values, &mut ranges, &mut check);
        }
    }

    sum
}
