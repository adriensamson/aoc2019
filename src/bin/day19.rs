fn main() {
    let input = include_str!("day19input.txt");
    step1(input);
    step2(input);
}

use aoc2019::intcode::{IntCode, IntCodeIo};
use std::collections::VecDeque;

pub fn step1(input: &str) {
    let beam_program = BeamProgram::new(input);
    let mut n = 0;
    for y in 0..50 {
        for x in 0..50 {
            if beam_program.is_tracted(x, y) {
                n += 1;
            }
        }
    }
    println!("{}", n);
}
pub fn step2(input: &str) {
    let beam_program = BeamProgram::new(input);
    let mut y = 50;
    let mut x = 0;
    while !beam_program.is_tracted(x, y) {
        x += 1;
    }
    'y: loop {
        loop {
            let right100 = beam_program.is_tracted(x + 99, y);
            let bottom100 = beam_program.is_tracted(x, y + 99);
            match (right100, bottom100) {
                (true, true) => break 'y,
                (false, false) => {
                    if beam_program.is_tracted(x, y + 1) {
                        y += 1;
                    } else if beam_program.is_tracted(x + 1, y) {
                        x += 1;
                    } else {
                        panic!("not tracted neither right or bottom of {} {}", x, y);
                    }
                }
                (true, false) => {
                    if !beam_program.is_tracted(x + 1, y) {
                        panic!("not tracted at right of {} {}", x, y);
                    }
                    x += 1;
                }
                (false, true) => {
                    if !beam_program.is_tracted(x, y + 1) {
                        panic!("not tracted at bottom of {} {}", x, y);
                    }
                    y += 1;
                }
            }
        }
    }

    println!("{}", x * 10000 + y);
}

#[derive(Clone)]
struct BeamIo {
    input: VecDeque<i64>,
    output: Option<i64>,
}

impl BeamIo {
    fn new() -> BeamIo {
        BeamIo {
            input: VecDeque::new(),
            output: None,
        }
    }

    fn set_coord(&mut self, x: i64, y: i64) {
        self.input.truncate(0);
        self.input.push_back(x);
        self.input.push_back(y);
    }

    fn read_output(&mut self) -> Option<i64> {
        let o = self.output;
        self.output = None;
        o
    }
}

impl IntCodeIo for BeamIo {
    fn input(&mut self) -> Option<i64> {
        self.input.pop_front()
    }

    fn output(&mut self, val: i64) {
        self.output = Some(val);
    }
}

struct BeamProgram {
    base_program: IntCode<BeamIo>,
}

impl BeamProgram {
    fn new(input: &str) -> BeamProgram {
        BeamProgram {
            base_program: IntCode::from_str(input, BeamIo::new()),
        }
    }

    fn is_tracted(&self, x: i64, y: i64) -> bool {
        let mut p = self.base_program.clone();
        p.io.set_coord(x, y);
        p.run();
        p.io.read_output().unwrap() == 1
    }
}
