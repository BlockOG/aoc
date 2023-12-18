use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    hash::{BuildHasherDefault, Hash},
    iter,
};

use indexmap::{map::Entry, IndexMap};

use rustc_hash::FxHasher;

aoc::parts!(1, 2);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn turn(&self) -> [Direction; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Down, Direction::Up],
        }
    }
}

type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

struct SmallestCostHolder {
    estimated_cost: usize,
    cost: usize,
    index: usize,
}

impl PartialEq for SmallestCostHolder {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost.eq(&other.estimated_cost) && self.cost.eq(&other.cost)
    }
}

impl Eq for SmallestCostHolder {}

impl PartialOrd for SmallestCostHolder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SmallestCostHolder {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.estimated_cost.cmp(&self.estimated_cost) {
            Ordering::Equal => self.cost.cmp(&other.cost),
            s => s,
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let n = input[0].len();
    let m = input.len();
    let grid: Vec<usize> = input
        .lines()
        .flat_map(|i| i.bytes().map(|i| (i - b'0') as usize))
        .collect();

    let mut to_see = BinaryHeap::new();
    to_see.push(SmallestCostHolder {
        estimated_cost: grid[1],
        cost: grid[1],
        index: 0,
    });
    to_see.push(SmallestCostHolder {
        estimated_cost: grid[n],
        cost: grid[n],
        index: 1,
    });

    let mut parents = FxIndexMap::default();
    parents.insert(((1, 0), 1, Direction::Right), (usize::MAX, grid[1]));
    parents.insert(((0, 1), 1, Direction::Down), (usize::MAX, grid[n]));

    while let Some(SmallestCostHolder { cost, index, .. }) = to_see.pop() {
        let (&(pos, consecutive, direction), &(_, c)) = parents.get_index(index).unwrap();
        if pos.0 == n - 1 && pos.1 == m - 1 {
            let mut i = index;
            return iter::from_fn(|| {
                parents.get_index(i).map(|(node, value)| {
                    i = value.0;
                    node
                })
            })
            .map(|(pos, _consecutive, _direction)| grid[pos.1 * n + pos.0])
            .sum::<usize>();
        }
        if cost > c {
            continue;
        }

        let mut add = |successor, move_cost| {
            let new_cost = cost + move_cost;
            let h;
            let i;
            match parents.entry(successor) {
                Entry::Vacant(e) => {
                    let (pos, _, _) = e.key();
                    h = n + m - pos.0 - pos.1;
                    i = e.index();
                    e.insert((index, new_cost));
                }
                Entry::Occupied(mut e) => {
                    if e.get().1 > new_cost {
                        let (pos, _, _) = e.key();
                        h = n + m - pos.0 - pos.1;
                        i = e.index();
                        e.insert((index, new_cost));
                    } else {
                        return;
                    }
                }
            }

            to_see.push(SmallestCostHolder {
                estimated_cost: new_cost + h,
                cost: new_cost,
                index: i,
            });
        };

        if consecutive < 3 {
            match direction {
                Direction::Up => {
                    if pos.1 > 0 {
                        add(
                            ((pos.0, pos.1 - 1), consecutive + 1, direction),
                            grid[(pos.1 - 1) * n + pos.0],
                        );
                    }
                }
                Direction::Left => {
                    if pos.0 > 0 {
                        add(
                            ((pos.0 - 1, pos.1), consecutive + 1, direction),
                            grid[pos.1 * n + pos.0 - 1],
                        );
                    }
                }
                Direction::Down => {
                    if pos.1 < m - 1 {
                        add(
                            ((pos.0, pos.1 + 1), consecutive + 1, direction),
                            grid[(pos.1 + 1) * n + pos.0],
                        );
                    }
                }
                Direction::Right => {
                    if pos.0 < n - 1 {
                        add(
                            ((pos.0 + 1, pos.1), consecutive + 1, direction),
                            grid[pos.1 * n + pos.0 + 1],
                        );
                    }
                }
            }
        }

        for dir in direction.turn() {
            match dir {
                Direction::Up => {
                    if pos.1 > 0 {
                        add(((pos.0, pos.1 - 1), 1, dir), grid[(pos.1 - 1) * n + pos.0]);
                    }
                }
                Direction::Left => {
                    if pos.0 > 0 {
                        add(((pos.0 - 1, pos.1), 1, dir), grid[pos.1 * n + pos.0 - 1]);
                    }
                }
                Direction::Down => {
                    if pos.1 < m - 1 {
                        add(((pos.0, pos.1 + 1), 1, dir), grid[(pos.1 + 1) * n + pos.0]);
                    }
                }
                Direction::Right => {
                    if pos.0 < n - 1 {
                        add(((pos.0 + 1, pos.1), 1, dir), grid[pos.1 * n + pos.0 + 1]);
                    }
                }
            }
        }
    }

    0
}

fn part_2(input: aoc::Input) -> impl ToString {
    let n = input[0].len();
    let m = input.len();
    let grid: Vec<usize> = input
        .lines()
        .flat_map(|i| i.bytes().map(|i| (i - b'0') as usize))
        .collect();

    let mut to_see = BinaryHeap::new();
    to_see.push(SmallestCostHolder {
        estimated_cost: grid[1] + grid[2] + grid[3] + grid[4],
        cost: grid[1] + grid[2] + grid[3] + grid[4],
        index: 0,
    });
    to_see.push(SmallestCostHolder {
        estimated_cost: grid[n] + grid[n * 2] + grid[n * 3] + grid[n * 4],
        cost: grid[n] + grid[n * 2] + grid[n * 3] + grid[n * 4],
        index: 1,
    });

    let mut parents = FxIndexMap::default();
    parents.insert(
        ((4, 0), 4, Direction::Right),
        (usize::MAX, grid[1] + grid[2] + grid[3] + grid[4]),
    );
    parents.insert(
        ((0, 4), 4, Direction::Down),
        (
            usize::MAX,
            grid[n] + grid[n * 2] + grid[n * 3] + grid[n * 4],
        ),
    );

    while let Some(SmallestCostHolder { cost, index, .. }) = to_see.pop() {
        let (&(pos, consecutive, direction), &(_, c)) = parents.get_index(index).unwrap();
        if pos.0 == n - 1 && pos.1 == m - 1 {
            let mut i = index;
            return iter::from_fn(|| {
                parents.get_index(i).map(|(node, value)| {
                    i = value.0;
                    node
                })
            })
            .map(|&(pos, consecutive, direction)| {
                if consecutive == 4 {
                    match direction {
                        Direction::Up => {
                            grid[pos.1 * n + pos.0]
                                + grid[(pos.1 + 1) * n + pos.0]
                                + grid[(pos.1 + 2) * n + pos.0]
                                + grid[(pos.1 + 3) * n + pos.0]
                        }
                        Direction::Left => {
                            grid[pos.1 * n + pos.0]
                                + grid[pos.1 * n + pos.0 + 1]
                                + grid[pos.1 * n + pos.0 + 2]
                                + grid[pos.1 * n + pos.0 + 3]
                        }
                        Direction::Down => {
                            grid[pos.1 * n + pos.0]
                                + grid[(pos.1 - 1) * n + pos.0]
                                + grid[(pos.1 - 2) * n + pos.0]
                                + grid[(pos.1 - 3) * n + pos.0]
                        }
                        Direction::Right => {
                            grid[pos.1 * n + pos.0]
                                + grid[pos.1 * n + pos.0 - 1]
                                + grid[pos.1 * n + pos.0 - 2]
                                + grid[pos.1 * n + pos.0 - 3]
                        }
                    }
                } else {
                    grid[pos.1 * n + pos.0]
                }
            })
            .sum::<usize>();
        }
        if cost > c {
            continue;
        }

        let mut add = |successor, move_cost| {
            let new_cost = cost + move_cost;
            let h;
            let i;
            match parents.entry(successor) {
                Entry::Vacant(e) => {
                    let (pos, _, _) = e.key();
                    h = n + m - pos.0 - pos.1;
                    i = e.index();
                    e.insert((index, new_cost));
                }
                Entry::Occupied(mut e) => {
                    if e.get().1 > new_cost {
                        let (pos, _, _) = e.key();
                        h = n + m - pos.0 - pos.1;
                        i = e.index();
                        e.insert((index, new_cost));
                    } else {
                        return;
                    }
                }
            }

            to_see.push(SmallestCostHolder {
                estimated_cost: new_cost + h,
                cost: new_cost,
                index: i,
            });
        };

        if consecutive < 10 {
            match direction {
                Direction::Up => {
                    if pos.1 > 0 {
                        add(
                            ((pos.0, pos.1 - 1), consecutive + 1, direction),
                            grid[(pos.1 - 1) * n + pos.0],
                        );
                    }
                }
                Direction::Left => {
                    if pos.0 > 0 {
                        add(
                            ((pos.0 - 1, pos.1), consecutive + 1, direction),
                            grid[pos.1 * n + pos.0 - 1],
                        );
                    }
                }
                Direction::Down => {
                    if pos.1 < m - 1 {
                        add(
                            ((pos.0, pos.1 + 1), consecutive + 1, direction),
                            grid[(pos.1 + 1) * n + pos.0],
                        );
                    }
                }
                Direction::Right => {
                    if pos.0 < n - 1 {
                        add(
                            ((pos.0 + 1, pos.1), consecutive + 1, direction),
                            grid[pos.1 * n + pos.0 + 1],
                        );
                    }
                }
            }
        }

        for dir in direction.turn() {
            match dir {
                Direction::Up => {
                    if pos.1 >= 4 {
                        add(
                            ((pos.0, pos.1 - 4), 4, dir),
                            grid[(pos.1 - 1) * n + pos.0]
                                + grid[(pos.1 - 2) * n + pos.0]
                                + grid[(pos.1 - 3) * n + pos.0]
                                + grid[(pos.1 - 4) * n + pos.0],
                        );
                    }
                }
                Direction::Left => {
                    if pos.0 >= 4 {
                        add(
                            ((pos.0 - 4, pos.1), 4, dir),
                            grid[pos.1 * n + pos.0 - 1]
                                + grid[pos.1 * n + pos.0 - 2]
                                + grid[pos.1 * n + pos.0 - 3]
                                + grid[pos.1 * n + pos.0 - 4],
                        );
                    }
                }
                Direction::Down => {
                    if pos.1 < m - 4 {
                        add(
                            ((pos.0, pos.1 + 4), 4, dir),
                            grid[(pos.1 + 1) * n + pos.0]
                                + grid[(pos.1 + 2) * n + pos.0]
                                + grid[(pos.1 + 3) * n + pos.0]
                                + grid[(pos.1 + 4) * n + pos.0],
                        );
                    }
                }
                Direction::Right => {
                    if pos.0 < n - 4 {
                        add(
                            ((pos.0 + 4, pos.1), 4, dir),
                            grid[pos.1 * n + pos.0 + 1]
                                + grid[pos.1 * n + pos.0 + 2]
                                + grid[pos.1 * n + pos.0 + 3]
                                + grid[pos.1 * n + pos.0 + 4],
                        );
                    }
                }
            }
        }
    }

    0
}
