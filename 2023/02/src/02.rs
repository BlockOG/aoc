use aoc::Parse;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .enumerate()
        .filter(|(_, i)| {
            let mut n = 0;

            for i in i.split(" ").skip(2) {
                if n == 0 {
                    n = i.parse_uw::<i32>();
                } else {
                    if i.starts_with('b') {
                        if n > 14 {
                            return false;
                        }
                    } else if i.starts_with('r') {
                        if n > 12 {
                            return false;
                        }
                    } else {
                        if n > 13 {
                            return false;
                        }
                    }
                    n = 0;
                }
            }
            true
        })
        .map(|i| i.0 + 1)
        .sum::<usize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|i| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            let mut n = 0;

            for i in i.split(" ").skip(2) {
                if n == 0 {
                    n = i.parse_uw::<i32>();
                } else {
                    if i.starts_with('b') {
                        blue = blue.max(n);
                    } else if i.starts_with('r') {
                        red = red.max(n);
                    } else {
                        green = green.max(n);
                    }
                    n = 0;
                }
            }

            red * green * blue
        })
        .sum::<i32>()
}
