use crate::intcode::{IntCode, IntCodeIo};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

pub fn step1(input: &str) {
    let robot = RefCell::new(Robot::new());
    let io = RobotIo::new(&robot);
    let mut program = IntCode::from_str(input, io);
    program.run();
    println!("{}", robot.borrow().count_painted_panels());
}

pub fn step2(input: &str) {
    let robot = RefCell::new(Robot::new());
    robot.borrow_mut().set_color(true);
    let io = RobotIo::new(&robot);
    let mut program = IntCode::from_str(input, io);
    program.run();
    println!("{}", robot.borrow());
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }
}

struct Robot {
    panels: HashMap<Coord, bool>,
    position: Coord,
    direction: Direction,
}

impl Robot {
    fn new() -> Robot {
        Robot {
            panels: HashMap::new(),
            position: Coord { x: 0, y: 0 },
            direction: Direction::Up,
        }
    }

    fn count_painted_panels(&self) -> usize {
        self.panels.len()
    }

    fn get_color(&self) -> bool {
        *self.panels.get(&self.position).unwrap_or(&false)
    }

    fn set_color(&mut self, color: bool) {
        self.panels.insert(self.position, color);
    }

    fn turn_and_move(&mut self, right: bool) {
        if right {
            self.direction = self.direction.turn_right();
        } else {
            self.direction = self.direction.turn_left();
        }
        self.position.move_in_direction(self.direction);
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let min_x = self.panels.keys().map(|c| c.x).min().unwrap();
        let max_x = self.panels.keys().map(|c| c.x).max().unwrap();
        let min_y = self.panels.keys().map(|c| c.y).min().unwrap();
        let max_y = self.panels.keys().map(|c| c.y).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                write!(
                    f,
                    "{}",
                    if *self.panels.get(&Coord { x, y }).unwrap_or(&false) {
                        "#"
                    } else {
                        " "
                    }
                )?
            }
            write!(f, "\n")?
        }
        Result::Ok(())
    }
}

enum NextOutput {
    Color,
    Turn,
}

struct RobotIo<'a> {
    robot: &'a RefCell<Robot>,
    next_output: NextOutput,
}

impl<'a> RobotIo<'a> {
    fn new(robot: &'a RefCell<Robot>) -> RobotIo<'a> {
        RobotIo {
            robot,
            next_output: NextOutput::Color,
        }
    }
}

impl<'a> IntCodeIo for RobotIo<'a> {
    fn input(&mut self) -> Option<i64> {
        Some(if self.robot.borrow().get_color() {
            1
        } else {
            0
        })
    }

    fn output(&mut self, val: i64) {
        match self.next_output {
            NextOutput::Color => {
                self.robot.borrow_mut().set_color(val == 1);
                self.next_output = NextOutput::Turn;
            }
            NextOutput::Turn => {
                self.robot.borrow_mut().turn_and_move(val == 1);
                self.next_output = NextOutput::Color;
            }
        }
    }
}
