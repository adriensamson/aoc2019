fn main() {
    let input = include_str!("day04input.txt");
    step1(input);
    step2(input);
}

use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt::Error;
use std::fmt::Formatter;
use std::str::FromStr;

pub fn step1(input: &str) {
    let (min, max) = parse_range(input);
    let mut current = min;
    let mut nb = 0;
    while current < max {
        if current.has_adjacent_same_digits() {
            nb += 1;
        }
        current = current.next();
    }
    println!("{}", nb);
}

pub fn step2(input: &str) {
    let (min, max) = parse_range(input);
    let mut current = min;
    let mut nb = 0;
    while current < max {
        if current.has_adjacent_same_digits2() {
            nb += 1;
        }
        current = current.next();
    }
    println!("{}", nb);
}

fn parse_range(input: &str) -> (SixDigits, SixDigits) {
    let v: Vec<SixDigits> = input.split('-').take(2).map(SixDigits::from_str).collect();
    (v[0], v[1])
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct SixDigits {
    d0: u8,
    d1: u8,
    d2: u8,
    d3: u8,
    d4: u8,
    d5: u8,
}

impl SixDigits {
    fn from_str(s: &str) -> SixDigits {
        SixDigits {
            d0: u8::from_str(&s[0..1]).unwrap(),
            d1: u8::from_str(&s[1..2]).unwrap(),
            d2: u8::from_str(&s[2..3]).unwrap(),
            d3: u8::from_str(&s[3..4]).unwrap(),
            d4: u8::from_str(&s[4..5]).unwrap(),
            d5: u8::from_str(&s[5..6]).unwrap(),
        }
    }

    fn has_adjacent_same_digits(self) -> bool {
        self.d0 == self.d1
            || self.d1 == self.d2
            || self.d2 == self.d3
            || self.d3 == self.d4
            || self.d4 == self.d5
    }

    fn has_adjacent_same_digits2(self) -> bool {
        self.d0 == self.d1 && self.d1 != self.d2
            || self.d1 == self.d2 && self.d0 != self.d1 && self.d2 != self.d3
            || self.d2 == self.d3 && self.d1 != self.d2 && self.d3 != self.d4
            || self.d3 == self.d4 && self.d2 != self.d3 && self.d4 != self.d5
            || self.d4 == self.d5 && self.d3 != self.d4
    }

    fn next(self) -> SixDigits {
        let mut next = self;
        if self.d5 < 9 {
            next.d5 += 1;
        } else if self.d4 < 9 {
            next.d4 += 1;
            next.d5 = next.d4;
        } else if self.d3 < 9 {
            next.d3 += 1;
            next.d4 = next.d3;
            next.d5 = next.d3;
        } else if self.d2 < 9 {
            next.d2 += 1;
            next.d3 = next.d2;
            next.d4 = next.d2;
            next.d5 = next.d2;
        } else if self.d1 < 9 {
            next.d1 += 1;
            next.d2 = next.d1;
            next.d3 = next.d1;
            next.d4 = next.d1;
            next.d5 = next.d1;
        } else if self.d0 < 9 {
            next.d0 += 1;
            next.d1 = next.d0;
            next.d2 = next.d0;
            next.d3 = next.d0;
            next.d4 = next.d0;
            next.d5 = next.d0;
        }

        next
    }
}

impl PartialOrd for SixDigits {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.d0
                .cmp(&other.d0)
                .then(self.d1.cmp(&other.d1))
                .then(self.d2.cmp(&other.d2))
                .then(self.d3.cmp(&other.d3))
                .then(self.d4.cmp(&other.d4))
                .then(self.d5.cmp(&other.d5)),
        )
    }
}

impl ::std::fmt::Display for SixDigits {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}{}{}{}{}{}",
            self.d0, self.d1, self.d2, self.d3, self.d4, self.d5
        )
    }
}
