use std::str::FromStr;

pub fn step1(input : &str) {
    let mut signal = Signal::from_str(input);
    for _ in 0..100 {
        signal = signal.apply_phase();
    }
    println!("{}", signal.get_8_digits());
}
pub fn step2(input : &str) {
    let mut real_input = String::new();
    for _ in 0..10_000 {
        real_input.push_str(input);
    }
    let mut signal = Signal::from_str(&real_input);
    let offset = usize::from_str(&input[0..7]).unwrap();
    signal = signal.offset(offset);
    for i in 0..100 {
        println!("{}", i);
        signal = signal.apply_phase();
    }

    println!("{}", signal.get_8_digits());
}

struct Signal {
    digits : Vec<i64>,
    offset : usize,
}

impl Signal {
    fn from_str(s : &str) -> Signal {
        let digits = s.chars().map(|c| i64::from_str(&c.to_string()).unwrap()).collect();
        Signal {digits, offset: 0}
    }

    fn offset(&self, offset : usize) -> Signal {
        Signal {
            offset,
            digits: self.digits[offset..].iter().map(|i| *i).collect(),
        }
    }

    fn apply_phase(&self) -> Signal {
        let mut digits = Vec::new();
        for n in 0..self.digits.len() {
            digits.push(self.get_new_digit(n));
        }
        Signal { digits, offset: self.offset }
    }

    fn get_new_digit(&self, n : usize) -> i64 {
        let n1 = self.offset + n + 1;
        let base : Vec<i64> = vec![0, 1, 0, -1];
        let mut sum = 0;
        let mut i = n;
        while i < self.digits.len() {
            let fact = base[((self.offset + i + 1)/n1) % 4];
            let s : i64 = self.digits[i..].iter().take(n1).sum();
            sum += fact * s;
            i += 2 * n1;
        }
        if sum > 0 {
            sum % 10
        } else {
            -sum % 10
        }
    }

    fn get_8_digits(&self) -> String {
        format!("{}{}{}{}{}{}{}{}", self.digits[0], self.digits[1], self.digits[2], self.digits[3], self.digits[4], self.digits[5], self.digits[6], self.digits[7])
    }
}