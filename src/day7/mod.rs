use crate::intcode::{IntCode, VecVecIo};
use std::collections::VecDeque;

pub fn step1(input : &str) {
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
pub fn step2(input : &str) {

}

fn run_amp(prog : &str, config : i64, input : i64) -> i64 {
    let mut io_input = VecDeque::from(vec![config, input]);
    let mut io_output = VecDeque::new();
    let io = VecVecIo::new(&mut io_input, &mut io_output);
    let mut amp = IntCode::from_str(prog, io);
    amp.run();
    io_output.pop_front().unwrap()
}

fn run_amp_chain(prog : &str, config : (i64, i64, i64, i64, i64)) -> i64 {
    let a = run_amp(prog, config.0, 0);
    let b = run_amp(prog, config.1, a);
    let c = run_amp(prog, config.2, b);
    let d = run_amp(prog, config.3, c);
    run_amp(prog, config.4, d)
}