use std::str::FromStr;

pub fn step1(input : &str) {
    let mut intcode = IntCode::from_str(input);
    intcode.set_at(1, 12);
    intcode.set_at(2, 2);
    intcode.run();

    println!("{}", intcode.get_at(0));
}

pub fn step2(input : &str) {}

#[derive(Debug)]
struct IntCode {
    program : Vec<usize>,
    opi: usize,
}

impl IntCode {
    pub fn from_str(input : &str) -> IntCode {
        let program = input.split(",").map(|s| usize::from_str(s).unwrap()).collect();
        IntCode {
            program,
            opi: 0,
        }
    }

    fn run_op(&mut self) -> bool {
        match self.program[self.opi] {
            1 => {
                self.set_at_opi(self.opi + 3, self.get_at_opi(self.opi + 1) + self.get_at_opi(self.opi + 2));
                self.opi += 4;
                true
            }
            2 => {
                self.set_at_opi(self.opi + 3, self.get_at_opi(self.opi + 1) * self.get_at_opi(self.opi + 2));
                self.opi += 4;
                true
            }
            99 => false,
            opcode => {
                println!("Unknown opcode {}", opcode);
                false
            },
        }
    }

    fn run(&mut self) {
        loop {
            let run= self.run_op();
            if !run {
                break;
            }
        }
    }

    fn get_at(&self, i : usize) -> usize {
        self.program[i]
    }

    fn get_at_opi(&self, opi : usize) -> usize {
        self.get_at(self.get_at(opi))
    }

    fn set_at(&mut self, i : usize, val : usize) {
        self.program[i] = val;
    }

    fn set_at_opi(&mut self, opi : usize, val : usize) {
        let i = self.get_at(opi);
        self.set_at(i, val);
    }

}