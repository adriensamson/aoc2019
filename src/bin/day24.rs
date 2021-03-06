fn main() {
    let input = include_str!("day24input.txt");
    step1(input);
    step2(input);
}

use aoc2019::coord::coord2u::Coord2u;
use std::collections::{HashMap, HashSet};
use aoc2019::coord::map2u::Map2u;

pub fn step1(input: &str) {
    let mut seen = HashSet::new();
    let mut map = Map::from_str(input);
    seen.insert(map.get_biodiversity_rating());
    loop {
        map = map.step();
        let bio = map.get_biodiversity_rating();
        if seen.contains(&bio) {
            println!("{}", bio);
            break;
        }
        seen.insert(bio);
    }
}
pub fn step2(input: &str) {
    let mut map = LevelMap::from_str(input);
    for _i in 0..200 {
        map = map.step();
    }
    let count = map.0.values().filter(|&&b| b).count();
    println!("{}", count);
}

#[derive(Copy, Clone)]
struct Infected(bool);
struct Map(Map2u<Infected>);

impl From<char> for Infected {
    fn from(c: char) -> Self {
        Infected(c == '#')
    }
}

impl Map {
    fn from_str(input: &str) -> Map {
        Map(Map2u::from(input))
    }

    fn is_infected(&self, c: Coord2u) -> bool {
        self
            .0
            .get_opt(c)
            .map(|inf| inf.0)
            .unwrap_or(false)
    }

    fn step(&self) -> Map {
        let mut new_map = self.0.clone();
        for (coord, infected) in self.0.iter() {
            let nb_around = coord
                .around()
                .iter()
                .filter(|c| self.is_infected(**c))
                .count();
            if infected.0 && nb_around != 1 {
                new_map.set(coord, Infected(false));
            }
            if !infected.0 && (nb_around == 1 || nb_around == 2) {
                new_map.set(coord, Infected(true));
            }
        }
        Map(new_map)
    }

    fn get_biodiversity_rating(&self) -> usize {
        let mut sum = 0;
        let mut pow = 1;
        for (_, infected) in self.0.iter() {
            if infected.0 {
                sum += pow;
            }
            pow *= 2;
        }
        sum
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct LevelCoord {
    level: i64,
    x: usize,
    y: usize,
}

impl LevelCoord {
    fn level0(x: usize, y: usize) -> LevelCoord {
        LevelCoord { x, y, level: 0 }
    }

    fn around(&self) -> Vec<LevelCoord> {
        let mut res = Vec::new();
        // Left
        match self.x {
            0 => res.push(LevelCoord {
                y: 2,
                x: 1,
                level: self.level - 1,
            }),
            3 if self.y == 2 => {
                for y in 0..5 {
                    res.push(LevelCoord {
                        y,
                        x: 4,
                        level: self.level + 1,
                    });
                }
            }
            _ => res.push(LevelCoord {
                y: self.y,
                x: self.x - 1,
                level: self.level,
            }),
        }
        // Right
        match self.x {
            4 => res.push(LevelCoord {
                y: 2,
                x: 3,
                level: self.level - 1,
            }),
            1 if self.y == 2 => {
                for y in 0..5 {
                    res.push(LevelCoord {
                        y,
                        x: 0,
                        level: self.level + 1,
                    });
                }
            }
            _ => res.push(LevelCoord {
                y: self.y,
                x: self.x + 1,
                level: self.level,
            }),
        }
        // Top
        match self.y {
            0 => res.push(LevelCoord {
                y: 1,
                x: 2,
                level: self.level - 1,
            }),
            3 if self.x == 2 => {
                for x in 0..5 {
                    res.push(LevelCoord {
                        y: 4,
                        x,
                        level: self.level + 1,
                    });
                }
            }
            _ => res.push(LevelCoord {
                y: self.y - 1,
                x: self.x,
                level: self.level,
            }),
        }
        // Bottom
        match self.y {
            4 => res.push(LevelCoord {
                y: 3,
                x: 2,
                level: self.level - 1,
            }),
            1 if self.x == 2 => {
                for x in 0..5 {
                    res.push(LevelCoord {
                        y: 0,
                        x,
                        level: self.level + 1,
                    });
                }
            }
            _ => res.push(LevelCoord {
                y: self.y + 1,
                x: self.x,
                level: self.level,
            }),
        }
        res
    }
}

struct LevelMap(HashMap<LevelCoord, bool>);

impl LevelMap {
    fn from_str(input: &str) -> LevelMap {
        let mut map = HashMap::new();
        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                map.insert(LevelCoord::level0(x, y), c == '#');
            }
        }
        LevelMap(map)
    }

    fn is_infected(&self, c: &LevelCoord) -> bool {
        *self.0.get(c).unwrap_or(&false)
    }

    fn step(&self) -> LevelMap {
        let mut new_map = self.0.clone();
        let mut to_check = HashSet::new();
        for c in self.0.keys() {
            to_check.insert(*c);
            for c1 in c.around() {
                to_check.insert(c1);
            }
        }
        for c in to_check {
            let infested = *self.0.get(&c).unwrap_or(&false);
            let nb_around = c.around().iter().filter(|c| self.is_infected(c)).count();
            if infested && nb_around != 1 {
                new_map.insert(c, false);
            }
            if !infested && (nb_around == 1 || nb_around == 2) {
                new_map.insert(c, true);
            }
        }
        LevelMap(new_map)
    }
}
