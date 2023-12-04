use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut lights = vec![vec![false; 1000]; 1000];
    for i in input.lines() {
        let toggle = i.bytes().nth(1).unwrap() == b'o';
        let turn = i.bytes().nth(6).unwrap() == b'n';
        let [sx, sy, ex, ey] = i.uints::<4, usize>();
        if toggle {
            for x in sx..=ex {
                for y in sy..=ey {
                    lights[x][y] = !lights[x][y];
                }
            }
        } else {
            for x in sx..=ex {
                for y in sy..=ey {
                    lights[x][y] = turn;
                }
            }
        }
    }

    lights.into_iter().flatten().filter(|i| *i).count()
}

fn part_2(input: Input) -> impl ToString {
    let mut lights = vec![vec![0; 1000]; 1000];
    for i in input.lines() {
        let toggle = i.bytes().nth(1).unwrap() == b'o';
        let turn = i.bytes().nth(6).unwrap() == b'n';
        let [sx, sy, ex, ey] = i.uints::<4, usize>();
        if toggle {
            for x in sx..=ex {
                for y in sy..=ey {
                    lights[x][y] += 2;
                }
            }
        } else if turn {
            for x in sx..=ex {
                for y in sy..=ey {
                    lights[x][y] += 1;
                }
            }
        } else {
            for x in sx..=ex {
                for y in sy..=ey {
                    lights[x][y] = (lights[x][y] - 1).max(0);
                }
            }
        }
    }

    lights.into_iter().flatten().sum::<i32>()
}
