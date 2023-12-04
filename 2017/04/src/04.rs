use std::collections::HashSet;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .filter(|s| {
            let mut words = HashSet::new();
            for i in s.split(" ") {
                if words.contains(i) {
                    return false;
                } else {
                    words.insert(i);
                }
            }
            true
        })
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .filter(|s| {
            let mut words = HashSet::new();
            for i in s.split(" ") {
                let mut j = [0; 26];
                for i in i.bytes() {
                    j[(i - b'a') as usize] += 1;
                }

                if words.contains(&j) {
                    return false;
                } else {
                    words.insert(j);
                }
            }
            true
        })
        .count()
}
