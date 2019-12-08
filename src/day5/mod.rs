use std::str::FromStr;

pub fn step1(input : &str) {
    let mut program = IntCode::from_str(input, VecPrintIo::new(vec![1]));
    program.run();
}

pub fn step2(input : &str) {

}


trait IntCodeIo {
    fn input(&mut self) -> i64;
    fn output(&mut self, val : i64);
}

struct VecPrintIo {
    input_data : Vec<i64>,
    input_ptr : usize,
}

impl VecPrintIo {
    fn new(input_data : Vec<i64>) -> VecPrintIo {
        VecPrintIo {
            input_data,
            input_ptr: 0,
        }
    }
}

impl IntCodeIo for VecPrintIo {
    fn input(&mut self) -> i64 {
        let val = self.input_data[self.input_ptr];
        self.input_ptr += 1;
        val
    }
    fn output(&mut self, val : i64) {
        println!("{}", val);
    }
}

#[derive(Debug, Clone)]
struct IntCode<Io : IntCodeIo> {
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