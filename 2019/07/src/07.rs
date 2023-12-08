use intcode::{Intcode, State};
use itertools::Itertools;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let start = Intcode::from(input);
    let mut res = 0;
    for i in (0..5).permutations(5) {
        let mut curr = 0;
        for i in i {
            let mut intcode = start.clone();
            intcode.run();
            intcode.input(i);
            intcode.run();
            intcode.input(curr);
            curr = *intcode.run().last().unwrap();
        }
        res = res.max(curr);
    }

    res
}

fn part_2(input: aoc::Input) -> impl ToString {
    let start = Intcode::from(input);
    let mut res = 0;
    for i in (5..10).permutations(5) {
        let mut curr = 0;
        let mut intcodes = [
            start.clone(),
            start.clone(),
            start.clone(),
            start.clone(),
            start.clone(),
        ];
        for (i, j) in i.into_iter().enumerate() {
            intcodes[i].run();
            intcodes[i].input(j);
        }
        for i in (0..5).cycle() {
            intcodes[i].run();
            if let State::Halted = intcodes[i].state {
                break;
            }
            intcodes[i].input(curr);
            curr = *intcodes[i].run().last().unwrap();
        }
        res = res.max(curr);
    }

    res
}
