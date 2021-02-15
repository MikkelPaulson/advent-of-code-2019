use std::collections::{BTreeMap, HashSet};

use crate::map::{Coord, Map};

pub fn part1(input: &str) -> Result<u64, String> {
    let map = parse(input);

    println!("{}", map);

    let best_coord = find_station(&map).ok_or("No station coordinate found.")?;
    println!("Best location is {:?}", best_coord);

    Ok(count_asteroids(&map, &best_coord))
}

pub fn part2(input: &str) -> Result<u64, String> {
    let map = parse(input);
    let station_coord = find_station(&map).ok_or("No station coordinate found.")?;

    let mut asteroid_groups = BTreeMap::new();
    for asteroid in map.points {
        if asteroid == station_coord {
            continue;
        }

        asteroid_groups
            .entry((station_coord - asteroid).reduce())
            .or_insert_with(Vec::new)
            .push(asteroid);
    }

    asteroid_groups.iter_mut().for_each(|(_, v)| {
        v.sort_unstable_by(|a, b| {
            (*a - station_coord)
                .len()
                .cmp(&(*b - station_coord).len())
                .reverse()
        })
    });

    let mut i = 0;
    while !asteroid_groups.is_empty() {
        for key in asteroid_groups.keys().copied().collect::<Vec<_>>() {
            i += 1;
            let asteroid = asteroid_groups.get_mut(&key).map(|v| v.pop()).flatten();
            println!("{}: {:?}", i, asteroid);
            if asteroid_groups.get(&key).map_or(false, |v| v.is_empty()) {
                asteroid_groups.remove(&key);
            }

            if i == 200 {
                return Ok((asteroid.unwrap().x * 100 + asteroid.unwrap().y) as u64);
            }
        }
    }

    Err("Ran out of asteroids to destroy.".to_string())
}

fn find_station(map: &Map) -> Option<Coord> {
    let mut best_coord = None;
    let mut best_count = 0;

    for coord in map.points.iter() {
        let count = count_asteroids(map, coord);
        if count > best_count {
            best_count = count;
            best_coord = Some(*coord);
        }
    }

    best_coord
}

fn count_asteroids(map: &Map, station_coord: &Coord) -> u64 {
    let mut count = 0;
    for asteroid_coord in map.points.iter() {
        if asteroid_coord == station_coord {
            continue;
        }
        count += 1;

        let step = (*asteroid_coord - *station_coord).reduce();
        let mut target_coord = *station_coord + step;

        while &target_coord != asteroid_coord {
            if map.points.contains(&target_coord) {
                count -= 1;
                break;
            }
            target_coord = target_coord + step;
        }
    }
    count
}

fn parse(input: &str) -> Map {
    let mut asteroids = HashSet::new();

    for (row, line) in input.split('\n').enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.insert([col as i64, row as i64].into());
            }
        }
    }

    Map { points: asteroids }
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_examples() {
        assert_eq!(Ok(8), part1(include_str!("test1.txt")));
        assert_eq!(Ok(33), part1(include_str!("test2.txt")));
        assert_eq!(Ok(35), part1(include_str!("test3.txt")));
        assert_eq!(Ok(41), part1(include_str!("test4.txt")));
        assert_eq!(Ok(210), part1(include_str!("test5.txt")));
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(269), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(Ok(802), part2(include_str!("test5.txt")));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(612), part2(include_str!("input.txt")));
    }
}
