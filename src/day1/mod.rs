use std::str::FromStr;

pub fn step1(input : &str) {
    let mut sum = 0;
    for line in input.lines().filter(|s| s.len() > 0) {
        let mass = u64::from_str(line).unwrap();
        sum += get_fuel_for_module(mass);
    }
    println!("{}", sum);
}

pub fn step2(input : &str) {
    let mut sum = 0;
    for line in input.lines().filter(|s| s.len() > 0) {
        let mass = u64::from_str(line).unwrap();
        sum += get_total_fuel_for_module(mass);
    }
    println!("{}", sum);
}


fn get_fuel_for_module(mass : u64) -> u64 {
    if mass < 6 {
        return 0;
    }
    mass / 3 - 2
}
fn get_total_fuel_for_module(mass : u64) -> u64 {
    let mut sum = get_fuel_for_module(mass);
    let mut more_fuel= get_fuel_for_module(sum);
    while more_fuel > 0 {
        sum += more_fuel;
        more_fuel = get_fuel_for_module(more_fuel);
    }
    sum
}