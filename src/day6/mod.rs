use std::collections::HashMap;

pub fn step1(input : &str) {
    let map = parse_map(input);
    let mut sum = 0;
    for obj in map.keys() {
        sum += count_orbits_for(&map, obj);
    }
    println!("{}", sum);
}
pub fn step2(input : &str) {}

fn parse_map(input : &str) -> HashMap<&str, &str> {
    let mut map = HashMap::new();
    for line in input.trim().lines() {
        let objs : Vec<&str> = line.split(")").take(2).collect();
        map.insert(objs[1], objs[0]);
    }
    map
}

fn count_orbits_for(map : &HashMap<&str, &str>, obj : &str) -> usize {
    map.get(obj).and_then(|around| Some(1 + count_orbits_for(map, around))).unwrap_or(0)
}