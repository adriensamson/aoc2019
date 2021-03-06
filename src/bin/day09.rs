fn main() {
    let input = include_str!("day09input.txt");
    step1(input);
    step2(input);
}

use aoc2019::intcode::{IntCode, VecPrintIo};
use std::collections::VecDeque;

pub fn step1(input: &str) {
    let mut program = IntCode::from_str(input, VecPrintIo::new(VecDeque::from(vec![1])));
    program.run();
}
pub fn step2(input: &str) {
    let mut program = IntCode::from_str(input, VecPrintIo::new(VecDeque::from(vec![2])));
    program.run();
}
