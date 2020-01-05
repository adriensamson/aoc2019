use crate::coord::Coord2;
use std::collections::HashSet;

pub fn step1(input : &str) {
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
pub fn step2(_input : &str) {}

struct Map(Vec<Vec<bool>>);

impl Map {
    fn from_str(input : &str) -> Map {
        let mut rows = Vec::new();
        for line in input.trim().lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c == '#');
            }
            rows.push(row);
        }
        Map(rows)
    }

    fn is_infected(&self, c : &Coord2) -> bool {
        *self.0.get(c.y).and_then(|row| row.get(c.x)).unwrap_or(&false)
    }

    fn step(&self) -> Map {
        let mut new_vec = self.0.clone();
        for (y, row) in self.0.iter().enumerate() {
            for (x, &infested) in row.iter().enumerate() {
                let nb_around = Coord2::new(x, y).around().iter().filter(|c| self.is_infected(c)).count();
                if infested && nb_around != 1 {
                    new_vec[y][x] = false;
                }
                if !infested && (nb_around == 1 || nb_around == 2) {
                    new_vec[y][x] = true;
                }
            }
        }
        Map(new_vec)
    }

    fn get_biodiversity_rating(&self) -> usize {
        let mut sum = 0;
        let mut pow = 1;
        for row in &self.0 {
            for &infested in row {
                if infested {
                    sum += pow;
                }
                pow *= 2;
            }
        }
        sum
    }
}