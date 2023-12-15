use rustc_hash::FxHashMap;

aoc::parts!(1, 2);

const N: usize = 100;

fn part_1(input: aoc::Input) -> impl ToString {
    let mut sum = 0;
    let mut c = [N; N];

    let mut y = N - 1;
    for line in input.lines() {
        for (byte, c) in line.bytes().zip(c.iter_mut()) {
            match byte {
                b'#' => {
                    *c = y;
                }
                b'O' => {
                    sum += *c;
                    *c -= 1;
                }
                _ => {}
            }
        }

        y -= 1;
    }

    sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut a = Vec::with_capacity(N);
    let mut b = Vec::with_capacity(N);

    for line in input.lines() {
        let mut arow = 0u128;
        let mut brow = 0u128;
        for byte in line.bytes().rev() {
            arow <<= 1;
            brow <<= 1;

            match byte {
                b'#' => {
                    brow += 1;
                }
                b'O' => {
                    arow += 1;
                }
                _ => {}
            }
        }

        a.push(arow);
        b.push(brow);
    }

    let mut hm = FxHashMap::default();
    let mut tpped = false;

    let mut i = 0;
    while i < 1000000000 {
        let mut c = [0; N];

        for y in 0..N {
            for x in 0..N {
                if b[y] >> x & 1 != 0 {
                    c[x] = y + 1;
                } else if a[y] >> x & 1 != 0 {
                    a[y] ^= 1 << x;
                    a[c[x]] ^= 1 << x;
                    c[x] += 1;
                }
            }
        }

        let mut c = [0; N];

        for y in 0..N {
            for x in 0..N {
                if b[y] >> x & 1 != 0 {
                    c[y] = x + 1;
                } else if a[y] >> x & 1 != 0 {
                    a[y] ^= 1 << x;
                    a[y] ^= 1 << c[y];
                    c[y] += 1;
                }
            }
        }

        let mut cd = [N - 1; N];
        let mut cr = [N - 1; N];

        for y in (0..N).rev() {
            for x in (0..N).rev() {
                if b[y] >> x & 1 != 0 {
                    cd[x] = y - 1;
                } else if a[y] >> x & 1 != 0 {
                    a[y] ^= 1 << x;
                    a[cd[x]] ^= 1 << x;
                    cd[x] -= 1;
                }
            }
        }

        for y in (0..N).rev() {
            for x in (0..N).rev() {
                if b[y] >> x & 1 != 0 {
                    cr[y] = x - 1;
                } else if a[y] >> x & 1 != 0 {
                    a[y] ^= 1 << x;
                    a[y] ^= 1 << cr[y];
                    cr[y] -= 1;
                }
            }
        }

        if !tpped {
            if hm.contains_key(&a) {
                i = 1000000000 - ((1000000000 - i) % (i - hm[&a]));
                tpped = true;
            }

            hm.insert(a.clone(), i);
        }

        i += 1;
    }

    a.into_iter()
        .rev()
        .enumerate()
        .map(|(j, i)| i.count_ones() as usize * (j + 1))
        .sum::<usize>()
}
