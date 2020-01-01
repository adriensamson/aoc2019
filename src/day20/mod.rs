use crate::coord::Coord2;
use crate::path_finder::{find_shortest_path, PathState};
use std::cmp::Ordering;

pub fn step1(input : &str) {
    let map = Map::parse(input);

    let path = find_shortest_path(Path::start(&map));

    println!("{}", path.unwrap().distance);
}
pub fn step2(input : &str) {
    let map = Map::parse(input);

    let path = find_shortest_path(LayeredPath::start(&map));

    println!("{}", path.unwrap().distance);
}

struct Map(Vec<Vec<char>>);

impl Map {
    fn parse(input : &str) -> Map {
        let mut rows = Vec::new();
        for l in input.lines() {
            let mut row = Vec::new();
            for c in l.chars() {
                row.push(c);
            }
            rows.push(row);
        }
        Map(rows)
    }

    fn get_at(&self, coord: Coord2) -> Option<char> {
        self.0.get(coord.y).and_then(|r| r.get(coord.x).map(|c| *c))
    }

    fn get_start(&self) -> Option<Coord2> {
        for (y, row) in self.0.iter().enumerate() {
            for (x, _ch) in row.iter().enumerate() {
                let c1 = Coord2::new(x, y);
                if self.get_at(c1) == Some('.') && self.get_portal_name(c1) == Some(('A', 'A')) {
                    return Some(c1);
                }
            }
        }
        None
    }

    fn get_portal_name(&self, o : Coord2) -> Option<(char, char)> {
        if let (Some(c1 @ 'A'..='Z'), Some(c2 @ 'A'..='Z')) = (self.get_at(Coord2::new(o.x + 1, o.y)), self.get_at(Coord2::new(o.x + 2, o.y))) {
            return Some((c1, c2));
        }
        if let (Some(c1 @ 'A'..='Z'), Some(c2 @ 'A'..='Z')) = (self.get_at(Coord2::new(o.x, o.y + 1)), self.get_at(Coord2::new(o.x, o.y + 2))) {
            return Some((c1, c2));
        }
        if o.x >= 2 {
            if let (Some(c1 @ 'A'..='Z'), Some(c2 @ 'A'..='Z')) = (self.get_at(Coord2::new(o.x - 2, o.y)), self.get_at(Coord2::new(o.x - 1, o.y))) {
                return Some((c1, c2));
            }
        }
        if o.y >= 2 {
            if let (Some(c1 @ 'A'..='Z'), Some(c2 @ 'A'..='Z')) = (self.get_at(Coord2::new(o.x, o.y - 2)), self.get_at(Coord2::new(o.x, o.y - 1))) {
                return Some((c1, c2));
            }
        }

        None
    }

    fn find_portal(&self, name : (char, char), from : Coord2) -> Option<Coord2> {
        for (y, row) in self.0.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                let c1 = Coord2::new(x, y);
                if *ch == '.' && c1 != from && self.get_portal_name(c1) == Some(name) {
                    return Some(c1);
                }
            }
        }
        None
    }

    fn is_outer(&self, c : &Coord2) -> bool {
        c.y == 2 || c.x == 2 || c.y == self.0.len() - 3 || c.x == self.0[2].len() - 1
    }
}

struct Path<'a> {
    map : &'a Map,
    coord : Coord2,
    distance : usize,
}

impl<'a> Path<'a> {
    fn start(map : &'a Map) -> Path<'a> {
        Path {
            map,
            coord: map.get_start().unwrap(),
            distance: 0,
        }
    }
}

impl PathState for Path<'_> {
    type HashKey = Coord2;

    fn is_finished(&self) -> bool {
        self.map.get_portal_name(self.coord) == Some(('Z', 'Z'))
    }

    fn get_next_states(&self) -> Vec<Self> {
        let mut next = Vec::new();
        if let Some(portal_name) = self.map.get_portal_name(self.coord) {
            if let Some(portal_out) = self.map.find_portal(portal_name, self.coord) {
                next.push(Path { coord: portal_out, distance: self.distance + 1, map: self.map });
            }
        }
        for o in self.coord.around() {
            match self.map.get_at(o) {
                Some('.') => {
                    next.push(Path {coord: o, distance: self.distance + 1, map: self.map});
                }
                _ => {}
            }
        }
        next
    }

    fn get_hash_key(&self) -> Self::HashKey {
        self.coord
    }

    fn distance(&self) -> usize {
        self.distance
    }
}

impl Ord for Path<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialOrd for Path<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Path<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Path<'_> {}

struct LayeredPath<'a> {
    map: &'a Map,
    coord: Coord2,
    layer: usize,
    distance: usize,
}

impl<'a> LayeredPath<'a> {
    fn start(map : &'a Map) -> LayeredPath<'a> {
        LayeredPath {
            map,
            coord: map.get_start().unwrap(),
            distance: 0,
            layer: 0,
        }
    }
}

impl PathState for LayeredPath<'_> {
    type HashKey = (usize, Coord2);

    fn is_finished(&self) -> bool {
        self.layer == 0 && self.map.get_portal_name(self.coord) == Some(('Z', 'Z'))
    }

    fn get_next_states(&self) -> Vec<Self> {
        let mut next = Vec::new();
        if let Some(portal_name) = self.map.get_portal_name(self.coord) {
            if let Some(portal_out) = self.map.find_portal(portal_name, self.coord) {
                if self.map.is_outer(&self.coord) {
                    if self.layer > 0 {
                        next.push(LayeredPath { coord: portal_out, distance: self.distance + 1, map: self.map, layer: self.layer - 1});
                    }
                } else {
                    next.push(LayeredPath { coord: portal_out, distance: self.distance + 1, map: self.map, layer: self.layer + 1 });
                }
            }
        }
        for o in self.coord.around() {
            match self.map.get_at(o) {
                Some('.') => {
                    next.push(LayeredPath {coord: o, distance: self.distance + 1, map: self.map, layer: self.layer});
                }
                _ => {}
            }
        }
        next
    }

    fn get_hash_key(&self) -> Self::HashKey {
        (self.layer, self.coord)
    }

    fn distance(&self) -> usize {
        self.distance
    }
}

impl Ord for LayeredPath<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialOrd for LayeredPath<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LayeredPath<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for LayeredPath<'_> {}
