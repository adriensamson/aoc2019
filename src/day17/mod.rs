use crate::intcode::{IntCodeIo, IntCode};

pub fn step1(input : &str) {
    let mut program = IntCode::from_str(input, AsciiIo::new());
    program.run();
    println!("{}", program.io);
    println!("{}", program.io.get_calibration());
}

pub fn step2(input : &str) {}

struct AsciiIo {
    rows : Vec<Vec<char>>,
}

impl AsciiIo {
    fn new() -> AsciiIo {
        AsciiIo {
            rows : vec![vec![]],
        }
    }

    fn get_at(&self, i : usize, j : usize) -> Option<char> {
        self.rows.get(i).and_then(|r| r.get(j).map(|c| *c))
    }

    fn get_calibration(&self) -> usize {
        let mut sum = 0;
        for i in 1..self.rows.len()-2 {
            for j in 1..self.rows[i].len()-1 {
                if self.get_at(i, j) == Some('#') {
                    match (self.get_at(i-1, j), self.get_at(i+1, j), self.get_at(i, j-1), self.get_at(i, j+1)) {
                        (Some('#'), Some('#'), Some('#'), Some('#')) => {sum += i * j;}
                        _ => {},
                    }
                }
            }
        }
        sum
    }
}

impl IntCodeIo for AsciiIo {
    fn input(&mut self) -> Option<i64> {
        None
    }

    fn output(&mut self, val: i64) {
        let c = char::from(val as u8);
        if c == '\n' {
            self.rows.push(Vec::new());
        } else {
            self.rows.last_mut().unwrap().push(c);
        }
    }
}

impl std::fmt::Display for AsciiIo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for row in &self.rows {
            for c in row {
                write!(f, "{}", c)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}