use std::cmp;
use std::collections::HashSet;
use std::fmt;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter;
use std::ops;
use std::str;

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

#[derive(Copy, Clone, Debug)]
struct CoordDiff(isize, isize);

impl CoordDiff {
    fn reduce(&self) -> CoordDiff {
        let gcd = gcd(self.0.abs(), self.1.abs());
        CoordDiff(self.0 / gcd, self.1 / gcd)
    }
}

pub fn part1(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let map = parse(input);

    println!("{:?}", map);
    let mut output = format!("{}", map);

    let mut best_coord = None;
    let mut best_count = 0;

    for coord in map.asteroids.iter() {
        let count = map.count_asteroids(coord);

        if count < 10 {
            let offset = coord.0 * (output.find('\n').unwrap() + 1) + coord.1;
            output.replace_range(offset..offset + 1, &count.to_string());
        }

        if count > best_count {
            best_count = count;
            best_coord = Some(coord);
        }
    }

    println!("{}", output);
    println!("Best location is {:?}", best_coord);

    Ok(best_count)
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
