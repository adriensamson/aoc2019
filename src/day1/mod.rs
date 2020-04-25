use std::str::FromStr;

#[test]
fn test_step1() {
    assert_eq!("654", step1("1969"))
}

pub fn step1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines().filter(|s| !s.is_empty()) {
        let mass = u64::from_str(line).unwrap();
        sum += get_fuel_for_module(mass);
    }
    format!("{}", sum)
}

#[test]
fn test_step2() {
    assert_eq!("966", step2("1969"))
}

pub fn step2(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines().filter(|s| !s.is_empty()) {
        let mass = u64::from_str(line).unwrap();
        sum += get_total_fuel_for_module(mass);
    }
    format!("{}", sum)
}

fn get_fuel_for_module(mass: u64) -> u64 {
    if mass < 6 {
        return 0;
    }
    mass / 3 - 2
}
fn get_total_fuel_for_module(mass: u64) -> u64 {
    let mut sum = get_fuel_for_module(mass);
    let mut more_fuel = get_fuel_for_module(sum);
    while more_fuel > 0 {
        sum += more_fuel;
        more_fuel = get_fuel_for_module(more_fuel);
    }
    sum
}
