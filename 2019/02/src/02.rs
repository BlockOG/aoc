use intcode::Intcode;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut intcode = Intcode::from(input);
    intcode.memory[1] = 12;
    intcode.memory[2] = 2;
    intcode.run();
    intcode.memory[0]
}

fn part_2(input: aoc::Input) -> impl ToString {
    let start = Intcode::from(input);
    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode = start.clone();
            intcode.memory[1] = noun;
            intcode.memory[2] = verb;
            intcode.run();
            if intcode.memory[0] == 19690720 {
                return noun * 100 + verb;
            }
        }
    }

    unreachable!()
}
