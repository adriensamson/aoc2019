use crate::intcode::{IntCodeIo, IntCode, RunState};
use std::io::{stdin, Stdin, Read};
use std::thread::sleep;
use std::time::Duration;

pub fn step1(input : &str) {
    let mut program = IntCode::from_str(input, Tty::create());
    loop {
        let state = program.run();
        if state != RunState::WaitingForInput {
            break;
        }
        sleep(Duration::from_millis(100));
    }
}

struct Tty(Stdin);

impl Tty {
    fn create() -> Tty {
        Tty(stdin())
    }
}

impl IntCodeIo for Tty {
    fn input(&mut self) -> Option<i64> {
        let buf = &mut [0_u8; 1];
        if self.0.read(buf).unwrap() > 0 {
            Some(buf[0] as i64)
        } else {
            None
        }
    }

    fn output(&mut self, val: i64) {
        print!("{}", char::from(val as u8));
    }
}

/*
west
take cake
west
take pointer
west
south
take tambourine
east
east
east
take mug
west
west
west
north
east
south
take monolith
north
east
east
south
take coin
south
west
north
north
drop pointer
drop tambourine
north
*/
