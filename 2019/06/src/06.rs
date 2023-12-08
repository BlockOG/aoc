use rustc_hash::FxHashMap;

aoc::parts!(1, 2);

fn sum(node: &str, i: usize, hm: &FxHashMap<&str, Vec<&str>>) -> usize {
    let mut res = i;
    if hm.contains_key(node) {
        for node in hm[node].iter() {
            res += sum(node, i + 1, hm);
        }
    }
    res
}

fn dfs<'a>(mut node: &'a str, hm: &'a FxHashMap<&str, &str>) -> Vec<&'a str> {
    let mut res = vec![node];
    while node != "COM" {
        node = hm[node];
        res.push(node);
    }
    res
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut hm = FxHashMap::default();
    for line in input.lines() {
        if !hm.contains_key(&line[0..3]) {
            hm.insert(&line[0..3], vec![]);
        }
        hm.get_mut(&line[0..3]).unwrap().push(&line[4..7]);
    }

    sum("COM", 0, &hm)
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut hm = FxHashMap::default();
    for line in input.lines() {
        hm.insert(&line[4..7], &line[0..3]);
    }

    let you = dfs("YOU", &hm);
    let san = dfs("SAN", &hm);

    let res = you
        .into_iter()
        .enumerate()
        .rev()
        .zip(san.into_iter().enumerate().rev())
        .find(|i| i.0 .1 != i.1 .1)
        .unwrap();
    res.0 .0 + res.1 .0
}
