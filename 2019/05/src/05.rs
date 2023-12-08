use intcode::Intcode;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut intcode = Intcode::from(input);
    intcode.run();
    intcode.input(1);
    *intcode.run().last().unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut intcode = Intcode::from(input);
    intcode.run();
    intcode.input(5);
    *intcode.run().last().unwrap()
}
