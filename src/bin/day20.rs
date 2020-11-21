fn main() {
    let input = include_str!("day20input.txt");
    step1(input);
    step2(input);
}

use aoc2019::coord::coord2u::Coord2u;
use aoc2019::path_finder::{find_shortest_path, PathState};
use aoc2019::coord::map2u::Map2u;

pub fn step1(input: &str) {
    let map = Map::parse(input);

    let path = find_shortest_path(Path::start(&map));

    println!("{}", path.unwrap().distance);
}
pub fn step2(input: &str) {
    let map = Map::parse(input);

    let path = find_shortest_path(LayeredPath::start(&map));

    println!("{}", path.unwrap().distance);
}

struct Map(Map2u<char>);

impl Map {
    fn parse(input: &str) -> Map {
        Map(Map2u::from(input))
    }

    fn get_at(&self, coord: Coord2u) -> Option<char> {
        self.0.get_opt(coord)
    }

    fn get_start(&self) -> Option<Coord2u> {
        for (c1, _ch) in self.0.iter() {
            if self.get_at(c1) == Some('.') && self.get_portal_name(c1) == Some(('A', 'A')) {
                return Some(c1);
            }
        }
        None
    }

    fn get_portal_name(&self, o: Coord2u) -> Option<(char, char)> {
        if let (Some(c1 @ 'A'..='Z'), Some(c2 @ 'A'..='Z')) = (
            self.get_at(Coord2u::new(o.x + 1, o.y)),
            self.get_at(Coord2u::new(o.x + 2, o.y)),
        ) {
            return Some((c1, c2));
        }
        if let (Some(c1 @ 'A'..='Z'), Some(c2 @ 'A'..='Z')) = (
            self.get_at(Coord2u::new(o.x, o.y + 1)),
            self.get_at(Coord2u::new(o.x, o.y + 2)),
        ) {
            return Some((c1, c2));
        }
        if o.x >= 2 {
            if let (Some(c1 @ 'A'..='Z'), Some(c2 @ 'A'..='Z')) = (
                self.get_at(Coord2u::new(o.x - 2, o.y)),
                self.get_at(Coord2u::new(o.x - 1, o.y)),
            ) {
                return Some((c1, c2));
            }
        }
        if o.y >= 2 {
            if let (Some(c1 @ 'A'..='Z'), Some(c2 @ 'A'..='Z')) = (
                self.get_at(Coord2u::new(o.x, o.y - 2)),
                self.get_at(Coord2u::new(o.x, o.y - 1)),
            ) {
                return Some((c1, c2));
            }
        }

        None
    }

    fn find_portal(&self, name: (char, char), from: Coord2u) -> Option<Coord2u> {
        for (c1, ch) in self.0.iter() {
            if ch == '.' && c1 != from && self.get_portal_name(c1) == Some(name) {
                return Some(c1);
            }
        }
        None
    }

    fn is_outer(&self, c: &Coord2u) -> bool {
        c.y == 2
            || c.x == 2
            || self.get_at(c.bottom().bottom()).is_none()
            || self.get_at(c.right()).is_none()
    }
}

struct Path<'a> {
    map: &'a Map,
    coord: Coord2u,
    distance: usize,
}

impl<'a> Path<'a> {
    fn start(map: &'a Map) -> Path<'a> {
        Path {
            map,
            coord: map.get_start().unwrap(),
            distance: 0,
        }
    }
}

impl PathState for Path<'_> {
    type HashKey = Coord2u;

    fn is_finished(&self) -> bool {
        self.map.get_portal_name(self.coord) == Some(('Z', 'Z'))
    }

    fn get_next_states(&self) -> Vec<Self> {
        let mut next = Vec::new();
        if let Some(portal_name) = self.map.get_portal_name(self.coord) {
            if let Some(portal_out) = self.map.find_portal(portal_name, self.coord) {
                next.push(Path {
                    coord: portal_out,
                    distance: self.distance + 1,
                    map: self.map,
                });
            }
        }
        for o in self.coord.around() {
            if let Some('.') = self.map.get_at(o) {
                next.push(Path {
                    coord: o,
                    distance: self.distance + 1,
                    map: self.map,
                });
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

struct LayeredPath<'a> {
    map: &'a Map,
    coord: Coord2u,
    layer: usize,
    distance: usize,
}

impl<'a> LayeredPath<'a> {
    fn start(map: &'a Map) -> LayeredPath<'a> {
        LayeredPath {
            map,
            coord: map.get_start().unwrap(),
            distance: 0,
            layer: 0,
        }
    }
}

impl PathState for LayeredPath<'_> {
    type HashKey = (usize, Coord2u);

    fn is_finished(&self) -> bool {
        self.layer == 0 && self.map.get_portal_name(self.coord) == Some(('Z', 'Z'))
    }

    fn get_next_states(&self) -> Vec<Self> {
        let mut next = Vec::new();
        if let Some(portal_name) = self.map.get_portal_name(self.coord) {
            if let Some(portal_out) = self.map.find_portal(portal_name, self.coord) {
                if self.map.is_outer(&self.coord) {
                    if self.layer > 0 {
                        next.push(LayeredPath {
                            coord: portal_out,
                            distance: self.distance + 1,
                            map: self.map,
                            layer: self.layer - 1,
                        });
                    }
                } else {
                    next.push(LayeredPath {
                        coord: portal_out,
                        distance: self.distance + 1,
                        map: self.map,
                        layer: self.layer + 1,
                    });
                }
            }
        }
        for o in self.coord.around() {
            if let Some('.') = self.map.get_at(o) {
                next.push(LayeredPath {
                    coord: o,
                    distance: self.distance + 1,
                    map: self.map,
                    layer: self.layer,
                });
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
