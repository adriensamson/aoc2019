use std::str::FromStr;
use regex::Regex;

lazy_static!{
    static ref NEW_STACK_RE : Regex = Regex::new("deal into new stack").unwrap();
    static ref CUT_RE : Regex = Regex::new("cut (-?\\d+)").unwrap();
    static ref DEAL_INCR_RE : Regex = Regex::new("deal with increment (\\d+)").unwrap();
}

pub fn step1(input : &str) {
    let mut deck = Deck::new(10_007);
    for line in input.trim().lines() {
        match Action::from_str(line) {
            Action::NewStack => deck.deal_new_stack(),
            Action::Cut(n) => deck.cut(n),
            Action::DealIncr(n) => deck.deal_with_incr(n),
        }
    }
    println!("{}", deck.0.iter().enumerate().find_map(|(i, &c)| if c == 2019 { Some(i) } else { None }).unwrap());
}
pub fn step2(input : &str) {}

enum Action {
    NewStack,
    Cut(i64),
    DealIncr(usize),
}

impl Action {
    fn from_str(line: &str) -> Action {
        if let Some(_) = NEW_STACK_RE.captures(line) {
            return Action::NewStack;
        }
        if let Some(caps) = CUT_RE.captures(line) {
            return Action::Cut(i64::from_str(&caps[1]).unwrap());
        }
        if let Some(caps) = DEAL_INCR_RE.captures(line) {
            return Action::DealIncr(usize::from_str(&caps[1]).unwrap());
        }
        panic!()
    }
}

struct Deck(Vec<usize>);

impl Deck {
    fn new(size : usize) -> Deck {
        let vec = (0..size).into_iter().collect();
        Deck(vec)
    }

    fn deal_new_stack(&mut self) {
        self.0.reverse();
    }

    fn cut(&mut self, n : i64) {
        let n2 = if n >= 0 {
            n as usize
        } else {
            self.0.len() - (-n as usize)
        };
        let (first, last) = self.0.split_at(n2);
        let mut new = Vec::from(last);
        new.append(&mut Vec::from(first));
        self.0 = new;
    }

    fn deal_with_incr(&mut self, n : usize) {
        let len = self.0.len();
        let mut new = vec![0; len];
        for (i, c) in self.0.iter().enumerate() {
            new[(i * n) % len] = *c;
        }
        self.0 = new;
    }
}