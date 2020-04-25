use crate::intcode::{IntCode, IntCodeIo, VecPrintIo};

pub fn step1(input: &str) {
    let mut program = IntCode::from_str(input, AsciiIo::new());
    program.run();
    println!("{}", program.io);
    println!("{}", program.io.get_calibration());
}

pub fn step2(input: &str) {
    let mut program = IntCode::from_str(input, AsciiIo::new());
    program.run();
    let path = program.io.find_path();
    let factorized = factorize_path(&path);
    println!("{}", factorized);
    let commands = factorized
        .chars()
        .chain(vec!['n', '\n'])
        .map(|c| c as i64)
        .collect();
    let mut program2 = IntCode::from_str(input, VecPrintIo::new(commands));
    program2.set_at(0, 2);
    program2.run();
}

struct AsciiIo {
    rows: Vec<Vec<char>>,
}

impl AsciiIo {
    fn new() -> AsciiIo {
        AsciiIo { rows: vec![vec![]] }
    }

    fn get_at(&self, i: usize, j: usize) -> Option<char> {
        self.rows.get(i).and_then(|r| r.get(j).map(|c| *c))
    }

    fn get_at_coord(&self, c: &Coord) -> Option<char> {
        self.get_at(c.y, c.x)
    }

    fn get_calibration(&self) -> usize {
        let mut sum = 0;
        for i in 1..self.rows.len() - 2 {
            for j in 1..self.rows[i].len() - 1 {
                if self.get_at(i, j) == Some('#') {
                    match (
                        self.get_at(i - 1, j),
                        self.get_at(i + 1, j),
                        self.get_at(i, j - 1),
                        self.get_at(i, j + 1),
                    ) {
                        (Some('#'), Some('#'), Some('#'), Some('#')) => {
                            sum += i * j;
                        }
                        _ => {}
                    }
                }
            }
        }
        sum
    }

    fn find_path(&self) -> String {
        let mut path = String::new();
        let mut pos: Coord = self
            .rows
            .iter()
            .enumerate()
            .find_map(|(i, r)| {
                r.iter()
                    .enumerate()
                    .find_map(|(j, c)| match c {
                        '^' | 'v' | '>' | '<' => Some(j),
                        _ => None,
                    })
                    .map(|j| Coord { y: i, x: j })
            })
            .unwrap();
        let mut dir = Direction::from_char(self.get_at_coord(&pos).unwrap());
        if self.get_at_coord(&pos.forward(dir, 1).unwrap()) != Some('#') {
            if self.get_at_coord(&pos.forward(dir.left(), 1).unwrap()) == Some('#') {
                path.push_str("L,");
                dir = dir.left();
            } else if self.get_at_coord(&pos.forward(dir.right(), 1).unwrap()) == Some('#') {
                path.push_str("R,");
                dir = dir.right();
            } else {
                panic!();
            }
        }
        loop {
            let mut i = 1;
            while pos.forward(dir, i + 1).and_then(|c| self.get_at_coord(&c)) == Some('#') {
                i += 1;
            }
            path.push_str(&format!("{},", i));
            pos = pos.forward(dir, i).unwrap();
            if pos
                .forward(dir.left(), 1)
                .and_then(|c| self.get_at_coord(&c))
                == Some('#')
            {
                path.push_str("L,");
                dir = dir.left();
            } else if pos
                .forward(dir.right(), 1)
                .and_then(|c| self.get_at_coord(&c))
                == Some('#')
            {
                path.push_str("R,");
                dir = dir.right();
            } else {
                break;
            }
        }
        path.pop();
        path
    }
}

impl IntCodeIo for AsciiIo {
    fn input(&mut self) -> Option<i64> {
        None
    }

    fn output(&mut self, val: i64) {
        let c = char::from(val as u8);
        if c == '\n' {
            self.rows.push(Vec::new());
        } else {
            self.rows.last_mut().unwrap().push(c);
        }
    }
}

impl std::fmt::Display for AsciiIo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for row in &self.rows {
            for c in row {
                write!(f, "{}", c)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

struct Coord {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => panic!(),
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

impl Coord {
    fn forward(&self, dir: Direction, n: usize) -> Option<Coord> {
        match dir {
            Direction::North => {
                if n <= self.y {
                    Some(Coord {
                        x: self.x,
                        y: self.y - n,
                    })
                } else {
                    None
                }
            }
            Direction::West => {
                if n <= self.x {
                    Some(Coord {
                        x: self.x - n,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Direction::East => Some(Coord {
                x: self.x + n,
                y: self.y,
            }),
            Direction::South => Some(Coord {
                x: self.x,
                y: self.y + n,
            }),
        }
    }
}

fn factorize_path(path: &str) -> String {
    for a in find_segments(path, 0) {
        let main_a = String::from(path).replace(&a, "A");
        if let Some(after_a) = main_a.find(|c| c == 'R' || c == 'L' || char::is_numeric(c)) {
            for b in find_segments(&main_a, after_a) {
                let main_b = String::from(&main_a).replace(&b, "B");
                if let Some(after_b) = main_b.find(|c| c == 'R' || c == 'L' || char::is_numeric(c))
                {
                    for c in find_segments(&main_b, after_b) {
                        let main_c = String::from(&main_b).replace(&c, "C");
                        if main_c.find(|c| c == 'R' || c == 'L' || char::is_numeric(c)) == None {
                            return format!("{}\n{}\n{}\n{}\n", main_c, a, b, c);
                        }
                    }
                }
            }
        }
    }
    panic!();
}

fn find_segments(path: &str, start: usize) -> Vec<String> {
    let mut found = Vec::new();

    for l in 1..=20 {
        if &path[start + l..=start + l] != "," {
            continue;
        }
        let s = &path[start..start + l];
        let nb = s.len() * path.matches(s).count();
        found.push((s, nb));
    }

    found.sort_by_key(|(_, nb)| *nb);

    found.iter().map(|f| String::from(f.0)).collect()
}
