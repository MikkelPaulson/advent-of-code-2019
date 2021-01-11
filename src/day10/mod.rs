use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter;
use std::str;

type Coord = [isize; 2];

pub fn part1(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let map = parse(input);

    println!("{:?}", map);
    let mut output = format!("{}", map);

    let mut best_coord = [0, 0];
    let mut best_count = 0;

    for coord in map.asteroids.iter() {
        let count = map.count_asteroids(coord);

        if count < 10 {
            let offset = coord[0] as usize * (output.find('\n').unwrap() + 1) + coord[1] as usize;
            output.replace_range(offset..offset + 1, &count.to_string());
        }

        if count > best_count {
            best_count = count;
            best_coord = *coord;
        }
    }

    println!("{}", output);
    println!("Best location is {:?}", [best_coord[1], best_coord[0]]);

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

            let step = [
                asteroid_coord[0] - station_coord[0],
                asteroid_coord[1] - station_coord[1],
            ];

            let gcd = gcd(step[0].abs(), step[1].abs());

            if gcd != 1 {
                let step = [step[0] / gcd, step[1] / gcd];
                for i in 1..gcd {
                    if self.asteroids.contains(&[
                        station_coord[0] + (step[0] * i),
                        station_coord[1] + (step[1] * i),
                    ]) {
                        count -= 1;
                        break;
                    }
                }
            }
        }
        count
    }
}

impl fmt::Display for Map {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let max_coord = self
            .asteroids
            .iter()
            .fold([0; 2], |[acc_row, acc_col], [row, col]| {
                [cmp::max(acc_row, *row), cmp::max(acc_col, *col)]
            });

        let output: String = iter::repeat(
            iter::repeat('.')
                .take(max_coord[1] as usize + 1)
                .chain(iter::once('\n'))
                .enumerate(),
        )
        .take(max_coord[0] as usize + 1)
        .flatten()
        .enumerate()
        .map(|(offset, (col, c))| {
            if self
                .asteroids
                .contains(&[(offset as isize / (max_coord[0] + 2)), col as isize])
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
                asteroids.insert([row as isize, col as isize]);
            }
        }
    }

    Map { asteroids }
}
