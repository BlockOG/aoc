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

    let mut cuboids: Vec<Cuboid> = input
        .lines()
        .map(|i| {
            let i = i.uints::<6, usize>();

            max_x = max_x.max(i[3]);
            max_y = max_y.max(i[4]);

            Cuboid {
                pos1: (i[0], i[1], i[2]),
                pos2: (i[3], i[4], i[5]),
            }
        })
        .collect();

    cuboids.sort_unstable_by_key(|i| i.pos1.2);

    let mut grid = vec![vec![(0, usize::MAX); max_y + 1]; max_x + 1];

    let mut resting_on_me = vec![vec![]; cuboids.len()];
    let mut is_resting_on = Vec::with_capacity(cuboids.len());
    for (i, cuboid) in cuboids.into_iter().enumerate() {
        let mut resting_on = FxHashSet::default();
        let mut z = 1;
        for x in cuboid.pos1.0..cuboid.pos2.0 + 1 {
            for y in cuboid.pos1.1..cuboid.pos2.1 + 1 {
                if grid[x][y].0 >= z {
                    z = grid[x][y].0 + 1;
                    resting_on.clear();
                    resting_on.insert(grid[x][y].1);
                } else if grid[x][y].1 != usize::MAX && grid[x][y].0 + 1 == z {
                    resting_on.insert(grid[x][y].1);
                }
            }
        }

        is_resting_on.push(resting_on.len());
        for j in resting_on {
            resting_on_me[j].push(i);
        }

        z += cuboid.pos2.2 - cuboid.pos1.2;
        for x in cuboid.pos1.0..cuboid.pos2.0 + 1 {
            for y in cuboid.pos1.1..cuboid.pos2.1 + 1 {
                grid[x][y] = (z, i);
            }
        }
    }

    resting_on_me
        .into_iter()
        .filter(|i| i.iter().all(|&i| is_resting_on[i] > 1))
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut max_x = 0;
    let mut max_y = 0;

    let mut cuboids: Vec<Cuboid> = input
        .lines()
        .map(|i| {
            let i = i.uints::<6, usize>();

            max_x = max_x.max(i[3]);
            max_y = max_y.max(i[4]);

            Cuboid {
                pos1: (i[0], i[1], i[2]),
                pos2: (i[3], i[4], i[5]),
            }
        })
        .collect();

    cuboids.sort_unstable_by_key(|i| i.pos1.2);

    let mut grid = vec![vec![(0, usize::MAX); max_y + 1]; max_x + 1];

    let mut resting_on_me = vec![vec![]; cuboids.len()];
    let mut is_resting_on = Vec::with_capacity(cuboids.len());
    for (i, cuboid) in cuboids.iter().enumerate() {
        let mut resting_on = FxHashSet::default();
        let mut z = 1;
        for x in cuboid.pos1.0..cuboid.pos2.0 + 1 {
            for y in cuboid.pos1.1..cuboid.pos2.1 + 1 {
                if grid[x][y].0 >= z {
                    z = grid[x][y].0 + 1;
                    resting_on.clear();
                    resting_on.insert(grid[x][y].1);
                } else if grid[x][y].1 != usize::MAX && grid[x][y].0 + 1 == z {
                    resting_on.insert(grid[x][y].1);
                }
            }
        }

        is_resting_on.push(resting_on.len());
        for j in resting_on {
            resting_on_me[j].push(i);
        }

        z += cuboid.pos2.2 - cuboid.pos1.2;
        for x in cuboid.pos1.0..cuboid.pos2.0 + 1 {
            for y in cuboid.pos1.1..cuboid.pos2.1 + 1 {
                grid[x][y] = (z, i);
            }
        }
    }

    (0..cuboids.len())
        .into_par_iter()
        .map(|i| {
            disintegrate(
                i,
                &resting_on_me,
                &is_resting_on,
                &mut vec![0; cuboids.len()],
            )
        })
        .sum::<usize>()
}

fn disintegrate(
    i: usize,
    resting_on_me: &Vec<Vec<usize>>,
    is_resting_on: &Vec<usize>,
    count: &mut Vec<usize>,
) -> usize {
    let mut sum = 0;
    for &j in resting_on_me[i].iter() {
        count[j] += 1;
        if count[j] == is_resting_on[j] {
            sum += 1 + disintegrate(j, resting_on_me, is_resting_on, count);
        }
    }

    sum
}
