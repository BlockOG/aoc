use aoc::{Input, Parse};

#[derive(Clone)]
pub enum State {
    Running,
    Halted,
    Input(isize),
}

#[derive(Clone)]
pub struct Intcode {
    pub memory: Vec<isize>,
    pub pointer: usize,
    pub state: State,
    pub relative_base: isize,
}

impl Intcode {
    pub fn new(memory: Vec<isize>) -> Self {
        Self {
            memory,
            pointer: 0,
            state: State::Running,
            relative_base: 0,
        }
    }

    pub fn run(&mut self) -> Vec<isize> {
        let mut res = vec![];
        while let State::Running = self.state {
            if let Some(output) = self.run_instruction() {
                res.push(output);
            }
        }
        res
    }

    pub fn run_instruction(&mut self) -> Option<isize> {
        let opcode = self.get() % 100;
        let parameter_mode = self.get() / 100;
        match opcode {
            1 => {
                self.parameter_set::<3>(
                    parameter_mode,
                    self.parameter::<1>(parameter_mode) + self.parameter::<2>(parameter_mode),
                );
                self.pointer += 4;
            }
            2 => {
                self.parameter_set::<3>(
                    parameter_mode,
                    self.parameter::<1>(parameter_mode) * self.parameter::<2>(parameter_mode),
                );
                self.pointer += 4;
            }
            3 => {
                self.state = State::Input(match parameter_mode % 10 {
                    0 => self.relative(1),
                    2 => self.relative_base + self.relative(1),
                    _ => unreachable!(),
                });
                self.pointer += 2;
            }
            4 => {
                let output = self.parameter::<1>(parameter_mode);
                self.pointer += 2;
                return Some(output);
            }
            5 => {
                if self.parameter::<1>(parameter_mode) != 0 {
                    self.pointer = self.parameter::<2>(parameter_mode) as usize;
                } else {
                    self.pointer += 3;
                }
            }
            6 => {
                if self.parameter::<1>(parameter_mode) == 0 {
                    self.pointer = self.parameter::<2>(parameter_mode) as usize;
                } else {
                    self.pointer += 3;
                }
            }
            7 => {
                self.parameter_set::<3>(
                    parameter_mode,
                    (self.parameter::<1>(parameter_mode) < self.parameter::<2>(parameter_mode))
                        as isize,
                );
                self.pointer += 4;
            }
            8 => {
                self.parameter_set::<3>(
                    parameter_mode,
                    (self.parameter::<1>(parameter_mode) == self.parameter::<2>(parameter_mode))
                        as isize,
                );
                self.pointer += 4;
            }
            9 => {
                self.relative_base += self.parameter::<1>(parameter_mode);
                self.pointer += 2;
            }
            99 => {
                self.state = State::Halted;
                self.pointer += 1;
            }
            _ => todo!(),
        }
        None
    }

    pub fn input(&mut self, value: isize) {
        if let State::Input(i) = self.state {
            self.set_direct(i as usize, value);
            self.state = State::Running;
        }
    }

    fn parameter<const N: usize>(&self, parameter_mode: isize) -> isize {
        match parameter_mode / 10isize.pow(N as u32 - 1) % 10 {
            0 => self.follow_relative(N),
            1 => self.relative(N),
            2 => self.get_direct((self.relative_base + self.relative(N)) as usize),
            _ => unreachable!(),
        }
    }

    fn parameter_set<const N: usize>(&mut self, parameter_mode: isize, value: isize) {
        match parameter_mode / 10isize.pow(N as u32 - 1) % 10 {
            0 => self.follow_relative_set(N, value),
            2 => self.set_direct((self.relative_base + self.relative(N)) as usize, value),
            _ => unreachable!(),
        }
    }

    fn get_direct(&self, index: usize) -> isize {
        if index < self.memory.len() {
            self.memory[index as usize]
        } else {
            0
        }
    }

    fn set_direct(&mut self, index: usize, value: isize) {
        while self.memory.len() <= index {
            self.memory.push(0);
        }
        self.memory[index] = value;
    }

    fn get(&self) -> isize {
        self.get_direct(self.pointer)
    }

    fn set(&mut self, value: isize) {
        self.set_direct(self.pointer, value);
    }

    fn relative(&self, offset: usize) -> isize {
        self.get_direct(self.pointer + offset)
    }

    fn relative_set(&mut self, offset: usize, value: isize) {
        self.set_direct(self.pointer + offset, value);
    }

    fn follow(&self) -> isize {
        self.get_direct(self.get() as usize)
    }

    fn follow_set(&mut self, value: isize) {
        self.set_direct(self.get() as usize, value);
    }

    fn follow_relative(&self, offset: usize) -> isize {
        self.get_direct(self.relative(offset) as usize)
    }

    fn follow_relative_set(&mut self, offset: usize, value: isize) {
        self.set_direct(self.relative(offset) as usize, value);
    }
}

impl From<Input<'_>> for Intcode {
    fn from(value: Input) -> Self {
        Self::new(value.raw().ints_iter().collect())
    }
}
