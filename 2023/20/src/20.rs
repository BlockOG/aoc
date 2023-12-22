use std::{collections::VecDeque, hint::unreachable_unchecked};

use aoc::Parse;
use rustc_hash::FxHashMap;
use string_to_index::StringToIndex;

aoc::parts!(1, 2);

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(Vec<usize>, bool),
    Conjunction(Vec<usize>, FxHashMap<usize, bool>),
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut modules = vec![None; input.len()];
    let mut hm = StringToIndex::new();
    modules[hm.get("rx")] = Some(Module::FlipFlop(vec![], false));

    let mut broadcaster = vec![];
    for line in input.lines() {
        match line.idx(0) {
            b'%' => {
                modules[hm.get(&line[1..3])] = Some(Module::FlipFlop(
                    line[7..].split(", ").map(|i| hm.get(i)).collect(),
                    false,
                ));
            }
            b'&' => {
                modules[hm.get(&line[1..3])] = Some(Module::Conjunction(
                    line[7..].split(", ").map(|i| hm.get(i)).collect(),
                    FxHashMap::default(),
                ));
            }
            _ => {
                broadcaster = line[15..].split(", ").map(|i| hm.get(i)).collect();
            }
        }
    }

    let mut modules: Vec<Module> = modules.into_iter().map(|i| i.unwrap()).collect();

    for i in 0..modules.len() {
        for &j in match unsafe { &*(&modules[i] as *const _) } {
            Module::FlipFlop(dests, _) | Module::Conjunction(dests, _) => dests,
        } {
            if let Module::Conjunction(_, inputs) = &mut modules[j] {
                inputs.insert(i, false);
            }
        }
    }

    let mut low = 1000 * (broadcaster.len() + 1);
    let mut high = 0;
    let mut pulses = VecDeque::new();
    for _ in 0..1000 {
        for &i in broadcaster.iter() {
            pulses.push_back((0, i, false));
        }
        while let Some((src, dest, val)) = pulses.pop_front() {
            match &mut modules[dest] {
                Module::FlipFlop(dests, state) => {
                    if !val {
                        *state = !*state;
                        if *state {
                            high += dests.len();
                        } else {
                            low += dests.len();
                        }
                        for &mut new_dest in dests {
                            pulses.push_back((dest, new_dest, *state));
                        }
                    }
                }
                Module::Conjunction(dests, state) => {
                    state.insert(src, val);
                    let mut val = false;
                    for &i in state.values() {
                        if !i {
                            val = true;
                            break;
                        }
                    }

                    if val {
                        high += dests.len();
                    } else {
                        low += dests.len();
                    }
                    for &mut new_dest in dests {
                        pulses.push_back((dest, new_dest, val));
                    }
                }
            }
        }
    }

    low * high
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut modules = vec![None; input.len() - 1 - 4 - 1];
    let mut hm = StringToIndex::new();

    let mut broadcaster = vec![];
    for line in input.lines() {
        match line.idx(0) {
            b'%' => {
                modules[hm.get(&line[1..3])] =
                    Some(line[7..].split(", ").map(|i| hm.get(i)).collect());
            }
            b'&' => {}
            _ => {
                broadcaster = line[15..].split(", ").map(|i| hm.get(i)).collect();
            }
        }
    }

    broadcaster
        .into_iter()
        .map(|i| recurse(i, &modules))
        .product::<usize>()
}

fn recurse(i: usize, modules: &Vec<Option<Vec<usize>>>) -> usize {
    if let Some(dests) = &modules[i] {
        match dests.len() {
            0 => 0,
            1 => {
                if modules[dests[0]].is_some() {
                    let i = recurse(dests[0], modules) << 1;
                    i
                } else {
                    1
                }
            }
            2 => {
                if modules[dests[0]].is_some() {
                    (recurse(dests[0], modules) << 1) + 1
                } else {
                    (recurse(dests[1], modules) << 1) + 1
                }
            }
            _ => unsafe { unreachable_unchecked() },
        }
    } else {
        unsafe { unreachable_unchecked() }
    }
}
