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
pub fn step2(input : &str) {
    let size = 119315717514047;

    let mut axpb = input.trim()
        .lines()
        .map(|line| match Action::from_str(line) {
            Action::NewStack => Axpb::new_stack(size),
            Action::Cut(n) => Axpb::cut(n, size),
            Action::DealIncr(n) => Axpb::deal_incr(n),
        })
        .fold(Axpb::ident(), |acc, a| acc.combine(&a, size));

    let mut comb_axpb = Axpb::ident();

    let mut repeats = 101741582076661_usize;
    while repeats > 0 {
        if repeats % 2 == 1 {
            comb_axpb = comb_axpb.combine(&axpb, size);
            repeats -= 1;
        }
        repeats /= 2;
        axpb = axpb.combine(&axpb, size);
    }
    println!("{}", comb_axpb.inverse(2020, size));
}

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

#[derive(Clone)]
struct Axpb {
    a : i128,
    b : i128,
}

impl Axpb {
    fn ident() -> Axpb {
        Axpb {
            a: 1,
            b: 0,
        }
    }

    fn new_stack(size : usize) -> Axpb {
        Axpb {
            a: -1,
            b: (size - 1) as i128,
        }
    }

    fn cut(n : i64, size : usize) -> Axpb {
        let n2 = if n >= 0 {
            n as i128
        } else {
            size as i128 + n as i128
        };
        Axpb {
            a: 1,
            b: -n2
        }
    }

    fn deal_incr(n : usize) -> Axpb {
        Axpb {
            a: n as i128,
            b: 0,
        }
    }

    fn combine(&self, rhs: &Self, size : usize) -> Axpb {
        // a2 * (a1 * x + b1) + b2 = (a1 * a2) * x + (a2 * b1 + b2)
        Axpb {
            a: (self.a * rhs.a) % size as i128,
            b: (rhs.a * self.b + rhs.b) % size as i128
        }
    }

    fn inverse(&self, pos : usize, size : usize) -> usize {
        let size_i128 = size as i128;
        // ax + b = pos
        // ax = pos - b
        // ax = c
        let ac0 = (norm_mod(&self.a, &size_i128), pos as i128 - self.b);
        let ac1 = (size_i128 - ac0.0, size_i128 - ac0.1);
        let mut acs = vec![ac0, ac1];
        loop {
            acs.sort_by_key(|ac| ac.0);
            if acs[0].0 == 1 {
                return norm_mod(&acs[0].1, &size_i128) as usize;
            }
            acs.push((acs[1].0 - acs[0].0, acs[1].1 - acs[0].1));
        }
    }
}

fn norm_mod(a : &i128, size : &i128) -> i128 {
    let m = a % size;
    if m >= 0 {
        m
    } else {
        size + m
    }
}

#[test]
fn axpb_works() {
    let size = 100007;
    let input = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";

    let actions : Vec<Action> = input.trim()
        .lines()
        .map(|line| Action::from_str(line))
        .collect();

    let mut axpb = actions.iter()
        .map(|a| match a {
            Action::NewStack => Axpb::new_stack(size),
            Action::Cut(n) => Axpb::cut(*n, size),
            Action::DealIncr(n) => Axpb::deal_incr(*n),
        })
        .fold(Axpb::ident(), |acc, a| acc.combine(&a, size));

    let mut deck = Deck::new(size);
    for a in &actions {
        match a {
            Action::NewStack => deck.deal_new_stack(),
            Action::Cut(n) => deck.cut(*n),
            Action::DealIncr(n) => deck.deal_with_incr(*n),
        }
    }

    assert_eq!(axpb.inverse(2020, size), deck.0[2020]);
}
