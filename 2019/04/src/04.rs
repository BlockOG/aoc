use std::cmp::Ordering;

use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let [a, b] = input.raw().uints::<2, usize>();

    let mut sum = 0;
    'outer: for i in a..b + 1 {
        let mut repeat = false;
        for i in i.to_string().as_bytes().windows(2) {
            match i[0].cmp(&i[1]) {
                Ordering::Less => {}
                Ordering::Equal => repeat = true,
                Ordering::Greater => continue 'outer,
            }
        }

        if repeat {
            sum += 1;
        }
    }

    sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    let [a, b] = input.raw().uints::<2, usize>();

    let mut sum = 0;
    'outer: for i in a..b + 1 {
        let mut repeat = false;
        let mut curr_repeat = 0;
        for i in i.to_string().as_bytes().windows(2) {
            match i[0].cmp(&i[1]) {
                Ordering::Less => {
                    if curr_repeat == 1 {
                        repeat = true;
                    }
                    curr_repeat = 0;
                }
                Ordering::Equal => {
                    curr_repeat += 1;
                }
                Ordering::Greater => continue 'outer,
            }
        }

        if repeat || curr_repeat == 1 {
            sum += 1;
        }
    }

    sum
}
