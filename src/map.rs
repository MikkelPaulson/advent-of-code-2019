use std::cmp;
use std::collections::HashSet;
use std::fmt;
use std::iter;
use std::ops;

#[derive(Debug, Default)]
pub struct Map {
    pub points: HashSet<Coord>,
}

impl fmt::Display for Map {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let max_coord = {
            let [max_x, max_y] = self.points.iter().fold([0; 2], |[acc_x, acc_y], coord| {
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
            if self.points.contains(&Coord {
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

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl From<[usize; 2]> for Coord {
    fn from(data: [usize; 2]) -> Coord {
        Self {
            x: data[0] as isize,
            y: data[1] as isize,
        }
    }
}

impl From<[isize; 2]> for Coord {
    fn from(data: [isize; 2]) -> Coord {
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

impl ops::Sub for Coord {
    type Output = CoordDiff;

    fn sub(self, other: Self) -> Self::Output {
        CoordDiff {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct CoordDiff {
    x: isize,
    y: isize,
}

impl CoordDiff {
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

    pub fn len(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
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
