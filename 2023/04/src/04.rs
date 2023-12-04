use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|i| {
            let mut res = 0;
            let mut a = [false; 100];
            let mut list = true;
            for i in i.split(" ").skip(2) {
                if i.chars().last() == Some(':') {
                    continue;
                }
                if i == "|" {
                    list = false;
                } else if i.len() > 0 {
                    if list {
                        a[i.parse_uw::<usize>()] = true;
                    } else {
                        if a[i.parse_uw::<usize>()] {
                            if res == 0 {
                                res = 1;
                            } else {
                                res *= 2;
                            }
                        }
                    }
                }
            }
            res
        })
        .sum::<i32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut sum = 0;
    let mut cards = vec![1; input.lines().len()];
    for i in 0..cards.len() {
        sum += cards[i];
        let mut k = i + 1;
        let mut a = [false; 100];
        let mut list = true;
        for j in input.as_lines()[i].split(" ").skip(2) {
            if j.chars().last() == Some(':') {
                continue;
            }
            if j == "|" {
                list = false;
            } else if j.len() > 0 {
                if list {
                    a[j.parse_uw::<usize>()] = true;
                } else {
                    if a[j.parse_uw::<usize>()] {
                        if k < cards.len() {
                            cards[k] += cards[i];
                            k += 1;
                        }
                    }
                }
            }
        }
    }

    sum
}
