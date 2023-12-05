use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut seeds = input
        .lines()
        .next()
        .unwrap()
        .uints_iter::<u32>()
        .collect::<Vec<u32>>();
    seeds.sort();

    let mut a = vec![vec![]];
    let mut i = 0;
    for line in input.lines().skip(2) {
        if line == "" {
            a[i].sort_by_key(|i: &(u32, u32, u32)| i.1);
            i = a.len();
            a.push(vec![]);
        } else if line.bytes().last().unwrap() != b':' {
            let j = line.uints::<3, u32>();
            a[i].push((j[0], j[1], j[2]));
        }
    }
    a[i].sort_by_key(|i| i.1);

    let mut j = 0;
    for i in a {
        for i in i {
            while j < seeds.len() && i.1 > seeds[j] {
                j += 1;
            }
            while j < seeds.len() && seeds[j] <= i.1 + i.2 {
                seeds[j] = i.0 + (seeds[j] - i.1);
                j += 1;
            }
            if j >= seeds.len() {
                break;
            }
        }
        j = 0;
        seeds.sort();
    }

    seeds.into_iter().min().unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut seeds = vec![];

    let mut turn = false;
    for i in input.lines().next().unwrap().uints_iter::<u32>() {
        if !turn {
            seeds.push((i, i));
        } else {
            seeds.last_mut().unwrap().1 += i;
        }
        turn = !turn;
    }

    let mut a = vec![vec![]];
    let mut i = 0;
    for line in input.lines().skip(2) {
        if line == "" {
            i = a.len();
            a.push(vec![]);
        } else if line.bytes().last().unwrap() != b':' {
            let j = line.uints::<3, u32>();
            a[i].push((j[1], j[1] + j[2], j[0]));
        }
    }

    for i in a {
        let mut new_seeds = vec![];
        'outer: while let Some(seed) = seeds.pop() {
            let mut s1 = seed;
            let mut s2 = seed;
            let mut s3 = seed;

            for i in i.iter() {
                if (seed.0 <= i.0 && i.0 < seed.1)
                    || (seed.0 < i.1 && i.1 <= seed.1)
                    || (i.0 <= seed.0 && seed.0 < i.1)
                    || (i.0 < seed.1 && seed.1 <= i.1)
                {
                    s1.1 = i.0;
                    s2.0 = i.0.max(s2.0) - i.0 + i.2;
                    s2.1 = i.1.min(s2.1) - i.0 + i.2;
                    s3.0 = i.1;

                    if s1.0 < s1.1 {
                        seeds.push(s1);
                    }
                    if s2.0 < s2.1 {
                        new_seeds.push(s2);
                    }
                    if s3.0 < s3.1 {
                        seeds.push(s3);
                    }

                    continue 'outer;
                }
            }

            new_seeds.push(seed);
        }
        seeds = new_seeds;
    }

    seeds.into_iter().min_by_key(|i| i.0).unwrap().0
}
