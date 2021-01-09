use std::collections::HashMap;
use std::io::prelude::*;
use std::str;

pub fn part1(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let mut map = parse(input);

    let mut orbit_count = 0;

    let body_ids = map.keys().cloned().collect::<Vec<BodyID>>();
    for body_id in body_ids {
        orbit_count += get_orbits(&body_id, &mut map);
    }

    Ok(orbit_count)
}

pub fn part2(input: Box<dyn Read>) -> Result<usize, &'static str> {
    Err("Not implemented")
}

type BodyID = String;

enum OrbitData {
    Body(BodyID),
    CenterOfMass,
}

fn get_orbits(body_id: &BodyID, mut map: &HashMap<BodyID, OrbitData>) -> usize {
    match map.get(body_id) {
        Some(OrbitData::Body(parent_id)) => {
            let count = get_orbits(parent_id, &mut map) + 1;
            count
        }
        Some(OrbitData::CenterOfMass) => 1,
        None => panic!(),
    }
}

fn parse(mut input: Box<dyn Read>) -> HashMap<BodyID, OrbitData> {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let mut map = HashMap::new();

    for line in buffer.trim_end().split('\n') {
        let mut parts = line
            .split(')')
            .map(|s| s.to_string())
            .collect::<Vec<BodyID>>();
        let child = parts.pop().unwrap();
        let parent = parts.pop().unwrap();

        map.insert(
            child,
            if "COM" == parent {
                OrbitData::CenterOfMass
            } else {
                OrbitData::Body(parent)
            },
        );
    }

    map
}
