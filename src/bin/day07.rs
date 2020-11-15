fn main() {
    let input = include_str!("day07input.txt");
    step1(input);
    step2(input);
}

use aoc2019::intcode::{IntCode, RunState, VecVecIo};
use std::cell::RefCell;
use std::collections::VecDeque;

pub fn step1(input: &str) {
    let mut max = 0;
    let mut config = (0, 0, 0, 0, 0);
    for a in 0..5 {
        for b in 0..5 {
            if b == a {
                continue;
            }
            for c in 0..5 {
                if c == a || c == b {
                    continue;
                }
                for d in 0..5 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in 0..5 {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        let signal = run_amp_chain(input, (a, b, c, d, e));
                        if signal > max {
                            max = signal;
                            config = (a, b, c, d, e);
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", config);
    println!("{}", max);
}
pub fn step2(input: &str) {
    let mut max = 0;
    let mut config = (0, 0, 0, 0, 0);
    for a in 5..10 {
        for b in 5..10 {
            if b == a {
                continue;
            }
            for c in 5..10 {
                if c == a || c == b {
                    continue;
                }
                for d in 5..10 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in 5..10 {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        let signal = run_amp_chain2(input, (a, b, c, d, e));
                        if signal > max {
                            max = signal;
                            config = (a, b, c, d, e);
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", config);
    println!("{}", max);
}

fn run_amp(prog: &str, config: i64, input: i64) -> i64 {
    let io_input = RefCell::new(VecDeque::from(vec![config, input]));
    let mut io_output = RefCell::new(VecDeque::new());
    let io = VecVecIo::new(&io_input, &io_output);
    let mut amp = IntCode::from_str(prog, io);
    amp.run();
    io_output.get_mut().pop_front().unwrap()
}

fn run_amp_chain(prog: &str, config: (i64, i64, i64, i64, i64)) -> i64 {
    let a = run_amp(prog, config.0, 0);
    let b = run_amp(prog, config.1, a);
    let c = run_amp(prog, config.2, b);
    let d = run_amp(prog, config.3, c);
    run_amp(prog, config.4, d)
}

fn run_amp_chain2(prog: &str, config: (i64, i64, i64, i64, i64)) -> i64 {
    let mut input_a = RefCell::new(VecDeque::from(vec![config.0, 0]));
    let input_b = RefCell::new(VecDeque::from(vec![config.1]));
    let input_c = RefCell::new(VecDeque::from(vec![config.2]));
    let input_d = RefCell::new(VecDeque::from(vec![config.3]));
    let input_e = RefCell::new(VecDeque::from(vec![config.4]));
    let io_a = VecVecIo::new(&input_a, &input_b);
    let io_b = VecVecIo::new(&input_b, &input_c);
    let io_c = VecVecIo::new(&input_c, &input_d);
    let io_d = VecVecIo::new(&input_d, &input_e);
    let io_e = VecVecIo::new(&input_e, &input_a);
    let mut amp_a = IntCode::from_str(prog, io_a);
    let mut amp_b = IntCode::from_str(prog, io_b);
    let mut amp_c = IntCode::from_str(prog, io_c);
    let mut amp_d = IntCode::from_str(prog, io_d);
    let mut amp_e = IntCode::from_str(prog, io_e);

    loop {
        amp_a.run();
        amp_b.run();
        amp_c.run();
        amp_d.run();
        let status_e = amp_e.run();
        if status_e == RunState::Halted {
            return input_a.get_mut().pop_back().unwrap();
        }
    }
}
