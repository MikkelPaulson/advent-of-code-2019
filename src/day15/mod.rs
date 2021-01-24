use crate::intcode::Intcode;
use crate::map::{Coord, CoordDiff, CoordMap};

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::ops;
use std::str;

pub fn part1(input: &str) -> Result<usize, String> {
    let mut map: Map = input.parse()?;
    let mut steps = 0;

    while let Some(&Tile::Floor) = map.get(&Coord::ORIGIN) {
        map.spread_oxygen();
        steps += 1;
    }

    println!("Step {}:", steps);
    println!("{}", map);

    Ok(steps)
}

pub fn part2(input: &str) -> Result<usize, String> {
    let mut map: Map = input.parse()?;
    let mut minutes = 0;

    while map.iter().find(|(_, &t)| t == Tile::Floor).is_some() {
        map.spread_oxygen();
        minutes += 1;
    }

    println!("Minute {}:", minutes);
    println!("{}", map);

    Ok(minutes)
}

#[derive(Debug, Default)]
struct Map {
    tiles: HashMap<Coord, Tile>,
}

impl Map {
    pub fn spread_oxygen(&mut self) {
        let mut tiles = HashMap::new();

        for coord in self
            .iter()
            .filter_map(|(&c, &t)| if t == Tile::Oxygen { Some(c) } else { None })
        {
            tiles.insert(coord, Tile::OxygenFilled);

            for adjacent in Direction::ALL.iter().map(|&d| coord + d.into()) {
                if let Some(&Tile::Floor) = self.get(&adjacent) {
                    tiles.insert(adjacent, Tile::Oxygen);
                }
            }
        }

        tiles.drain().for_each(|(c, t)| {
            self.insert(c, t);
        });
    }
}

impl str::FromStr for Map {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let intcode: Intcode = raw.parse()?;
        let mut map = Map::default();
        let mut times_visited: HashMap<Coord, u16> = HashMap::new();

        for _ in 0..10 {
            let mut intcode_temp = intcode.clone();
            map.insert(Coord::ORIGIN, Tile::Droid);
            let mut location = Coord::ORIGIN;

            loop {
                let direction = Direction::ALL
                    .iter()
                    .map(|&d| (d, location + d.into()))
                    .min_by(|(_, a), (_, b)| match (map.get(&a), map.get(&b)) {
                        (Some(&Tile::Wall), Some(&Tile::Wall)) => Ordering::Equal,
                        (Some(&Tile::Wall), _) => Ordering::Greater,
                        (_, Some(&Tile::Wall)) => Ordering::Less,
                        _ => times_visited
                            .get(a)
                            .unwrap_or(&0)
                            .cmp(&times_visited.get(b).unwrap_or(&0)),
                    })
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
                        map.insert(location + direction.into(), Tile::Wall);
                    }
                    1 => {
                        if let Some(Tile::Oxygen) = map.insert(location, Tile::Floor) {
                            // oops.
                            map.insert(location, Tile::Oxygen);
                        }

                        location += direction.into();
                        map.insert(location, Tile::Droid);
                    }
                    2 => {
                        map.insert(location, Tile::Floor);
                        location += direction.into();
                        map.insert(location, Tile::Oxygen);
                        *times_visited.entry(location).or_insert(0) += 1;

                        break;
                    }
                    _ => return Err(format!("Unexpected output: {}", result)),
                }

                *times_visited.entry(location).or_insert(0) += 1;
            }
        }

        Ok(map)
    }
}

impl ops::Deref for Map {
    type Target = HashMap<Coord, Tile>;

    fn deref(&self) -> &Self::Target {
        &self.tiles
    }
}

impl ops::DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tiles
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [min_coord, max_coord] = self
            .keys()
            .fold([Coord::ORIGIN; 2], |[acc_min, acc_max], coord| {
                [coord.min(&acc_min), coord.max(&acc_max)]
            });

        for coord in CoordMap::new(min_coord, max_coord) {
            if coord == Coord::ORIGIN {
                write!(f, "*")?;
            } else {
                write!(f, "{}", self.get(&coord).unwrap_or_default())?;
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
    OxygenFilled,
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
                Tile::OxygenFilled => 'o',
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
    const ALL: &'static [Direction] = &[
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

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(248), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(382), part2(include_str!("input.txt")));
    }
}
