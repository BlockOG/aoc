use aoc::{Input, Parse};

aoc::parts!(1, 2);

fn part_1(input: Input) -> impl ToString {
    let mut lights = vec![vec![false; 1000]; 1000];
    for i in input {
        let toggle = i.as_bytes()[1] == b'o';
        let turn = i.as_bytes()[6] == b'n';
        let [sx, sy, ex, ey] = i.uints::<4, usize>();
        if toggle {
            for x in sx..ex + 1 {
                for y in sy..ey + 1 {
                    lights[x][y] = !lights[x][y];
                }
            }
        } else {
            for x in sx..ex + 1 {
                for y in sy..ey + 1 {
                    lights[x][y] = turn;
                }
            }
        }
    }

    lights.into_iter().flatten().filter(|i| *i).count()
}

fn part_2(input: Input) -> impl ToString {
    let mut lights = vec![vec![0; 1000]; 1000];
    for i in input {
        let toggle = i.as_bytes()[1] == b'o';
        let turn = i.as_bytes()[6] == b'n';
        let [sx, sy, ex, ey] = i.uints::<4, usize>();
        if toggle {
            for x in sx..ex + 1 {
                for y in sy..ey + 1 {
                    lights[x][y] += 2;
                }
            }
        } else if turn {
            for x in sx..ex + 1 {
                for y in sy..ey + 1 {
                    lights[x][y] += 1;
                }
            }
        } else {
            for x in sx..ex + 1 {
                for y in sy..ey + 1 {
                    lights[x][y] = (lights[x][y] - 1).max(0);
                }
            }
        }
    }

    lights.into_iter().flatten().sum::<i32>()
}
