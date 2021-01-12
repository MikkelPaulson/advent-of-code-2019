use std::cmp;
use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter;
use std::ops;
use std::str;

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
                return Ok(asteroid.unwrap().1 * 100 + asteroid.unwrap().0);
            }
        }
    }

    Err("Ran out of asteroids to destroy.")
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Coord(usize, usize);

impl From<[usize; 2]> for Coord {
    fn from(data: [usize; 2]) -> Coord {
        Self(data[0], data[1])
    }
}

impl ops::Add<CoordDiff> for Coord {
    type Output = Coord;

    fn add(self, other: CoordDiff) -> Self::Output {
        Coord(
            (self.0 as isize + other.0) as usize,
            (self.1 as isize + other.1) as usize,
        )
    }
}

impl ops::Sub for Coord {
    type Output = CoordDiff;

    fn sub(self, other: Self) -> Self::Output {
        CoordDiff(
            self.0 as isize - other.0 as isize,
            self.1 as isize - other.1 as isize,
        )
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct CoordDiff(isize, isize);

impl CoordDiff {
    fn reduce(&self) -> CoordDiff {
        let gcd = gcd(self.0.abs(), self.1.abs());
        CoordDiff(self.0 / gcd, self.1 / gcd)
    }

    fn angle(&self) -> f64 {
        let angle = (self.0 as f64).atan2(self.1 as f64);
        if angle >= std::f64::consts::FRAC_PI_2 {
            angle - std::f64::consts::FRAC_PI_2
        } else {
            angle + std::f64::consts::PI + std::f64::consts::FRAC_PI_2
        }
    }

    fn len(&self) -> usize {
        (self.0.abs() + self.1.abs()) as usize
    }
}

impl cmp::Ord for CoordDiff {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.angle().partial_cmp(&other.angle()) {
            Some(cmp::Ordering::Equal) | None => self.len().cmp(&other.len()),
            Some(o) => o,
        }
    }
}

impl cmp::PartialOrd for CoordDiff {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
            let [max_row, max_col] =
                self.asteroids
                    .iter()
                    .fold([0; 2], |[acc_row, acc_col], coord| {
                        [cmp::max(acc_row, coord.0), cmp::max(acc_col, coord.1)]
                    });
            Coord(max_row, max_col)
        };

        let output: String = iter::repeat(
            iter::repeat('.')
                .take(max_coord.1 + 1)
                .chain(iter::once('\n'))
                .enumerate(),
        )
        .take(max_coord.0 + 1)
        .flatten()
        .enumerate()
        .map(|(offset, (col, c))| {
            if self
                .asteroids
                .contains(&Coord(offset / (max_coord.0 + 2), col))
            {
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
                asteroids.insert(Coord(row, col));
            }
        }
    }

    Map { asteroids }
}
