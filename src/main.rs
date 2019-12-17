extern crate regex;
extern crate core;

use std::env;
use std::str::FromStr;
use std::fs::read_to_string;

mod intcode;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

const DEFAULT_DAY : i32 = 10;
const DEFAULT_STEP : i32 = 1;
const DEFAULT_FILE : &str = "input";

fn main() {
    let args : Vec<String> = env::args().collect();
    let day = args.get(1). map(parse_int).unwrap_or(DEFAULT_DAY);
    let step = args.get(2).map(parse_int).unwrap_or(DEFAULT_STEP);

    let filename = format!("src/day{}/{}.txt", day, args.get(3).map(|s| s.as_ref()).unwrap_or(DEFAULT_FILE));
    let input = &read_to_string(filename).unwrap();

    match (day, step) {
        (1, 1) => day1::step1(input),
        (1, 2) => day1::step2(input),
        (2, 1) => day2::step1(input),
        (2, 2) => day2::step2(input),
        (3, 1) => day3::step1(input),
        (3, 2) => day3::step2(input),
        (4, 1) => day4::step1(input),
        (4, 2) => day4::step2(input),
        (5, 1) => day5::step1(input),
        (5, 2) => day5::step2(input),
        (6, 1) => day6::step1(input),
        (6, 2) => day6::step2(input),
        (7, 1) => day7::step1(input),
        (7, 2) => day7::step2(input),
        (8, 1) => day8::step1(input),
        (8, 2) => day8::step2(input),
        (9, 1) => day9::step1(input),
        (9, 2) => day9::step2(input),
        (10, 1) => day10::step1(input),
        (10, 2) => day10::step2(input),
        _ => println!("Unknown day or step"),
    }
}


fn parse_int(s : &String) -> i32 {
        i32::from_str(s).unwrap()
}