use std::collections::{VecDeque, HashMap};
use std::str::FromStr;
use std::cell::RefCell;

pub trait IntCodeIo {
    fn input(&mut self) -> Option<i64>;
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
    fn input(&mut self) -> Option<i64> {
        self.input_data.pop_front()
    }
    fn output(&mut self, val : i64) {
        println!("{}", val);
    }
}

pub struct VecVecIo<'a> {
    input_data: &'a RefCell<VecDeque<i64>>,
    output_data: &'a RefCell<VecDeque<i64>>,
}

impl<'a> VecVecIo<'a> {
    pub fn new(input_data : &'a RefCell<VecDeque<i64>>, output_data : &'a RefCell<VecDeque<i64>>) -> VecVecIo<'a> {
        VecVecIo {
            input_data,
            output_data,
        }
    }
}

impl<'a> IntCodeIo for VecVecIo<'a> {
    fn input(&mut self) -> Option<i64> {
        self.input_data.borrow_mut().pop_front()
    }
    fn output(&mut self, val : i64) {
        self.output_data.borrow_mut().push_back(val);
    }
}

#[derive(Eq, PartialEq)]
pub enum RunState {
    Halted,
    Error,
    WaitingForInput,
}

#[derive(Debug, Clone)]
pub struct IntCode<Io : IntCodeIo> {
    memory: HashMap<usize, i64>,
    ip: usize,
    pub io: Box<Io>,
    relative_base: i64,
}

impl<Io : IntCodeIo> IntCode<Io> {
    pub fn from_str(input : &str, io : Io) -> IntCode<Io> {
        let memory = input.trim().split(",").map(|s| i64::from_str(s).unwrap()).enumerate().collect();
        IntCode {
            memory,
            ip: 0,
            io: Box::new(io),
            relative_base: 0,
        }
    }

    fn run_op(&mut self) -> Option<RunState> {
        let opcode = self.memory[&self.ip] % 100;
        match opcode {
            1 => {
                self.set_param(3, self.get_param(1) + self.get_param(2));
                self.ip += 4;
                None
            }
            2 => {
                self.set_param(3, self.get_param(1) * self.get_param(2));
                self.ip += 4;
                None
            }
            3 => {
                match self.io.input() {
                    None => { return Some(RunState::WaitingForInput); }
                    Some(val) => {
                        self.set_param(1, val);
                        self.ip += 2;
                        None
                    }
                }
            },
            4 => {
                self.io.output(self.get_param(1));
                self.ip += 2;
                None
            },
            5 => {
                if self.get_param(1) != 0 {
                    self.ip = self.get_param(2) as usize;
                } else {
                    self.ip += 3;
                }
                None
            },
            6 => {
                if self.get_param(1) == 0 {
                    self.ip = self.get_param(2) as usize;
                } else {
                    self.ip += 3;
                }
                None
            },
            7 => {
                self.set_param(3, if self.get_param(1) < self.get_param(2) { 1 } else { 0 });
                self.ip += 4;
                None
            }
            8 => {
                self.set_param(3, if self.get_param(1) == self.get_param(2) { 1 } else { 0 });
                self.ip += 4;
                None
            }
            9 => {
                self.relative_base += self.get_param(1);
                self.ip += 2;
                None
            }
            99 => Some(RunState::Halted),
            opcode => {
                println!("Unknown opcode {}", opcode);
                Some(RunState::Error)
            },
        }
    }

    pub fn run(&mut self) -> RunState {
        loop {
            match self.run_op() {
                None => {},
                Some(state) => { return state; }
            }
        }
    }

    fn get_param(&self, i : usize) -> i64 {
        let param_value = self.get_at(self.ip + i);
        let mode_pow = 10 * (10i64).pow(i as u32);
        match (self.memory[&self.ip] / mode_pow) % 10 {
            0 => self.get_at(param_value as usize),
            1 => param_value,
            2 => self.get_at((self.relative_base + param_value) as usize),
            i => panic!("bad param mode {}", i),
        }
    }

    fn set_param(&mut self, i : usize, val : i64) {
        let param_value = self.get_at(self.ip + i);
        let mode_pow = 10 * (10i64).pow(i as u32);
        match (self.memory[&self.ip] / mode_pow) % 10 {
            0 => self.set_at(param_value as usize, val),
            1 => panic!("no set in param mode 1"),
            2 => self.set_at((self.relative_base + param_value) as usize, val),
            i => panic!("bad param mode {}", i),
        }
    }

    fn get_at(&self, i : usize) -> i64 {
        *self.memory.get(&i).unwrap_or(&0i64)
    }

    pub fn set_at(&mut self, i : usize, val : i64) {
        self.memory.insert(i, val);
    }
}