fn main() {
    let input = include_str!("day21input.txt");
    step1(input);
    step2(input);
}

use aoc2019::intcode::{IntCode, IntCodeIo};
use std::collections::VecDeque;
use std::iter::FromIterator;

pub fn step1(input: &str) {
    let mut droid = IntCode::from_str(input, AsciiPrintIo::new());
    // (!A || !B || !C) && D
    // !(A && B && C) && D
    let instructions = "\
OR A J
AND B J
AND C J
NOT J J
AND D J
WALK
";
    let inst_vec = &mut VecDeque::from_iter(instructions.chars().map(|c| c as i64));
    droid.io.input.append(inst_vec);
    droid.run();
}

pub fn step2(input: &str) {
    let mut droid = IntCode::from_str(input, AsciiPrintIo::new());
    // !(A && B && C) && D && (E || H)
    let instructions = "\
OR A J
AND B J
AND C J
NOT J J
AND D J
OR E T
OR H T
AND T J
RUN
";
    let inst_vec = &mut VecDeque::from_iter(instructions.chars().map(|c| c as i64));
    droid.io.input.append(inst_vec);
    droid.run();
}

struct AsciiPrintIo {
    input: VecDeque<i64>,
}

impl AsciiPrintIo {
    fn new() -> AsciiPrintIo {
        AsciiPrintIo {
            input: VecDeque::new(),
        }
    }
}

impl IntCodeIo for AsciiPrintIo {
    fn input(&mut self) -> Option<i64> {
        self.input.pop_front()
    }

    fn output(&mut self, val: i64) {
        if 0 <= val && val <= 255 {
            print!("{}", char::from(val as u8));
        } else {
            println!("{}", val);
        }
    }
}
