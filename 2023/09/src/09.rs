use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut sum = 0;
    for i in input {
        let mut lasts = vec![];
        let mut i: Vec<i32> = i.ints_iter().collect();

        'outer: loop {
            for &j in i.iter() {
                if j != 0 {
                    let mut k = vec![0; i.len() - 1];
                    for (j, i) in i.windows(2).enumerate() {
                        k[j] = i[1] - i[0];
                    }
                    lasts.push(*i.last().unwrap());
                    i = k;

                    continue 'outer;
                }
            }

            break;
        }

        println!("{lasts:?}");

        sum += lasts.into_iter().sum::<i32>();
    }

    sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut sum = 0;
    for i in input {
        let mut firsts = vec![];
        let mut i: Vec<i32> = i.ints_iter().collect();

        'outer: loop {
            for &j in i.iter() {
                if j != 0 {
                    let mut k = vec![0; i.len() - 1];
                    for (j, i) in i.windows(2).enumerate() {
                        k[j] = i[1] - i[0];
                    }
                    firsts.push(*i.first().unwrap());
                    i = k;

                    continue 'outer;
                }
            }

            break;
        }

        let mut neg = false;
        for i in firsts {
            if neg {
                sum -= i;
            } else {
                sum += i;
            }
            neg = !neg;
        }
    }

    sum
}
