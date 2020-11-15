use crate::coord::coord2u::Coord2u;

#[derive(Clone)]
pub struct Map2u<S> (Vec<Vec<S>>);

impl<S : Copy> Map2u<S> {
    pub fn get(&self, c : Coord2u) -> S {
        self.0[c.y][c.x]
    }

    pub fn get_opt(&self, c : Coord2u) -> Option<S> {
        self.0.get(c.y).and_then(|row| row.get(c.x)).map(|s| *s)
    }

    pub fn set(&mut self, c : Coord2u, s : S) {
        self.0[c.y][c.x] = s;
    }

    pub fn iter(&self) -> impl Iterator<Item = (Coord2u, S)> + '_ {
        self.0.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, s)| (Coord2u::new(x, y), *s))
            })
    }
}

pub trait FromChar {
    fn from_char(c: char) -> Self;
}

impl FromChar for char {
    fn from_char(c: char) -> char {c}
}

impl<S : FromChar> Map2u<S> {
    pub fn from_str(input: &str) -> Map2u<S> {
        let rows = input
            .trim()
            .lines()
            .map(|l| l.chars().map(S::from_char).collect())
            .collect();
        Map2u(rows)
    }
}

