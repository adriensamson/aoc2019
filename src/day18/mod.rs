use std::collections::{HashSet, BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::iter::FromIterator;

pub fn step1(input : &str) {
    let map = &Map::parse(input);
    let graph_map = &GraphMap::from_map(map);

    let shortest = find_shortest_path(graph_map, &map.get_start().unwrap(), &map.get_keys()).unwrap();

    println!("{}", shortest.len);
}
pub fn step2(_input : &str) {}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum State {
    Empty,
    Wall,
    Start,
    Key(char),
    Door(char),
}

impl State {
    fn from_char(c : char) -> State {
        match c {
            '.' => State::Empty,
            '@' => State::Start,
            '#' => State::Wall,
            c if 'a' <= c && c <= 'z' => State::Key(c),
            c if 'A' <= c && c <= 'Z' => State::Door(c.to_ascii_lowercase()),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Coord {
    x : usize,
    y : usize,
}

impl Coord {
    fn around(&self) -> Vec<Coord> {
        let mut around = vec![Coord {x: self.x + 1, y: self.y}, Coord {x: self.x, y: self.y + 1},];
        if self.x > 0 {
            around.push(Coord {x: self.x - 1, y : self.y});
        }
        if self.y > 0 {
            around.push(Coord {x: self.x, y : self.y - 1});
        }
        around
    }
}

struct Map {
    rows : Vec<Vec<State>>,
}

impl Map {
    fn parse(input : &str) -> Map {
        let rows = input.trim().lines()
            .map(|l| l.chars().map(State::from_char).collect())
            .collect();
        Map {rows}
    }

    fn get_start(&self) -> Option<Coord> {
        self.rows.iter()
            .enumerate()
            .find_map(|(y, row)| row.iter().enumerate().find_map(|(x, s)| if *s == State::Start { Some(Coord{x, y})} else {None}))
    }

    fn get_keys(&self) -> HashSet<char> {
        let mut keys = HashSet::new();
        for row in &self.rows {
            for s in row {
                if let State::Key(c) = s {
                    keys.insert(*c);
                }
            }
        }
        keys
    }

    fn get_state_at_coord(&self, c : &Coord) -> Option<State> {
        self.rows.get(c.y).and_then(|row| row.get(c.x)).map(|s| *s)
    }
}

trait ReachableKeys {
    fn get_reachable_nodes(&self, from : &Coord) -> Vec<(Coord, usize, State)>;
    fn get_reachable_keys(&self, from : &Coord, already : &[char]) -> Vec<(char, Coord, usize)> {
        let mut visited = vec![from.clone()];
        let mut current = vec![(from.clone(), 0_usize)];
        let mut found = vec![];
        loop {
            let mut next = HashSet::new();
            for (c, n) in current {
                for (a, d, s) in self.get_reachable_nodes(&c) {
                    if visited.contains(&a) {
                        continue;
                    }
                    visited.push(a);
                    match s {
                        State::Wall => {continue;}
                        State::Empty |
                        State::Start  => {
                            next.insert((a, n + d));
                        },
                        State::Door(key) |
                        State::Key(key) if already.contains(&key) => {
                            next.insert((a, n + d));
                        },
                        State::Door(_) => {},
                        State::Key(key) => {
                            found.push((key, a, n + d));
                        }
                    }
                }
            }
            current = next.iter().map(|c| *c).collect();
            if current.len() == 0 {
                break;
            }
        }
        found
    }
}

impl ReachableKeys for Map {
    fn get_reachable_nodes(&self, from : &Coord) -> Vec<(Coord, usize, State)> {
        let mut n = 0_usize;
        let mut visited = vec![from.clone()];
        let mut current = vec![from.clone()];
        let mut found = vec![];
        loop {
            n += 1;
            let mut next = HashSet::new();
            for c in current {
                for a in &c.around() {
                    if visited.contains(a) {
                        continue;
                    }
                    visited.push(*a);
                    match self.get_state_at_coord(a) {
                        None | Some(State::Wall) => {continue;}
                        Some(State::Empty) => { next.insert(*a); }
                        Some(s) => {
                            found.push((*a, n, s));
                        }
                    }
                }
            }
            current = next.iter().map(|c| *c).collect();
            if current.len() == 0 {
                break;
            }
        }
        found.sort_by_key(|a| a.1);
        found
    }
}

#[derive(Eq)]
struct ExaminedPath {
    c : Coord,
    len : usize,
    keys : Vec<char>,
}

impl ExaminedPath {
    fn get_next_paths(&self, map : &dyn ReachableKeys) -> Vec<ExaminedPath> {
        let mut next = vec![];
        for (key, at, len) in map.get_reachable_keys(&self.c, &self.keys) {
            let mut keys = self.keys.clone();
            keys.push(key);
            next.push(ExaminedPath {c : at, len: self.len + len, keys});
        }
        next
    }

    fn key_set_str(&self) -> String {
        let mut ordered = self.keys.clone();
        ordered.pop();
        ordered.sort();
        ordered.push(*self.keys.last().unwrap());
        String::from_iter(ordered)
    }
}

impl Ord for ExaminedPath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.len.cmp(&other.len).reverse()
    }
}

impl PartialOrd for ExaminedPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ExaminedPath {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len
    }
}

fn find_shortest_path(map : &dyn ReachableKeys, start : &Coord, all_keys : &HashSet<char>) -> Option<ExaminedPath> {
    let mut paths = BinaryHeap::new();
    paths.push(ExaminedPath { c: *start, keys: vec![], len : 0});
    let mut found = BinaryHeap::new();
    let mut preferred_by_key_set : HashMap<String, usize> = HashMap::new();

    loop {
        let ep = &paths.pop().unwrap();
        for next in ep.get_next_paths(map) {
            let key_set = next.key_set_str();
            let min_by_key_set = preferred_by_key_set.entry(key_set).or_insert(next.len + 1);
            println!("{:?} {}", next.keys, next.len);
            if all_keys.len() == next.keys.len() {
                found.push(next);
            } else if next.len < *min_by_key_set {
                *min_by_key_set = next.len;
                paths.push(next);
            } else {
                println!("ignoring {}", *min_by_key_set);
            }
        }
        if paths.len() == 0 {
            break;
        }
        if let (Some(sp), Some(np)) = (found.peek(), paths.peek()) {
            if sp.len < np.len {
                break;
            }
        }
    };

    found.pop()
}

struct GraphMap {
    edges : HashMap<Coord, Vec<(Coord, usize, State)>>,
}

impl GraphMap {
    fn from_map(map : &Map) -> GraphMap {
        let mut edges = HashMap::new();
        let start = map.get_start().unwrap();

        let mut visited = vec![];
        let mut current = vec![start.clone()];

        loop {
            let mut next = HashSet::new();
            for c in current {
                visited.push(c);
                let s = map.get_state_at_coord(&c).unwrap();
                for (c2, d, s2) in map.get_reachable_nodes(&c) {
                    if visited.contains(&c2) {
                        continue;
                    }
                    edges.entry(c).or_insert(Vec::new()).push((c2, d, s2));
                    edges.entry(c2).or_insert(Vec::new()).push((c, d, s));
                    next.insert(c2);
                }
            }
            current = next.iter().map(|c| *c).collect();
            if current.len() == 0 {
                break;
            }
        }

        for e in &mut edges {
            e.1.sort_by_key(| a | a.1);
        }

        GraphMap {edges}
    }
}

impl ReachableKeys for GraphMap {
    fn get_reachable_nodes(&self, from: &Coord) -> Vec<(Coord, usize, State)> {
        self.edges.get(from).unwrap().to_vec()
    }
}