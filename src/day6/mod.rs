use std::collections::HashMap;

pub fn step1(input: &str) {
    let map = parse_map(input);
    let mut sum = 0;
    for obj in map.keys() {
        sum += count_orbits_for(&map, obj);
    }
    println!("{}", sum);
}
pub fn step2(input: &str) {
    let map = parse_map(input);
    let path_from_you = make_path_from(&map, "YOU");
    let path_from_san = make_path_from(&map, "SAN");
    let dist = dist_between(&path_from_you, &path_from_san);
    println!("{}", dist);
}

fn parse_map(input: &str) -> HashMap<&str, &str> {
    let mut map = HashMap::new();
    for line in input.trim().lines() {
        let objs: Vec<&str> = line.split(")").take(2).collect();
        map.insert(objs[1], objs[0]);
    }
    map
}

fn count_orbits_for(map: &HashMap<&str, &str>, obj: &str) -> usize {
    map.get(obj)
        .and_then(|around| Some(1 + count_orbits_for(map, around)))
        .unwrap_or(0)
}

fn make_path_from<'a>(map: &HashMap<&str, &'a str>, from: &str) -> Vec<&'a str> {
    let mut path = Vec::new();
    let mut current = from;
    loop {
        match map.get(current) {
            None => break,
            Some(parent) => {
                path.push(*parent);
                current = parent;
            }
        }
    }
    path
}

fn dist_between(path1: &Vec<&str>, path2: &Vec<&str>) -> usize {
    for (i, obj1) in path1.iter().enumerate() {
        for (j, obj2) in path2.iter().enumerate() {
            if obj1 == obj2 {
                return i + j;
            }
        }
    }
    panic!("no path");
}
