use std::cmp;
use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter;
use std::str;

use super::coord::Coord;

pub fn part1(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let map = parse(input);

    println!("{:?}", map);

    let best_coord = map.find_station().ok_or("No station coordinate found.")?;
    println!("Best location is {:?}", best_coord);

    Ok(map.count_asteroids(&best_coord))
}

pub fn part2(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let map = parse(input);
    let station_coord = map.find_station().ok_or("No station coordinate found.")?;

    let mut asteroid_groups = BTreeMap::new();
    for asteroid in map.asteroids {
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
                return Ok((asteroid.unwrap().x * 100 + asteroid.unwrap().y) as usize);
            }
        }
    }

    Err("Ran out of asteroids to destroy.")
}

#[derive(Debug)]
struct Map {
    asteroids: HashSet<Coord>,
}

impl Map {
    fn find_station(&self) -> Option<Coord> {
        let mut best_coord = None;
        let mut best_count = 0;

        for coord in self.asteroids.iter() {
            let count = self.count_asteroids(coord);
            if count > best_count {
                best_count = count;
                best_coord = Some(*coord);
            }
        }

        best_coord
    }

    fn count_asteroids(&self, station_coord: &Coord) -> usize {
        let mut count = 0;
        for asteroid_coord in self.asteroids.iter() {
            if asteroid_coord == station_coord {
                continue;
            }
            count += 1;

            let step = (*asteroid_coord - *station_coord).reduce();
            let mut target_coord = *station_coord + step;

            while &target_coord != asteroid_coord {
                if self.asteroids.contains(&target_coord) {
                    count -= 1;
                    break;
                }
                target_coord = target_coord + step;
            }
        }
        count
    }
}

impl fmt::Display for Map {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let max_coord = {
            let [max_x, max_y] = self.asteroids.iter().fold([0; 2], |[acc_x, acc_y], coord| {
                [cmp::max(acc_x, coord.x), cmp::max(acc_y, coord.y)]
            });
            Coord { x: max_x, y: max_y }
        };

        let output: String = iter::repeat(
            iter::repeat('.')
                .take(max_coord.x as usize + 1)
                .chain(iter::once('\n'))
                .enumerate(),
        )
        .take(max_coord.y as usize + 1)
        .flatten()
        .enumerate()
        .map(|(offset, (col, c))| {
            if self.asteroids.contains(&Coord {
                x: offset as isize / (max_coord.x + 2),
                y: col as isize,
            }) {
                '#'
            } else {
                c
            }
        })
        .collect();

        write!(formatter, "{}", output)?;

        Ok(())
    }
}

fn parse(input: Box<dyn Read>) -> Map {
    let reader = BufReader::new(input);
    let mut asteroids = HashSet::new();

    for (row, line) in reader.lines().enumerate() {
        for (col, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                asteroids.insert([col, row].into());
            }
        }
    }

    Map { asteroids }
}
