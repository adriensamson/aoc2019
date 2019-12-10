use std::collections::VecDeque;
use std::str::FromStr;

pub trait IntCodeIo {
    fn input(&mut self) -> i64;
    fn output(&mut self, val : i64);
}

pub struct VecPrintIo {
    input_data : VecDeque<i64>,
}

impl VecPrintIo {
    pub fn new(input_data : VecDeque<i64>) -> VecPrintIo {
        VecPrintIo {
            input_data,
        }
    }
}

impl IntCodeIo for VecPrintIo {
    fn input(&mut self) -> i64 {
        self.input_data.pop_front().unwrap()
    }
    fn output(&mut self, val : i64) {
        println!("{}", val);
    }
}

pub struct VecVecIo<'a> {
    input_data: &'a mut VecDeque<i64>,
    output_data: &'a mut VecDeque<i64>,
}

impl<'a> VecVecIo<'a> {
    pub fn new(input_data : &'a mut VecDeque<i64>, output_data : &'a mut VecDeque<i64>) -> VecVecIo<'a> {
        VecVecIo {
            input_data,
            output_data,
        }
    }
}

impl<'a> IntCodeIo for VecVecIo<'a> {
    fn input(&mut self) -> i64 {
        self.input_data.pop_front().unwrap()
    }
    fn output(&mut self, val : i64) {
        self.output_data.push_back(val);
    }
}

#[derive(Debug, Clone)]
pub struct IntCode<Io : IntCodeIo> {
    memory: Vec<i64>,
    ip: usize,
    io: Box<Io>,
}

impl<Io : IntCodeIo> IntCode<Io> {
    pub fn from_str(input : &str, io : Io) -> IntCode<Io> {
        let memory = input.trim().split(",").map(|s| i64::from_str(s).unwrap()).collect();
        IntCode {
            memory,
            ip: 0,
            io: Box::new(io),
        }
    }

    fn run_op(&mut self) -> bool {
        let opcode = self.memory[self.ip] % 100;
        match opcode {
            1 => {
                self.set_at_p(self.ip + 3, self.get_param(1) + self.get_param(2));
                self.ip += 4;
                true
            }
            2 => {
                self.set_at_p(self.ip + 3, self.get_param(1) * self.get_param(2));
                self.ip += 4;
                true
            }
            3 => {
                let val = self.io.input();
                self.set_at_p(self.ip + 1, val);
                self.ip += 2;
                true
            },
            4 => {
                self.io.output(self.get_param(1));
                self.ip += 2;
                true
            },
            5 => {
                if self.get_param(1) != 0 {
                    self.ip = self.get_param(2) as usize;
                } else {
                    self.ip += 3;
                }
                true
            },
            6 => {
                if self.get_param(1) == 0 {
                    self.ip = self.get_param(2) as usize;
                } else {
                    self.ip += 3;
                }
                true
            },
            7 => {
                self.set_at_p(self.ip + 3, if self.get_param(1) < self.get_param(2) { 1 } else { 0 });
                self.ip += 4;
                true
            }
            8 => {
                self.set_at_p(self.ip + 3, if self.get_param(1) == self.get_param(2) { 1 } else { 0 });
                self.ip += 4;
                true
            }
            99 => false,
            opcode => {
                println!("Unknown opcode {}", opcode);
                false
            },
        }
    }

    pub fn run(&mut self) {
        loop {
            let run= self.run_op();
            if !run {
                break;
            }
        }
    }

    fn get_param(&self, i : usize) -> i64 {
        let param_value = self.get_at(self.ip + i);
        let mode_pow = 10 * (10i64).pow(i as u32);
        match (self.memory[self.ip] / mode_pow) % 10 {
            0 => self.get_at(param_value as usize),
            1 => param_value,
            i => panic!("bad param mode {}", i),
        }
    }

    fn get_at(&self, i : usize) -> i64 {
        self.memory[i]
    }

    fn get_at_p(&self, p : usize) -> i64 {
        self.get_at(self.get_at(p) as usize)
    }

    fn set_at(&mut self, i : usize, val : i64) {
        self.memory[i] = val;
    }

    fn set_at_p(&mut self, p : usize, val : i64) {
        let i = self.get_at(p) as usize;
        self.set_at(i, val);
    }

}