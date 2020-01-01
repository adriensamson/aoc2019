#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Coord2 {
    pub x : usize,
    pub y : usize,
}

impl Coord2 {
    pub fn new(x : usize, y : usize) -> Coord2 {
        Coord2 {x, y}
    }

    pub fn around(&self) -> Vec<Coord2> {
        let mut around = vec![Coord2 {x: self.x + 1, y: self.y}, Coord2 {x: self.x, y: self.y + 1},];
        if self.x > 0 {
            around.push(Coord2 {x: self.x - 1, y : self.y});
        }
        if self.y > 0 {
            around.push(Coord2 {x: self.x, y : self.y - 1});
        }
        around
    }
}
