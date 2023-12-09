use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut sum = 0;
    for i in input {
        let mut i: Vec<i32> = i.ints_iter().collect();

        'outer: for j in (1..=i.len()).rev() {
            for &k in i.iter().take(j) {
                if k != 0 {
                    for j in 0..j - 1 {
                        i[j] = i[j + 1] - i[j];
                    }

                    sum += i[j - 1];
                    continue 'outer;
                }
            }

            break;
        }
    }

    sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut sum = 0;
    for i in input {
        let mut i: Vec<i32> = i.ints_iter().collect();

        let mut neg = false;
        'outer: for j in 0..i.len() {
            for &k in i.iter().skip(j) {
                if k != 0 {
                    for j in (j + 1..i.len()).rev() {
                        i[j] = i[j] - i[j - 1];
                    }

                    if neg {
                        sum -= i[j];
                    } else {
                        sum += i[j];
                    }
                    neg = !neg;
                    continue 'outer;
                }
            }

            break;
        }
    }

    sum
}
