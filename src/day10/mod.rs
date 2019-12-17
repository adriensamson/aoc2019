use std::collections::{HashSet, HashMap};

pub fn step1(input : &str) {
    let asteroids = Map::parse(input);
    let max = asteroids.asteroids.iter().map(|a| asteroids.count_seen_from(a)).max();

    println!("{:?}", max.unwrap());
}
pub fn step2(input : &str) {}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x : usize,
    y : usize,
}

#[derive(Debug)]
struct Map {
    asteroids : HashSet<Coord>,
}

impl Map {
    fn parse(input : &str) -> Map {
        let mut asteroids = HashSet::new();

        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    asteroids.insert(Coord {x, y});
                }
            }
        }

        Map {asteroids}
    }

    fn count_seen_from(&self, from : &Coord) -> usize {
        let mut seen = HashMap::new();
        for a in &self.asteroids {
            if a == from {
                continue;
            }
            let vect = Vector::from_coords(from, a);
            let dir = vect.normed_dir();
            match seen.get(&dir) {
                None => { seen.insert(dir, vect); },
                Some(old) => {
                    if old.is_behind(&vect) {
                        seen.insert(dir, vect);
                    }
                },
            }
        }
        seen.len()
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Vector {
    x : i64,
    y : i64,
}

impl Vector {
    fn from_coords(c1 : &Coord, c2 : &Coord) -> Vector {
        Vector {
            x: c2.x as i64 - c1.x as i64,
            y: c2.y as i64 - c1.y as i64,
        }
    }

    fn normed_dir(&self) -> Vector {
        let gcd = ::num::integer::gcd(self.x, self.y);
        Vector {
            x: self.x / gcd,
            y: self.y / gcd,
        }
    }

    fn is_aligned_wih(&self, other : &Self) -> bool {
        self.x * other.y == other.x * self.y
    }

    fn is_behind(&self, other : &Self) -> bool {
        self.is_aligned_wih(other) && if self.x != 0 {other.x / self.x >= 1} else {other.y / self.y >= 1}
    }
}