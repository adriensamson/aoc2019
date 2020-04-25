use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub fn step1(input: &str) {
    let reactions = AllReactions::from_str(input);
    let n_ore = reactions
        .get_x_needed_for_y(&String::from("ORE"), &String::from("FUEL"), 1)
        .unwrap();
    println!("{:?}", n_ore);
}
pub fn step2(input: &str) {
    let reactions = AllReactions::from_str(input);
    let n_ore = reactions
        .get_x_needed_for_y(&String::from("ORE"), &String::from("FUEL"), 1)
        .unwrap();
    let avail_ore = 1_000_000_000_000_usize;
    let mut n_fuel_min = avail_ore / n_ore;
    let mut n_fuel_max = n_fuel_min * 2;
    while n_fuel_min < n_fuel_max - 1 {
        let testing = (n_fuel_min + n_fuel_max) / 2;
        let needed = reactions
            .get_x_needed_for_y(&String::from("ORE"), &String::from("FUEL"), testing)
            .unwrap();
        if needed < avail_ore {
            n_fuel_min = testing;
        } else {
            n_fuel_max = testing;
        }
    }
    println!("{:?}", n_fuel_min);
}

lazy_static! {
    static ref ELEMENT_RE: Regex = Regex::new("(\\d+) ([A-Z]+)").unwrap();
    static ref LINE_RE: Regex =
        Regex::new("((\\d+ [A-Z]+, )*\\d+ [A-Z]+) => (\\d+ [A-Z]+)").unwrap();
}

#[derive(Debug)]
struct ReactionElement {
    element: String,
    n: usize,
}

impl ReactionElement {
    fn from_str(s: &str) -> ReactionElement {
        let caps = ELEMENT_RE.captures(s).unwrap();
        ReactionElement {
            element: String::from(&caps[2]),
            n: usize::from_str(&caps[1]).unwrap(),
        }
    }
}

#[derive(Debug)]
struct Reaction {
    output: ReactionElement,
    input: Vec<ReactionElement>,
}

impl Reaction {
    fn from_str(line: &str) -> Reaction {
        let caps = LINE_RE.captures(line).unwrap();
        let output = ReactionElement::from_str(&caps[3]);
        let input = caps[1].split(", ").map(ReactionElement::from_str).collect();
        Reaction { input, output }
    }
}

struct AllReactions {
    reactions: HashMap<String, Reaction>,
}

impl AllReactions {
    fn from_str(input: &str) -> AllReactions {
        let reactions: HashMap<String, Reaction> = input
            .trim()
            .lines()
            .map(Reaction::from_str)
            .map(|r| (r.output.element.clone(), r))
            .collect();
        AllReactions { reactions }
    }

    fn get_x_needed_for_y(&self, x: &str, y: &str, ny: usize) -> Option<usize> {
        let mut needed = HashMap::new();
        needed.insert(y, ny);
        let mut consumed = 0usize;
        let mut remaining = HashMap::new();
        while !needed.is_empty() {
            let el = *needed.keys().take(1).last().unwrap();
            let mut n = needed.remove(el).unwrap();
            let already = *remaining.get(el).unwrap_or(&0);
            if n < already {
                remaining.insert(el, already - n);
                continue;
            }
            remaining.remove(el);
            n -= already;
            match self.reactions.get(el) {
                None => return None,
                Some(r) => {
                    let n_reac = ceil_div(n, r.output.n);
                    for i in &r.input {
                        if i.element == *x {
                            consumed += i.n * n_reac;
                        } else {
                            *needed.entry(&i.element).or_insert(0) += i.n * n_reac;
                        }
                    }
                    if n_reac * r.output.n > n {
                        remaining.insert(el, n_reac * r.output.n - n);
                    }
                }
            }
        }
        Some(consumed)
    }
}

fn ceil_div(x: usize, y: usize) -> usize {
    (x + y - 1) / y
}
