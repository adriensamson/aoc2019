
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Coord2u {
    pub x: usize,
    pub y: usize,
}

impl Coord2u {
    pub fn new(x: usize, y: usize) -> Coord2u {
        Coord2u { x, y }
    }

    pub fn left_opt(&self) -> Option<Coord2u> {
        if self.x > 0 {
            Some(Coord2u::new(self.x - 1, self.y))
        } else {
            None
        }
    }

    pub fn left(&self) -> Coord2u {
        self.left_opt().unwrap()
    }

    pub fn right(&self) -> Coord2u {
        Coord2u::new(self.x + 1, self.y)
    }

    pub fn top_opt(&self) -> Option<Coord2u> {
        if self.y > 0 {
            Some(Coord2u::new(self.x, self.y - 1))
        } else {
            None
        }
    }

    pub fn top(&self) -> Coord2u {
        self.top_opt().unwrap()
    }

    pub fn bottom(&self) -> Coord2u {
        Coord2u::new(self.x, self.y + 1)
    }

    pub fn around(&self) -> Vec<Coord2u> {
        let mut around = vec![
            Coord2u {
                x: self.x + 1,
                y: self.y,
            },
            Coord2u {
                x: self.x,
                y: self.y + 1,
            },
        ];
        if self.x > 0 {
            around.push(Coord2u {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            around.push(Coord2u {
                x: self.x,
                y: self.y - 1,
            });
        }
        around
    }
}
