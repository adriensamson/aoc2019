use std::str::FromStr;

pub fn step1(input : &str) {
    let mut signal = Signal::from_str(input);
    for _ in 0..100 {
        signal = signal.apply_phase();
    }
    println!("{}{}{}{}{}{}{}{}", signal.digits[0], signal.digits[1], signal.digits[2], signal.digits[3], signal.digits[4], signal.digits[5], signal.digits[6], signal.digits[7]);
}
pub fn step2(input : &str) {
    let mut real_input = String::new();
    for _ in 0..10_000 {
        real_input.push_str(input);
    }
    let mut signal = Signal::from_str(&real_input);
    for _ in 0..100 {
        signal = signal.apply_phase();
    }
    let offset = usize::from_str(&input[0..7]).unwrap();
    println!("{}{}{}{}{}{}{}{}", signal.digits[offset], signal.digits[offset+1], signal.digits[offset+2], signal.digits[offset+3], signal.digits[offset+4], signal.digits[offset+5], signal.digits[offset+6], signal.digits[offset+7]);
}

struct Signal {
    digits : Vec<i64>,
}

impl Signal {
    fn from_str(s : &str) -> Signal {
        let digits = s.chars().map(|c| i64::from_str(&c.to_string()).unwrap()).collect();
        Signal {digits}
    }

    fn apply_phase(&self) -> Signal {
        let mut digits = Vec::new();
        for n in 0..self.digits.len() {
            digits.push(Signal::mult(&self.digits, &Signal::make_pattern(n, self.digits.len())));
        }
        Signal { digits }
    }

    fn make_pattern(n : usize, size : usize) -> Vec<i64> {
        let n1 = n + 1;
        let base : Vec<i64> = vec![0, 1, 0, -1];
        let mut pattern = Vec::new();
        for i1 in 1..=size {
            pattern.push(base[(i1/n1) % 4]);
        }
        pattern
    }

    fn mult(v1 : &Vec<i64>, v2 : &Vec<i64>) -> i64 {
        let mut sum = 0;
        for i in 0..v1.len() {
            sum += v1[i] * v2[i];
        }
        if sum > 0 {
            sum % 10
        } else {
            -sum % 10
        }
    }
}