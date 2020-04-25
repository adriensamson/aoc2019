use crate::intcode::{IntCode, IntCodeIo};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

pub fn step1(input: &str) {
    let screen = RefCell::new(Screen::new());
    let mut program = IntCode::from_str(input, ScreenIo::new(&screen));
    program.run();
    println!("{}", screen.borrow());

    let nb_block = screen
        .borrow()
        .tiles
        .values()
        .filter(|&&t| t == Tile::Block)
        .count();
    println!("{}", nb_block);
}

pub fn step2(input: &str) {
    let screen = RefCell::new(Screen::new());
    let mut program = IntCode::from_str(input, ScreenIo::new(&screen));
    program.set_at(0, 2);
    program.run();
    println!("{}", screen.borrow());
    println!("{}", screen.borrow().get_score());
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HPaddle,
    Ball,
}

impl Tile {
    fn as_str(&self) -> &str {
        match self {
            Tile::Empty => " ",
            Tile::Wall => "#",
            Tile::Block => "+",
            Tile::HPaddle => "–",
            Tile::Ball => "o",
        }
    }

    fn from_i64(val: i64) -> Tile {
        match val {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HPaddle,
            4 => Tile::Ball,
            _ => panic!(),
        }
    }
}

struct Screen {
    tiles: HashMap<(i64, i64), Tile>,
    score: i64,
}

impl Screen {
    fn new() -> Screen {
        Screen {
            tiles: HashMap::new(),
            score: 0,
        }
    }

    fn get(&self, x: i64, y: i64) -> Tile {
        *self.tiles.get(&(x, y)).unwrap_or(&Tile::Empty)
    }

    fn set(&mut self, x: i64, y: i64, val: Tile) {
        self.tiles.insert((x, y), val);
    }

    fn set_score(&mut self, val: i64) {
        self.score = val;
    }

    fn get_score(&self) -> i64 {
        self.score
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let min_x = self.tiles.keys().map(|xy| xy.0).min().unwrap();
        let max_x = self.tiles.keys().map(|xy| xy.0).max().unwrap();
        let min_y = self.tiles.keys().map(|xy| xy.1).min().unwrap();
        let max_y = self.tiles.keys().map(|xy| xy.1).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                write!(f, "{}", self.get(x, y).as_str())?
            }
            write!(f, "\n")?
        }
        Result::Ok(())
    }
}

enum OutputState {
    Empty,
    X(i64),
    XY(i64, i64),
}

struct ScreenIo<'a> {
    screen: &'a RefCell<Screen>,
    output_state: OutputState,
}

impl<'a> ScreenIo<'a> {
    fn new(screen: &'a RefCell<Screen>) -> ScreenIo {
        ScreenIo {
            screen,
            output_state: OutputState::Empty,
        }
    }
}

impl<'a> IntCodeIo for ScreenIo<'a> {
    fn input(&mut self) -> Option<i64> {
        let x_paddle = self
            .screen
            .borrow()
            .tiles
            .iter()
            .find(|(_, t)| **t == Tile::HPaddle)
            .map(|(xy, _)| xy.0);
        let x_ball = self
            .screen
            .borrow()
            .tiles
            .iter()
            .find(|(_, t)| **t == Tile::Ball)
            .map(|(xy, _)| xy.0);
        match (x_paddle, x_ball) {
            (Some(p), Some(b)) => {
                if p < b {
                    Some(1)
                } else if p > b {
                    Some(-1)
                } else {
                    Some(0)
                }
            }
            _ => None,
        }
    }

    fn output(&mut self, val: i64) {
        match self.output_state {
            OutputState::Empty => {
                self.output_state = OutputState::X(val);
            }
            OutputState::X(x) => {
                self.output_state = OutputState::XY(x, val);
            }
            OutputState::XY(x, y) => {
                if x == -1 && y == 0 {
                    self.screen.borrow_mut().set_score(val);
                } else {
                    self.screen.borrow_mut().set(x, y, Tile::from_i64(val));
                }
                self.output_state = OutputState::Empty;
            }
        }
    }
}
