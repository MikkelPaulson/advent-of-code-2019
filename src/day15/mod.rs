use crate::intcode::Intcode;
use crate::map::{Coord, CoordDiff, CoordMap};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

pub fn part1(input: &str) -> Result<usize, String> {
    let map = get_map(input.parse()?)?;
    println!("{}", map);
    Ok(0)
}

fn get_map(intcode: Intcode) -> Result<Map, String> {
    let mut map = Map::default();

    for _ in 0..10 {
        let mut intcode_temp: Intcode = intcode.clone();
        map.tiles.insert(Coord::ORIGIN, Tile::Droid);
        let mut location: Coord = [0isize, 0].into();

        loop {
            let direction = Direction::DIRECTIONS
                .iter()
                .map(|&d| (d, location + d.into()))
                .min_by(
                    |(_, a), (_, b)| match (map.tiles.get(&a), map.tiles.get(&b)) {
                        (Some(&Tile::Wall), Some(&Tile::Wall)) => Ordering::Equal,
                        (Some(&Tile::Wall), _) => Ordering::Greater,
                        (_, Some(&Tile::Wall)) => Ordering::Less,
                        _ => map
                            .times_visited
                            .get(a)
                            .unwrap_or(&0)
                            .cmp(&map.times_visited.get(b).unwrap_or(&0)),
                    },
                )
                .map(|(d, _)| d)
                .ok_or_else(|| format!("No legal moves from {:?}.", location))?
                .to_owned();

            intcode_temp.input.push(direction.into());
            intcode_temp.run();
            let result = intcode_temp
                .output
                .pop()
                .ok_or_else(|| "No output!".to_string())?;

            match result {
                0 => {
                    map.tiles.insert(location + direction.into(), Tile::Wall);
                }
                1 => {
                    if let Some(Tile::Oxygen) = map.tiles.insert(location, Tile::Floor) {
                        // oops.
                        map.tiles.insert(location, Tile::Oxygen);
                    }

                    location += direction.into();
                    map.tiles.insert(location, Tile::Droid);
                }
                2 => {
                    map.tiles.insert(location, Tile::Floor);
                    location += direction.into();
                    map.tiles.insert(location, Tile::Oxygen);
                    *map.times_visited.entry(location).or_insert(0) += 1;

                    break;
                }
                _ => return Err(format!("Unexpected output: {}", result)),
            }

            *map.times_visited.entry(location).or_insert(0) += 1;
        }
    }

    Ok(map)
}

#[derive(Debug, Default)]
struct Map {
    tiles: HashMap<Coord, Tile>,
    times_visited: HashMap<Coord, u16>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [min_coord, max_coord] = self
            .tiles
            .keys()
            .fold([Coord::ORIGIN; 2], |[acc_min, acc_max], coord| {
                [coord.min(&acc_min), coord.max(&acc_max)]
            });

        for coord in CoordMap::new(min_coord, max_coord) {
            if coord == Coord::ORIGIN {
                write!(f, "*")?;
            } else {
                write!(f, "{}", self.tiles.get(&coord).unwrap_or_default())?;
            }
            if coord.x == max_coord.x && coord.y != max_coord.y {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Droid,
    Wall,
    Floor,
    Oxygen,
    Blank,
}

impl Default for &Tile {
    fn default() -> Self {
        &Tile::Blank
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Droid => 'D',
                Tile::Wall => '#',
                Tile::Floor => '.',
                Tile::Oxygen => 'O',
                Tile::Blank => ' ',
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const DIRECTIONS: &'static [Direction] = &[
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
}

impl From<Direction> for CoordDiff {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::North => CoordDiff { x: 0, y: -1 },
            Direction::East => CoordDiff { x: 1, y: 0 },
            Direction::South => CoordDiff { x: 0, y: 1 },
            Direction::West => CoordDiff { x: -1, y: 0 },
        }
    }
}

impl From<Direction> for isize {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::North => 1,
            Direction::East => 4,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}
