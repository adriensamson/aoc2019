use std::collections::{HashMap, HashSet};
use std::f64;

pub fn step1(input: &str) {
    let asteroids = Map::parse(input);
    let max = asteroids
        .asteroids
        .iter()
        .map(|a| asteroids.count_seen_from(a))
        .max();

    println!("{:?}", max.unwrap());
}

pub fn step2(input: &str) {
    let asteroids = Map::parse(input);
    let max = asteroids
        .asteroids
        .iter()
        .map(|a| (a, asteroids.count_seen_from(a)))
        .max_by_key(|(_a, d)| *d);
    let station = max.unwrap().0;

    let mut ast_by_dir = HashMap::new();
    for a in &asteroids.asteroids {
        if a == station {
            continue;
        }
        let vect = Vector::from_coords(station, a);
        let (dir, dist) = vect.normed_dir();
        ast_by_dir
            .entry(dir)
            .or_insert_with(Vec::new)
            .push((dist, a));
    }
    for by_angle in ast_by_dir.values_mut() {
        by_angle.sort_by_key(|ba| ba.0);
    }
    let mut asts: Vec<(usize, &Vector, &Coord)> = ast_by_dir
        .iter()
        .flat_map(|(dir, by_angle)| {
            by_angle
                .iter()
                .enumerate()
                .map(move |(i, ba)| (i, dir, ba.1))
        })
        .collect();
    asts.sort_by(|a1, a2| {
        a1.0.cmp(&a2.0)
            .then(a1.1.angle().partial_cmp(&a2.1.angle()).unwrap())
    });
    let nth200 = asts[199].2;
    println!("{}", 100 * nth200.x + nth200.y);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    asteroids: HashSet<Coord>,
}

impl Map {
    fn parse(input: &str) -> Map {
        let mut asteroids = HashSet::new();

        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    asteroids.insert(Coord { x, y });
                }
            }
        }

        Map { asteroids }
    }

    fn count_seen_from(&self, from: &Coord) -> usize {
        let mut seen = HashMap::new();
        for a in &self.asteroids {
            if a == from {
                continue;
            }
            let vect = Vector::from_coords(from, a);
            let dir = vect.normed_dir().0;
            match seen.get(&dir) {
                None => {
                    seen.insert(dir, vect);
                }
                Some(old) => {
                    if old.is_behind(&vect) {
                        seen.insert(dir, vect);
                    }
                }
            }
        }
        seen.len()
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Vector {
    x: i64,
    y: i64,
}

impl Vector {
    fn from_coords(c1: &Coord, c2: &Coord) -> Vector {
        Vector {
            x: c2.x as i64 - c1.x as i64,
            y: c2.y as i64 - c1.y as i64,
        }
    }

    fn normed_dir(&self) -> (Vector, i64) {
        let gcd = ::num::integer::gcd(self.x, self.y);
        (
            Vector {
                x: self.x / gcd,
                y: self.y / gcd,
            },
            gcd,
        )
    }

    fn is_aligned_wih(&self, other: &Self) -> bool {
        self.x * other.y == other.x * self.y
    }

    fn is_behind(&self, other: &Self) -> bool {
        self.is_aligned_wih(other)
            && if self.x != 0 {
                other.x / self.x >= 1
            } else {
                other.y / self.y >= 1
            }
    }

    fn angle(&self) -> f64 {
        let a = (self.x as f64).atan2(-self.y as f64);
        (2f64 * f64::consts::PI + a) % (2f64 * f64::consts::PI)
    }
}
