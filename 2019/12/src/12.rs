use std::cmp::Ordering;

use aoc::Parse;

aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut moons: Vec<[i32; 3]> = input.lines().map(|i| i.ints()).collect();
    let mut vels = vec![[0; 3]; moons.len()];
    for _ in 0..1000 {
        for i in 0..moons.len() - 1 {
            for j in i + 1..moons.len() {
                for k in 0..3 {
                    match moons[i][k].cmp(&moons[j][k]) {
                        Ordering::Less => {
                            vels[i][k] += 1;
                            vels[j][k] -= 1;
                        }
                        Ordering::Equal => {}
                        Ordering::Greater => {
                            vels[i][k] -= 1;
                            vels[j][k] += 1;
                        }
                    }
                }
            }
        }

        for (i, j) in moons.iter_mut().zip(vels.iter()) {
            for k in 0..3 {
                i[k] += j[k];
            }
        }
    }

    moons
        .into_iter()
        .zip(vels)
        .map(|i| {
            (i.0[0].abs() + i.0[1].abs() + i.0[2].abs())
                * (i.1[0].abs() + i.1[1].abs() + i.1[2].abs())
        })
        .sum::<i32>()
}

fn part_2(_input: aoc::Input) -> impl ToString {
    // let mut moons: Vec<[i32; 3]> = input.lines().map(|i| i.ints()).collect();
    // let mut vels = vec![[0; 3]; moons.len()];

    // let mut hm: FxHashSet<(Vec<[i32; 3]>, Vec<[i32; 3]>)> = FxHashSet::default();

    // let mut steps = [0; 3];
    // for k in 0..3 {
    //     let mut step = 0;
    //     loop {
    //         hm.insert((moons.clone(), vels.clone()));

    //         for i in 0..moons.len() - 1 {
    //             for j in i + 1..moons.len() {
    //                 match moons[i][k].cmp(&moons[j][k]) {
    //                     Ordering::Less => {
    //                         vels[i][k] += 1;
    //                         vels[j][k] -= 1;
    //                     }
    //                     Ordering::Equal => {}
    //                     Ordering::Greater => {
    //                         vels[i][k] -= 1;
    //                         vels[j][k] += 1;
    //                     }
    //                 }
    //             }
    //         }

    //         for (i, j) in moons.iter_mut().zip(vels.iter()) {
    //             for k in 0..3 {
    //                 i[k] += j[k];
    //             }
    //         }

    //         step += 1;

    //     }

    //     steps[k] = step;
    // }

    // steps[0]
    0
}
