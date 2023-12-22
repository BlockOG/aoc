use aoc::Parse;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rustc_hash::FxHashSet;

aoc::parts!(1, 2);

#[derive(Debug, Clone)]
struct Cuboid {
    pos1: (usize, usize, usize),
    pos2: (usize, usize, usize),
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    let mut cuboids: Vec<Cuboid> = input
        .lines()
        .map(|i| {
            let i = i.uints::<6, usize>();

            max_x = max_x.max(i[3]);
            max_y = max_y.max(i[4]);
            max_z = max_z.max(i[5]);

            Cuboid {
                pos1: (i[0], i[1], i[2]),
                pos2: (i[3], i[4], i[5]),
            }
        })
        .collect();

    cuboids.sort_unstable_by_key(|i| i.pos1.2);

    let mut grid = vec![vec![vec![usize::MAX; max_z + 1]; max_y + 1]; max_x + 1];

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    let mut resting_on_me = vec![vec![]; cuboids.len()];
    let mut is_resting_on = Vec::with_capacity(cuboids.len());
    for (i, cuboid) in cuboids.iter_mut().enumerate() {
        let mut resting_on = FxHashSet::default();
        while resting_on.is_empty() {
            cuboid.pos1.2 -= 1;
            cuboid.pos2.2 -= 1;
            if cuboid.pos1.2 < 1 {
                break;
            }
            for x in cuboid.pos1.0..cuboid.pos2.0 + 1 {
                for y in cuboid.pos1.1..cuboid.pos2.1 + 1 {
                    for z in cuboid.pos1.2..cuboid.pos2.2 + 1 {
                        if grid[x][y][z] != usize::MAX {
                            resting_on.insert(grid[x][y][z]);
                        }
                    }
                }
            }
        }

        is_resting_on.push(resting_on.len());
        for j in resting_on {
            resting_on_me[j].push(i);
        }

        cuboid.pos1.2 += 1;
        cuboid.pos2.2 += 1;
        for x in cuboid.pos1.0..cuboid.pos2.0 + 1 {
            for y in cuboid.pos1.1..cuboid.pos2.1 + 1 {
                for z in cuboid.pos1.2..cuboid.pos2.2 + 1 {
                    grid[x][y][z] = i;
                }
            }
        }

        max_x = max_x.max(cuboid.pos2.0);
        max_y = max_y.max(cuboid.pos2.1);
        max_z = max_z.max(cuboid.pos2.2);
    }

    resting_on_me
        .into_iter()
        .filter(|i| i.iter().all(|&i| is_resting_on[i] > 1))
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    let mut cuboids: Vec<Cuboid> = input
        .lines()
        .map(|i| {
            let i = i.uints::<6, usize>();

            max_x = max_x.max(i[3]);
            max_y = max_y.max(i[4]);
            max_z = max_z.max(i[5]);

            Cuboid {
                pos1: (i[0], i[1], i[2]),
                pos2: (i[3], i[4], i[5]),
            }
        })
        .collect();

    cuboids.sort_unstable_by_key(|i| i.pos1.2);

    let mut grid = vec![vec![vec![usize::MAX; max_z + 1]; max_y + 1]; max_x + 1];

    let mut resting_on_me = vec![vec![]; cuboids.len()];
    let mut is_resting_on = Vec::with_capacity(cuboids.len());
    for (i, cuboid) in cuboids.iter_mut().enumerate() {
        let mut resting_on = FxHashSet::default();
        while resting_on.is_empty() {
            cuboid.pos1.2 -= 1;
            cuboid.pos2.2 -= 1;
            if cuboid.pos1.2 < 1 {
                break;
            }
            for x in cuboid.pos1.0..cuboid.pos2.0 + 1 {
                for y in cuboid.pos1.1..cuboid.pos2.1 + 1 {
                    for z in cuboid.pos1.2..cuboid.pos2.2 + 1 {
                        if grid[x][y][z] != usize::MAX {
                            resting_on.insert(grid[x][y][z]);
                        }
                    }
                }
            }
        }

        is_resting_on.push(resting_on.len());
        for j in resting_on {
            resting_on_me[j].push(i);
        }

        cuboid.pos1.2 += 1;
        cuboid.pos2.2 += 1;
        for x in cuboid.pos1.0..cuboid.pos2.0 + 1 {
            for y in cuboid.pos1.1..cuboid.pos2.1 + 1 {
                for z in cuboid.pos1.2..cuboid.pos2.2 + 1 {
                    grid[x][y][z] = i;
                }
            }
        }
    }

    (0..resting_on_me.len())
        .into_par_iter()
        .map(|i| {
            recurse(
                i,
                cuboids.clone(),
                vec![vec![vec![usize::MAX; max_z + 1]; max_y + 1]; max_x + 1],
            )
        })
        .sum::<usize>()
}

fn recurse(i: usize, mut cuboids: Vec<Cuboid>, mut grid: Vec<Vec<Vec<usize>>>) -> usize {
    cuboids.remove(i);

    let mut sum = 0;
    for (i, cuboid) in cuboids.iter_mut().enumerate() {
        let mut fell = 0;
        'outer: loop {
            cuboid.pos1.2 -= 1;
            cuboid.pos2.2 -= 1;
            fell += 1;
            if cuboid.pos1.2 < 1 {
                break;
            }
            for x in cuboid.pos1.0..cuboid.pos2.0 + 1 {
                for y in cuboid.pos1.1..cuboid.pos2.1 + 1 {
                    for z in cuboid.pos1.2..cuboid.pos2.2 + 1 {
                        if grid[x][y][z] != usize::MAX {
                            break 'outer;
                        }
                    }
                }
            }
        }

        if fell > 1 {
            sum += 1;
        }

        cuboid.pos1.2 += 1;
        cuboid.pos2.2 += 1;
        for x in cuboid.pos1.0..cuboid.pos2.0 + 1 {
            for y in cuboid.pos1.1..cuboid.pos2.1 + 1 {
                for z in cuboid.pos1.2..cuboid.pos2.2 + 1 {
                    grid[x][y][z] = i;
                }
            }
        }
    }

    sum
}
