use std::str::FromStr;

pub fn step1(input: &str) {
    let mut intcode = IntCode::from_str(input);
    intcode.set_at(1, 12);
    intcode.set_at(2, 2);
    intcode.run();

    println!("{}", intcode.get_at(0));
}

const TARGET: usize = 19690720;

pub fn step2(input: &str) {
    let base = IntCode::from_str(input);
    'outer: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut intcode = base.clone();
            intcode.set_at(1, noun);
            intcode.set_at(2, verb);
            intcode.run();

            if intcode.get_at(0) == TARGET {
                println!("100 * {} + {} = {}", noun, verb, 100 * noun + verb);
                break 'outer;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct IntCode {
    memory: Vec<usize>,
    ip: usize,
}

impl IntCode {
    pub fn from_str(input: &str) -> IntCode {
        let memory = input
            .split(",")
            .map(|s| usize::from_str(s).unwrap())
            .collect();
        IntCode { memory, ip: 0 }
    }

    fn run_op(&mut self) -> bool {
        match self.memory[self.ip] {
            1 => {
                self.set_at_p(
                    self.ip + 3,
                    self.get_at_p(self.ip + 1) + self.get_at_p(self.ip + 2),
                );
                self.ip += 4;
                true
            }
            2 => {
                self.set_at_p(
                    self.ip + 3,
                    self.get_at_p(self.ip + 1) * self.get_at_p(self.ip + 2),
                );
                self.ip += 4;
                true
            }
            99 => false,
            opcode => {
                println!("Unknown opcode {}", opcode);
                false
            }
        }
    }

    fn run(&mut self) {
        loop {
            let run = self.run_op();
            if !run {
                break;
            }
        }
    }

    fn get_at(&self, i: usize) -> usize {
        self.memory[i]
    }

    fn get_at_p(&self, p: usize) -> usize {
        self.get_at(self.get_at(p))
    }

    fn set_at(&mut self, i: usize, val: usize) {
        self.memory[i] = val;
    }

    fn set_at_p(&mut self, p: usize, val: usize) {
        let i = self.get_at(p);
        self.set_at(i, val);
    }
}
