use crate::coord::coord2u::Coord2u;

#[derive(Clone)]
pub struct Map2u<S> (Vec<Vec<S>>);

impl<S : Copy> Map2u<S> {
    pub fn get(&self, c : Coord2u) -> S {
        self.0[c.y][c.x]
    }

    pub fn get_opt(&self, c : Coord2u) -> Option<S> {
        self.0.get(c.y).and_then(|row| row.get(c.x)).copied()
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

impl<S : From<char>> From<&str> for Map2u<S> {
    fn from(input: &str) -> Map2u<S> {
        let rows = input
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect();
        Map2u(rows)
    }
}

