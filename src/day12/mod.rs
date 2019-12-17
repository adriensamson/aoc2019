use regex::Regex;
use std::str::FromStr;

pub fn step1(input : &str) {
    let mut space = Space::from_str(input);
    for _i in 0..1000 {
        space.step_time();
    }
    println!("{}", space.get_total_energy());
}
pub fn step2(input : &str) {}

#[derive(Debug, Copy, Clone)]
struct Vector {
    x : i64,
    y : i64,
    z : i64,
}

impl Vector {
    fn compute_gravity(&self, other : &Self) -> Vector {
        Vector {
            x: if self.x < other.x { 1 } else if self.x > other.x { -1 } else { 0 },
            y: if self.y < other.y { 1 } else if self.y > other.y { -1 } else { 0 },
            z: if self.z < other.z { 1 } else if self.z > other.z { -1 } else { 0 },
        }
    }

    fn get_energy(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl ::std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ::std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

struct Moon {
    position : Vector,
    velocity : Vector,
}


impl Moon {
    fn from_str(s : &str) -> Option<Moon> {
        let input_re= Regex::new("<x=(-?\\d+), y=(-?\\d+), z=(-?\\d+)>").unwrap();
        input_re.captures(s).map(|cap| Moon {
            position: Vector {
                x: i64::from_str(&cap[1]).unwrap(),
                y: i64::from_str(&cap[2]).unwrap(),
                z: i64::from_str(&cap[3]).unwrap(),
            },
            velocity: Vector {x: 0, y: 0, z: 0},
        })
    }

    fn add_gravity(&mut self, gravity : Vector) {
        self.velocity += gravity;
    }

    fn do_move(&mut self) {
        self.position += self.velocity;
    }

    fn compute_gravity(&self, other : &Self) -> Vector {
        self.position.compute_gravity(&other.position)
    }

    fn get_total_energy(&self) -> i64 {
        self.position.get_energy() * self.velocity.get_energy()
    }
}

struct Space {
    moons : Vec<Moon>,
}

impl Space {
    fn from_str(s : &str) -> Space {
        let moons : Vec<Moon> = s.trim().lines().map(|s| Moon::from_str(s).unwrap()).collect();
        Space { moons }
    }

    fn step_time(&mut self) {
        self.apply_gravity();
        self.move_moons();
    }

    fn apply_gravity(&mut self) {
        let n = self.moons.len();
        for i in 0..n {
            for j in i+1..n {
                let moon1 = &self.moons[i];
                let moon2 = &self.moons[j];
                let gravity = moon1.compute_gravity(&moon2);
                self.moons[i].add_gravity(gravity);
                self.moons[j].add_gravity(-gravity);
            }
        }
    }

    fn move_moons(&mut self) {
        for moon in &mut self.moons {
            moon.do_move();
        }
    }

    fn get_total_energy(&self) -> i64 {
        self.moons.iter().map(|m| m.get_total_energy()).sum()
    }
}