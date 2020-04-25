use std::str::FromStr;

pub fn step1(input: &str) -> String {
    let mut intcode = IntCode::from_str(input);
    intcode.set_at(1, 12);
    intcode.set_at(2, 2);
    intcode.run();

    format!("{}", intcode.get_at(0))
}

const TARGET: usize = 19_690_720;

pub fn step2(input: &str) -> String {
    let base = IntCode::from_str(input);
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut intcode = base.clone();
            intcode.set_at(1, noun);
            intcode.set_at(2, verb);
            intcode.run();

            if intcode.get_at(0) == TARGET {
                return format!("100 * {} + {} = {}", noun, verb, 100 * noun + verb);
            }
        }
    }
    panic!("Not found");
}

#[test]
fn test_incode() {
    let mut prog1 = IntCode::from_str("1,0,0,0,99");
    prog1.run();
    assert_eq!(2, prog1.get_at(0));
    let mut prog2 = IntCode::from_str("1,1,1,4,99,5,6,0,99");
    prog2.run();
    assert_eq!(30, prog2.get_at(0));
}

#[derive(Debug, Clone)]
struct IntCode {
    memory: Vec<usize>,
    ip: usize,
}

impl IntCode {
    pub fn from_str(input: &str) -> IntCode {
        let memory = input
            .split(',')
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
