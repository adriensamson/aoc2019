fn main() {
    let input = include_str!("day03input.txt");
    step1(input);
    step2(input);
}

use std::str::FromStr;

pub fn step1(input: &str) {
    let paths: Vec<Path> = input
        .lines()
        .take(2)
        .map(|str| Path::from_str(str))
        .collect();
    let mut intersections = paths[0].get_intersections(&paths[1]);
    let origin = Coord { x: 0, y: 0 };
    intersections.sort_by_key(|int| int.dist(&origin));
    println!("{:?}", intersections);
    println!("{}", intersections[1].dist(&origin));
}

pub fn step2(input: &str) {
    let paths: Vec<Path> = input
        .lines()
        .take(2)
        .map(|str| Path::from_str(str))
        .collect();
    let intersections = paths[0].get_intersections(&paths[1]);
    let mut steps: Vec<i64> = intersections
        .iter()
        .map(|int| paths[0].steps_to_coord(int).unwrap() + paths[1].steps_to_coord(int).unwrap())
        .collect();
    steps.sort();
    println!("{:?}", steps);
    println!("{}", steps[1]);
}

struct Path {
    segments: Vec<PathSegment>,
}

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct PathSegment {
    from: Coord,
    dir: Direction,
    len: i64,
}

impl Path {
    fn from_str(input: &str) -> Path {
        let mut current_coord = Coord { x: 0, y: 0 };
        let mut segments = Vec::new();
        for pstr in input.split(',') {
            let segment = current_coord.make_segment(pstr);
            current_coord = segment.get_end();
            segments.push(segment);
        }
        Path { segments }
    }

    fn get_intersections(&self, other: &Self) -> Vec<Coord> {
        let mut vec = Vec::new();
        for s1 in &self.segments {
            for s2 in &other.segments {
                for coord in s1.intersection(s2) {
                    vec.push(coord);
                }
            }
        }
        vec
    }

    fn steps_to_coord(&self, to: &Coord) -> Option<i64> {
        let mut steps = 0;
        for segment in &self.segments {
            if segment.contains(to) {
                steps += segment.from.dist(to);
                return Some(steps);
            }
            steps += segment.len;
        }
        None
    }
}

impl Direction {
    fn from_str(str: &str) -> Direction {
        match &str[0..1] {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!(),
        }
    }
}

impl PathSegment {
    fn get_end(&self) -> Coord {
        match self.dir {
            Direction::Right => Coord {
                x: self.from.x + self.len,
                y: self.from.y,
            },
            Direction::Left => Coord {
                x: self.from.x - self.len,
                y: self.from.y,
            },
            Direction::Up => Coord {
                x: self.from.x,
                y: self.from.y + self.len,
            },
            Direction::Down => Coord {
                x: self.from.x,
                y: self.from.y - self.len,
            },
        }
    }

    fn contains(&self, coord: &Coord) -> bool {
        match self.dir {
            Direction::Right => {
                self.from.y == coord.y
                    && self.from.x <= coord.x
                    && coord.x <= self.from.x + self.len
            }
            Direction::Left => {
                self.from.y == coord.y
                    && self.from.x - self.len <= coord.x
                    && coord.x <= self.from.x
            }
            Direction::Up => {
                self.from.x == coord.x
                    && self.from.y <= coord.y
                    && coord.y <= self.from.y + self.len
            }
            Direction::Down => {
                self.from.x == coord.x
                    && self.from.y - self.len <= coord.y
                    && coord.y <= self.from.y
            }
        }
    }

    fn intersection(&self, other: &Self) -> Vec<Coord> {
        let coords = match (self.dir, other.dir) {
            (Direction::Up, Direction::Right)
            | (Direction::Up, Direction::Left)
            | (Direction::Down, Direction::Right)
            | (Direction::Down, Direction::Left) => vec![Coord {
                x: self.from.x,
                y: other.from.y,
            }],

            (Direction::Left, Direction::Up)
            | (Direction::Left, Direction::Down)
            | (Direction::Right, Direction::Up)
            | (Direction::Right, Direction::Down) => vec![Coord {
                x: other.from.x,
                y: self.from.y,
            }],

            (Direction::Up, Direction::Up)
            | (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Down, Direction::Down) => {
                if self.from.x != other.from.x {
                    vec![]
                } else {
                    let x = self.from.x;
                    let ys = vec![
                        self.from.y,
                        self.from.y - self.len,
                        self.from.y + self.len,
                        other.from.y,
                        other.from.y - other.len,
                        other.from.y + other.len,
                    ];
                    let mut coords = Vec::new();
                    let mut y = *ys.iter().min().unwrap();
                    let ymax = *ys.iter().max().unwrap();
                    while y <= ymax {
                        coords.push(Coord { x, y });
                        y += 1;
                    }
                    coords
                }
            }

            (Direction::Right, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Left, Direction::Right)
            | (Direction::Left, Direction::Left) => {
                if self.from.y != other.from.y {
                    vec![]
                } else {
                    let y = self.from.y;
                    let xs = vec![
                        self.from.x,
                        self.from.x - self.len,
                        self.from.x + self.len,
                        other.from.x,
                        other.from.x - other.len,
                        other.from.x + other.len,
                    ];
                    let mut coords = Vec::new();
                    let mut x = *xs.iter().min().unwrap();
                    let xmax = *xs.iter().max().unwrap();
                    while x <= xmax {
                        coords.push(Coord { x, y });
                        x += 1;
                    }
                    coords
                }
            }
        };
        let mut res = Vec::new();
        for coord in coords {
            if self.contains(&coord) && other.contains(&coord) {
                res.push(coord);
            }
        }
        res
    }
}

impl Coord {
    fn dist(&self, other: &Coord) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn make_segment(&self, pstr: &str) -> PathSegment {
        let len = i64::from_str(&pstr[1..]).unwrap();
        PathSegment {
            from: *self,
            dir: Direction::from_str(pstr),
            len,
        }
    }
}
