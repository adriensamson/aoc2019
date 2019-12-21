use crate::intcode::{IntCode, VecVecIo, IntCodeIo};
use std::collections::{HashMap, VecDeque};
use std::cell::RefCell;

pub fn step1(input : &str) {
    let mut droid = RepairDroid::create(input);
    println!("{:?}", droid.explore(0));
}
pub fn step2(input : &str) {}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn from_i64(i : i64) -> Direction {
        match i {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::East,
            _ => panic!(),
        }
    }

    fn to_i64(&self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

impl std::ops::Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Coord {
    x : i64,
    y : i64,
}

impl Coord {
    fn add(&self, dir : Direction) -> Coord {
        match dir {
            Direction::North => Coord {x: self.x, y: self.y - 1},
            Direction::South => Coord {x: self.x, y: self.y + 1},
            Direction::West => Coord {x: self.x - 1, y: self.y},
            Direction::East => Coord {x: self.x + 1, y: self.y},
        }
    }

    fn assign_add(&mut self, dir : Direction) {
        match dir {
            Direction::North => self.y -= 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
            Direction::East => self.x += 1,
        }
    }
}

#[derive(Eq, PartialEq)]
enum State {
    Empty,
    Wall,
    Oxygen,
}

struct ManualIo {
    input : Option<i64>,
    output : Option<i64>,
}

impl ManualIo {
    fn new() -> ManualIo {
        ManualIo {
            input: None,
            output: None,
        }
    }
}

impl IntCodeIo for ManualIo {
    fn input(&mut self) -> Option<i64> {
        let val = self.input;
        self.input = None;
        val
    }

    fn output(&mut self, val: i64) {
        self.output = Some(val);
    }
}

impl IntCode<ManualIo> {
    fn set_input(&mut self, val : i64) {
        self.io.input = Some(val);
    }

    fn get_output(&self) -> Option<i64> {
        self.io.output
    }
}

struct RepairDroid {
    program : IntCode<ManualIo>,
    position : Coord,
    map : HashMap<Coord, State>
}

impl RepairDroid {
    fn create(s : &str) -> RepairDroid {
        let program = IntCode::from_str(s, ManualIo::new());
        RepairDroid {
            program,
            position: Coord {x: 0, y: 0},
            map: HashMap::new(),
        }
    }

    fn try_move(&mut self, dir : Direction) -> bool {
        self.program.set_input(dir.to_i64());
        self.program.run();
        match self.program.get_output().unwrap() {
            0 => {
                self.map.insert(self.position.add(dir), State::Wall);
                false
            },
            1 => {
                self.position.assign_add(dir);
                self.map.insert(self.position, State::Empty);
                true
            },
            2 => {
                self.position.assign_add(dir);
                self.map.insert(self.position, State::Oxygen);
                true
            }
            _ => panic!(),
        }
    }

    fn get_oxygen_coord(&self) -> Option<Coord> {
        self.map.iter().find(|(_, s)| **s == State::Oxygen).map(|(c, _)| *c)
    }

    fn is_explored(&self, dir : Direction) -> bool {
        self.map.contains_key(&self.position.add(dir))
    }

    fn explore(&mut self, depth : usize) -> Option<usize> {
        let dirs = vec![Direction::North, Direction::West, Direction::South, Direction::East];
        dirs.iter()
            .filter_map(|&dir| {
                if self.is_explored(dir) {
                    return None;
                }
                if self.try_move(dir) {
                    if self.get_oxygen_coord() != None {
                        return Some(depth + 1);
                    }
                    let d = self.explore(depth + 1);
                    self.try_move(-dir);
                    d
                } else {
                    None
                }
            })
            .min()
    }
}