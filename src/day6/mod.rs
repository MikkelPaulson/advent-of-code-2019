use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> Result<usize, String> {
    let map = parse(input);

    let mut orbit_count = 0;

    let body_ids = map.keys().cloned().collect::<Vec<BodyID>>();
    for body_id in body_ids {
        orbit_count += get_orbit_count(&body_id, &map);
    }

    Ok(orbit_count)
}

pub fn part2(input: &str) -> Result<usize, String> {
    let map = parse(input);

    let me = &"YOU".to_string();
    let my_orbits = get_orbits(&me, &map);

    let santa = &"SAN".to_string();
    let santa_orbits = get_orbits(&santa, &map);

    println!("{:?}", my_orbits);
    println!("{:?}", santa_orbits);

    Ok(my_orbits.symmetric_difference(&santa_orbits).count())
}

type BodyID = String;

enum OrbitData {
    Body(BodyID),
    CenterOfMass,
}

fn get_orbits<'a>(
    mut body_id: &'a BodyID,
    map: &'a HashMap<BodyID, OrbitData>,
) -> HashSet<&'a BodyID> {
    let mut my_orbits = HashSet::new();
    while let Some(OrbitData::Body(parent_id)) = map.get(body_id) {
        body_id = parent_id;
        my_orbits.insert(parent_id);
    }
    my_orbits
}

fn get_orbit_count(body_id: &BodyID, map: &HashMap<BodyID, OrbitData>) -> usize {
    match map.get(body_id) {
        Some(OrbitData::Body(parent_id)) => {
            let count = get_orbit_count(parent_id, &map) + 1;
            count
        }
        Some(OrbitData::CenterOfMass) => 1,
        None => panic!(),
    }
}

fn parse(input: &str) -> HashMap<BodyID, OrbitData> {
    let mut map = HashMap::new();

    for line in input.trim_end().split('\n') {
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

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_examples() {
        assert_eq!(Ok(42), part1(include_str!("test1.txt")));
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(301100), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(Ok(4), part2(include_str!("test2.txt")));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(547), part2(include_str!("input.txt")));
    }
}
