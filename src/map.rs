use std::cmp;
use std::collections::HashSet;
use std::fmt;
use std::iter;
use std::ops;

use super::math::gcd;

#[derive(Debug, Default)]
pub struct Map {
    pub points: HashSet<Coord>,
}

impl fmt::Display for Map {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let (min_coord, max_coord) = {
            let [[min_x, min_y], [max_x, max_y]] =
                self.points
                    .iter()
                    .fold([[0; 2]; 2], |[[min_x, min_y], [max_x, max_y]], coord| {
                        [
                            [cmp::min(min_x, coord.x), cmp::min(min_y, coord.y)],
                            [cmp::max(max_x, coord.x), cmp::max(max_y, coord.y)],
                        ]
                    });
            (Coord { x: min_x, y: min_y }, Coord { x: max_x, y: max_y })
        };

        let mut output = String::with_capacity(
            ((max_coord.x - min_coord.x + 1) * (max_coord.y - min_coord.y + 1)) as usize,
        );

        for y in min_coord.y..=max_coord.y {
            if !output.is_empty() {
                output.push('\n');
            }
            for x in min_coord.x..=max_coord.x {
                output.push(match [x, y] {
                    _ if self.points.contains(&Coord { x, y }) => '#',
                    [0, 0] => '+',
                    [0, _] => '|',
                    [_, 0] => '-',
                    _ => '.',
                })
            }
        }

        write!(formatter, "{}", output)?;

        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}

impl Coord {
    pub const ORIGIN: Self = Self { x: 0, y: 0 };

    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<[u64; 2]> for Coord {
    fn from(data: [u64; 2]) -> Coord {
        Self {
            x: data[0] as i64,
            y: data[1] as i64,
        }
    }
}

impl From<[i64; 2]> for Coord {
    fn from(data: [i64; 2]) -> Coord {
        Self {
            x: data[0],
            y: data[1],
        }
    }
}

impl ops::Add<CoordDiff> for Coord {
    type Output = Coord;

    fn add(self, other: CoordDiff) -> Self::Output {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign<CoordDiff> for Coord {
    fn add_assign(&mut self, other: CoordDiff) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub<Coord> for Coord {
    type Output = CoordDiff;

    fn sub(self, other: Self) -> Self::Output {
        CoordDiff {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Sub<CoordDiff> for Coord {
    type Output = Self;

    fn sub(self, other: CoordDiff) -> Self::Output {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct CoordDiff {
    pub x: i64,
    pub y: i64,
}

impl CoordDiff {
    pub const ZERO: CoordDiff = CoordDiff { x: 0, y: 0 };

    pub const DIRECTIONS: [Self; 4] = [
        Self { x: -1, y: 0 },
        Self { x: 0, y: -1 },
        Self { x: 1, y: 0 },
        Self { x: 0, y: 1 },
    ];

    pub fn reduce(&self) -> CoordDiff {
        let gcd = gcd(self.x.abs(), self.y.abs());
        CoordDiff {
            x: self.x / gcd,
            y: self.y / gcd,
        }
    }

    pub fn angle(&self) -> f64 {
        let angle = (self.y as f64).atan2(self.x as f64);
        if angle >= std::f64::consts::FRAC_PI_2 {
            angle - std::f64::consts::FRAC_PI_2
        } else {
            angle + std::f64::consts::PI + std::f64::consts::FRAC_PI_2
        }
    }

    pub fn len(&self) -> u64 {
        (self.x.abs() + self.y.abs()) as u64
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

pub struct CoordMap {
    max: Coord,
    min: Coord,
    next: Option<Coord>,
}

impl CoordMap {
    pub fn new(min: Coord, max: Coord) -> Self {
        Self {
            max,
            min,
            next: Some(min),
        }
    }
}

impl iter::Iterator for CoordMap {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(coord) = self.next.take() {
            self.next = if coord == self.max {
                None
            } else if coord.x == self.max.x {
                Some(Coord {
                    x: self.min.x,
                    y: coord.y + 1,
                })
            } else {
                Some(Coord {
                    x: coord.x + 1,
                    y: coord.y,
                })
            };

            Some(coord)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const ALL: &'static [Direction] = &[
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub fn reverse(&self) -> Direction {
        match *self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
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

impl From<Direction> for i64 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::North => 1,
            Direction::East => 4,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}
