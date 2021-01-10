use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;

type Coord = [isize; 2];

pub fn part1(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let map = parse(input);

    println!("{:?}", map);

    let mut best_coord = [0, 0];
    let mut best_count = 0;

    for coord in map.asteroids.iter() {
        let count = map.count_asteroids(coord);
        if count > best_count {
            best_count = count;
            best_coord = *coord;
        }
    }

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
                count += 1;
                continue;
            }

            let step = [
                asteroid_coord[0] - station_coord[0],
                asteroid_coord[1] - station_coord[1],
            ];

            let gcd = gcd(step[0].abs(), step[1].abs());

            count += 1;
            if gcd != 1 {
                let step = [step[0] / gcd, step[1] / gcd];
                for i in 1..=gcd {
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
