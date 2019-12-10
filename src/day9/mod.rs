use crate::intcode::{IntCode, VecPrintIo};
use std::collections::VecDeque;

pub fn step1(input : &str) {
    let mut program = IntCode::from_str(input, VecPrintIo::new(VecDeque::from(vec![1])));
    program.run();
}
pub fn step2(input : &str) {}