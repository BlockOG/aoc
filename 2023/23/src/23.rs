use std::{cell::RefCell, hint::unreachable_unchecked, rc::Rc};

use rustc_hash::FxHashMap;

aoc::parts!(1, 2);

enum Tile {
    None,
    Wall,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    len: usize,
    children: Vec<Rc<RefCell<Node>>>,
}

const N: usize = 141;

fn part_1(input: aoc::Input) -> impl ToString {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|i| {
            i.bytes()
                .map(|i| match i {
                    b'.' => Tile::None,
                    b'#' => Tile::Wall,
                    b'>' => Tile::Right,
                    b'v' => Tile::Down,
                    b'<' => Tile::Left,
                    b'^' => Tile::Up,
                    _ => unsafe { unreachable_unchecked() },
                })
                .collect()
        })
        .collect();

    recurse2(recurse(
        1,
        0,
        Rc::new(RefCell::new(Node {
            len: 0,
            children: vec![],
        })),
        (1, usize::MAX),
        &grid,
        &mut FxHashMap::default(),
    ))
}

fn part_2(input: aoc::Input) -> impl ToString {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|i| {
            i.bytes()
                .map(|i| match i {
                    b'.' => Tile::None,
                    b'#' => Tile::Wall,
                    b'>' => Tile::None,
                    b'v' => Tile::None,
                    b'<' => Tile::None,
                    b'^' => Tile::None,
                    _ => unsafe { unreachable_unchecked() },
                })
                .collect()
        })
        .collect();

    recurse3(1, 0, 0, &grid, &mut vec![vec![false; N]; N])
}

fn recurse(
    x: usize,
    y: usize,
    mut i: Rc<RefCell<Node>>,
    coming_from: (usize, usize),
    grid: &Vec<Vec<Tile>>,
    cache: &mut FxHashMap<(usize, usize), Rc<RefCell<Node>>>,
) -> Rc<RefCell<Node>> {
    // if cache.contains_key(&(x, y)) {
    //     let j = cache[&(x, y)].clone();
    //     i.borrow_mut().children.push(j.clone());
    //     return j;
    // }

    if y == N - 1 {
        i.borrow_mut().children.push(Rc::new(RefCell::new(Node {
            len: usize::MAX,
            children: vec![],
        })));
        return i;
    }

    match grid[y][x] {
        Tile::None => {
            i.borrow_mut().len += 1;
        }
        Tile::Up | Tile::Down | Tile::Right | Tile::Left => {
            let j = Rc::new(RefCell::new(Node {
                len: 1,
                children: vec![],
            }));
            i.borrow_mut().children.push(j.clone());
            cache.insert((x, y), j.clone());
            i = j;
        }
        _ => unsafe { unreachable_unchecked() },
    }

    if matches!(grid[y][x], Tile::None | Tile::Up)
        && y > 0
        && (x, y - 1) != coming_from
        && !matches!(grid[y - 1][x], Tile::Wall)
    {
        recurse(x, y - 1, i.clone(), (x, y), grid, cache);
    }
    if matches!(grid[y][x], Tile::None | Tile::Left)
        && x > 0
        && (x - 1, y) != coming_from
        && !matches!(grid[y][x - 1], Tile::Wall)
    {
        recurse(x - 1, y, i.clone(), (x, y), grid, cache);
    }
    if matches!(grid[y][x], Tile::None | Tile::Down)
        && y < N - 1
        && (x, y + 1) != coming_from
        && !matches!(grid[y + 1][x], Tile::Wall)
    {
        recurse(x, y + 1, i.clone(), (x, y), grid, cache);
    }
    if matches!(grid[y][x], Tile::None | Tile::Right)
        && x < N - 1
        && (x + 1, y) != coming_from
        && !matches!(grid[y][x + 1], Tile::Wall)
    {
        recurse(x + 1, y, i.clone(), (x, y), grid, cache);
    }

    i
}

fn recurse3(
    mut x: usize,
    mut y: usize,
    mut len: usize,
    grid: &Vec<Vec<Tile>>,
    been: &mut Vec<Vec<bool>>,
) -> usize {
    let mut a: Vec<(usize, usize)> = vec![];
    loop {
        if y == N - 1 {
            for (x, y) in a {
                been[y][x] = false;
            }
            return len;
        }

        len += 1;
        been[y][x] = true;
        a.push((x, y));
        let mut next = vec![];
        if y > 0 && !matches!(grid[y - 1][x], Tile::Wall) && !been[y - 1][x] {
            next.push((x, y - 1));
        }
        if x > 0 && !matches!(grid[y][x - 1], Tile::Wall) && !been[y][x - 1] {
            next.push((x - 1, y));
        }
        if y < N - 1 && !matches!(grid[y + 1][x], Tile::Wall) && !been[y + 1][x] {
            next.push((x, y + 1));
        }
        if x < N - 1 && !matches!(grid[y][x + 1], Tile::Wall) && !been[y][x + 1] {
            next.push((x + 1, y));
        }

        if next.len() == 1 {
            x = next[0].0;
            y = next[0].1;
        } else {
            let mut m = 0;
            for (x, y) in next {
                m = m.max(recurse3(x, y, len, grid, been));
            }

            for (x, y) in a {
                been[y][x] = false;
            }
            return m;
        }
    }
}

fn recurse2(i: Rc<RefCell<Node>>) -> usize {
    if i.borrow().children.is_empty() {
        return 0;
    }
    if i.borrow().children[0].borrow().len == usize::MAX {
        return i.borrow().len;
    }

    let len = i.borrow().len;
    i.borrow()
        .children
        .iter()
        .map(|j| len + recurse2(j.clone()))
        .max()
        .unwrap()
}
