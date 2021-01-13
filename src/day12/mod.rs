use regex::Regex;
use std::cell::RefCell;
use std::cmp;
use std::fmt;
use std::str;

pub fn part1(input: &str) -> Result<usize, String> {
    part1_steps(input, 1000)
}

fn part1_steps(input: &str, steps: usize) -> Result<usize, String> {
    let moons = parse(input)?;

    for step in 0..steps {
        for (i, moon_ref) in moons.iter().enumerate() {
            let mut moon = moon_ref.borrow_mut();

            for (j, other_moon_ref) in moons.iter().enumerate() {
                if i == j {
                    continue;
                }
                moon.apply_gravity(&other_moon_ref.borrow());
            }
        }

        println!("After {} steps:", step + 1);
        for mut moon in moons.iter().map(|m| m.borrow_mut()) {
            moon.apply_velocity();
            println!("{}", moon);
        }
        println!("");
    }

    Ok(moons.iter().map(|moon| moon.borrow().total_energy()).sum())
}

fn parse(input: &str) -> Result<[RefCell<Moon>; 4], String> {
    let mut iter = input
        .trim()
        .split('\n')
        .map(|line| line.parse().map(|m| RefCell::new(m)));

    Ok([
        iter.next()
            .ok_or_else(|| "Missing expected line.".to_string())??,
        iter.next()
            .ok_or_else(|| "Missing expected line.".to_string())??,
        iter.next()
            .ok_or_else(|| "Missing expected line.".to_string())??,
        iter.next()
            .ok_or_else(|| "Missing expected line.".to_string())??,
    ])
}

struct Moon {
    position: [isize; 3],
    velocity: [isize; 3],
}

impl Moon {
    fn total_energy(&self) -> usize {
        self.potential_energy() * self.kinetic_energy()
    }

    fn potential_energy(&self) -> usize {
        self.position.iter().map(|i| i.abs() as usize).sum()
    }

    fn kinetic_energy(&self) -> usize {
        self.velocity.iter().map(|i| i.abs() as usize).sum()
    }

    fn apply_gravity(&mut self, other: &Moon) {
        for i in 0..3 {
            self.velocity[i] += match self.position[i].cmp(&other.position[i]) {
                cmp::Ordering::Less => 1,
                cmp::Ordering::Equal => 0,
                cmp::Ordering::Greater => -1,
            };
        }
    }

    fn apply_velocity(&mut self) {
        for i in 0..3 {
            self.position[i] += self.velocity[i];
        }
    }
}

impl str::FromStr for Moon {
    type Err = String;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^<x=([0-9-]+), y=([0-9-]+), z=([0-9-]+)>")
            .map_err(|e| format!("{:?}", e))?;

        let caps = re
            .captures(data)
            .ok_or_else(|| format!("Invalid input: {:?}", data))?;

        Ok(Self {
            position: [
                caps.get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .map_err(|e| format!("{:?}", e))?,
                caps.get(2)
                    .unwrap()
                    .as_str()
                    .parse()
                    .map_err(|e| format!("{:?}", e))?,
                caps.get(3)
                    .unwrap()
                    .as_str()
                    .parse()
                    .map_err(|e| format!("{:?}", e))?,
            ],
            velocity: [0; 3],
        })
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
            self.position[0],
            self.position[1],
            self.position[2],
            self.velocity[0],
            self.velocity[1],
            self.velocity[2],
        )
    }
}

#[cfg(test)]
mod test {
    use super::{part1, part1_steps};

    #[test]
    fn part1_examples() {
        assert_eq!(Ok(179), part1_steps(include_str!("test1.txt"), 10));
        assert_eq!(Ok(1940), part1_steps(include_str!("test2.txt"), 100));
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(10845), part1(include_str!("input.txt")));
    }
}
