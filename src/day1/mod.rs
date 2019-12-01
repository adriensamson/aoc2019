use std::str::FromStr;

pub fn step1(input : &str) {
    let mut sum = 0;
    for line in input.lines().filter(|s| s.len() > 0) {
        let mass = u64::from_str(line).unwrap();
        sum += get_fuel_for_module(mass);
    }
    println!("{}", sum);
}

fn get_fuel_for_module(mass : u64) -> u64 {
    mass / 3 - 2
}