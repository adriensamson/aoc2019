use std::collections::{HashSet, BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::iter::FromIterator;
use std::hash::Hash;

pub fn step1(input : &str) {
    let map = &Map::parse(input);
    let graph_map = &GraphMap::from_map(map);
    let start = ExaminedPath::new(graph_map, map.get_starts()[0]);
    let shortest = find_shortest_path(start).unwrap();

    println!("{}", shortest.len);
}
pub fn step2(input : &str) {
    let mut map = Map::parse(input);
    map.replace_start();
    let graph_map = &GraphMap::from_map(&map);
    let starts = map.get_starts();
    let start_state = ExaminedPath4::new(graph_map, [starts[0], starts[1], starts[2], starts[3]]);
    let shortest = find_shortest_path(start_state).unwrap();

    println!("{}", shortest.len);
}

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

    fn get_starts(&self) -> Vec<Coord> {
        self.rows.iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().filter_map(move |(x, s)| if *s == State::Start { Some(Coord{x, y})} else {None}))
            .collect()
    }

    fn replace_start(&mut self)  {
        let start = self.get_starts()[0];
        self.rows[start.y - 1][start.x - 1] = State::Start;
        self.rows[start.y - 1][start.x] = State::Wall;
        self.rows[start.y - 1][start.x + 1] = State::Start;
        self.rows[start.y][start.x - 1] = State::Wall;
        self.rows[start.y][start.x] = State::Wall;
        self.rows[start.y][start.x + 1] = State::Wall;
        self.rows[start.y + 1][start.x - 1] = State::Start;
        self.rows[start.y + 1][start.x] = State::Wall;
        self.rows[start.y + 1][start.x + 1] = State::Start;
    }

    fn get_state_at_coord(&self, c : &Coord) -> Option<State> {
        self.rows.get(c.y).and_then(|row| row.get(c.x)).map(|s| *s)
    }

    fn get_all_keys(&self) -> HashSet<char> {
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

    fn has_all_keys(&self, keys : &Vec<char>) -> bool;
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

    fn has_all_keys(&self, keys : &Vec<char>) -> bool {
        self.get_all_keys().len() == keys.len()
    }
}

trait PathState : Ord + Sized {
    type HashKey : Hash + Eq;
    fn is_finished(&self) -> bool;
    fn get_next_states(&self) -> Vec<Self>;
    fn get_hash_key(&self) -> Self::HashKey;
    fn distance(&self) -> usize;
}

fn find_shortest_path<P : PathState>(start : P) -> Option<P> {
    let mut paths = BinaryHeap::new();
    paths.push(start);
    let mut found = BinaryHeap::new();
    let mut preferred_by_hash_key: HashMap<P::HashKey, usize> = HashMap::new();

    loop {
        let ep = &paths.pop().unwrap();
        for next in ep.get_next_states() {
            let hash_key = next.get_hash_key();
            let min_by_has_key = preferred_by_hash_key.entry(hash_key).or_insert(next.distance() + 1);
            if next.is_finished() {
                found.push(next);
            } else if next.distance() < *min_by_has_key {
                *min_by_has_key = next.distance();
                paths.push(next);
            }
        }
        if paths.len() == 0 {
            break;
        }
        if let (Some(sp), Some(np)) = (found.peek(), paths.peek()) {
            if sp.distance() < np.distance() {
                break;
            }
        }
    };

    found.pop()
}

struct ExaminedPath<'a> {
    map : &'a dyn ReachableKeys,
    c : Coord,
    len : usize,
    keys : Vec<char>,
}

impl<'a> ExaminedPath<'a> {
    fn new(map: &'a dyn ReachableKeys, start: Coord) -> ExaminedPath<'a> {
        ExaminedPath {
            map,
            c: start,
            len: 0,
            keys: Vec::new(),
        }
    }
}

impl<'a> PathState for ExaminedPath<'a> {
    type HashKey = String;

    fn is_finished(&self) -> bool {
        self.map.has_all_keys(&self.keys)
    }

    fn get_next_states(&self) -> Vec<ExaminedPath<'a>> {
        let mut next = vec![];
        for (key, at, len) in self.map.get_reachable_keys(&self.c, &self.keys) {
            let mut keys = self.keys.clone();
            keys.push(key);
            next.push(ExaminedPath {map: self.map, c : at, len: self.len + len, keys});
        }
        next
    }

    fn get_hash_key(&self) -> String {
        let mut ordered = self.keys.clone();
        ordered.pop();
        ordered.sort();
        ordered.push(*self.keys.last().unwrap());
        String::from_iter(ordered)
    }

    fn distance(&self) -> usize {
        self.len
    }
}

impl Ord for ExaminedPath<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.len.cmp(&other.len).reverse()
    }
}

impl PartialOrd for ExaminedPath<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ExaminedPath<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len
    }
}

impl Eq for ExaminedPath<'_> {}

struct ExaminedPath4<'a> {
    map : &'a dyn ReachableKeys,
    c : [Coord; 4],
    len : usize,
    previous_keys : Vec<char>,
    current_keys : [char; 4],
}

impl<'a> ExaminedPath4<'a> {
    fn new(map: &'a dyn ReachableKeys, starts: [Coord; 4]) -> ExaminedPath4<'a> {
        ExaminedPath4 {
            map,
            c: starts,
            len: 0,
            previous_keys: Vec::new(),
            current_keys: ['@', '@', '@', '@'],
        }
    }

    fn get_keys(&self) -> Vec<char> {
        let mut keys = self.previous_keys.clone();
        for k in &self.current_keys {
            if *k != '@' {
                keys.push(*k);
            }
        }
        keys
    }
}

impl PathState for ExaminedPath4<'_> {
    type HashKey = String;

    fn is_finished(&self) -> bool {
        self.map.has_all_keys(&self.get_keys())
    }

    fn get_next_states(&self) -> Vec<Self> {
        let mut next = vec![];
        for (i, c) in self.c.iter().enumerate() {
            for (key, at, len) in self.map.get_reachable_keys(c, &self.get_keys()) {
                let mut previous_keys = self.previous_keys.clone();
                if self.current_keys[i] != '@' {
                    previous_keys.push(self.current_keys[i]);
                }
                let mut new_c = self.c.clone();
                new_c[i] = at;
                let mut current_keys = self.current_keys.clone();
                current_keys[i] = key;
                next.push(ExaminedPath4 { map: self.map, c: new_c, len: self.len + len, previous_keys, current_keys });
            }
        }
        next
    }

    fn get_hash_key(&self) -> Self::HashKey {
        let mut ordered = self.previous_keys.clone();
        ordered.sort();
        for k in &self.current_keys {
            ordered.push(*k);
        }
        String::from_iter(ordered)
    }

    fn distance(&self) -> usize {
        self.len
    }
}

impl Ord for ExaminedPath4<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.len.cmp(&other.len).reverse()
    }
}

impl PartialOrd for ExaminedPath4<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ExaminedPath4<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len
    }
}

impl Eq for ExaminedPath4<'_> {}

struct GraphMap {
    edges : HashMap<Coord, Vec<(Coord, usize, State)>>,
    all_keys : HashSet<char>,
}

impl GraphMap {
    fn from_map(map : &Map) -> GraphMap {
        let mut edges = HashMap::new();
        let all_keys = map.get_all_keys();

        let mut current = map.get_starts();
        let mut visited = vec![];

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

        GraphMap {edges, all_keys}
    }
}

impl ReachableKeys for GraphMap {
    fn get_reachable_nodes(&self, from: &Coord) -> Vec<(Coord, usize, State)> {
        self.edges.get(from).unwrap().to_vec()
    }

    fn has_all_keys(&self, keys : &Vec<char>) -> bool {
        self.all_keys.len() == keys.len()
    }
}