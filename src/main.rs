extern crate regex;
extern crate core;

use std::env;
use std::str::FromStr;
use std::fs::read_to_string;

mod day1;

const DEFAULT_DAY : i32 = 1;
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
        _ => println!("Unknown day or step"),
    }
}


fn parse_int(s : &String) -> i32 {
        i32::from_str(s).unwrap()
}