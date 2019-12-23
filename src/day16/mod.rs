use std::str::FromStr;

pub fn step1(input : &str) {
    let mut signal = Signal::from_str(input);
    for _ in 0..100 {
        signal = signal.apply_phase();
    }
    println!("{}", signal.get_8_digits(0));
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
    println!("{}", signal.get_8_digits(offset));
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
            digits.push(self.get_new_digit(n));
        }
        Signal { digits }
    }

    fn get_new_digit(&self, n : usize) -> i64 {
        let n1 = n + 1;
        let base : Vec<i64> = vec![0, 1, 0, -1];
        let mut sum = 0;
        let mut i = n;
        while i < self.digits.len() {
            let fact = base[((i + 1)/n1) % 4];
            if fact == 0 {
                i += n1;
                continue;
            }
            sum += self.digits[i] * fact;
            i += 1;
        }
        if sum > 0 {
            sum % 10
        } else {
            -sum % 10
        }
    }

    fn get_8_digits(&self, offset : usize) -> String {
        format!("{}{}{}{}{}{}{}{}", self.digits[offset + 0], self.digits[offset + 1], self.digits[offset + 2], self.digits[offset + 3], self.digits[offset + 4], self.digits[offset + 5], self.digits[offset + 6], self.digits[offset + 7])
    }
}