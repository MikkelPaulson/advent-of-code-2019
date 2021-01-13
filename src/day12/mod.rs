use std::cell::RefCell;
use std::cmp;
use std::collections::HashSet;
use std::fmt;
use std::str;

use regex::Regex;

use super::math::lcm;

const AXIS_COUNT: usize = 3;
const MOON_COUNT: usize = 4;

pub fn part1(input: &str) -> Result<usize, String> {
    part1_steps(input, 1000)
}

fn part1_steps(input: &str, steps: usize) -> Result<usize, String> {
    let moons = parse(input)?;

    for step in 0..steps {
        for i in 0..MOON_COUNT {
            for j in 0..MOON_COUNT {
                if i == j {
                    continue;
                }
                moons[i].borrow_mut().apply_gravity(&moons[j].borrow());
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

pub fn part2(input: &str) -> Result<usize, String> {
    let moons = parse(input)?;

    Ok((0..AXIS_COUNT)
        .map(|i| get_axis_period(&moons, i))
        .fold(1, |acc, period| lcm(acc as isize, period as isize) as usize))
}

fn parse(input: &str) -> Result<[RefCell<Moon>; MOON_COUNT], String> {
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

fn get_axis_period(moons: &[RefCell<Moon>; MOON_COUNT], axis: usize) -> usize {
    let mut states: HashSet<[[[isize; AXIS_COUNT]; 2]; MOON_COUNT]> = HashSet::new();

    loop {
        let mut value = [[[0; AXIS_COUNT]; 2]; MOON_COUNT];

        for i in 0..MOON_COUNT {
            for j in 0..MOON_COUNT {
                if i == j {
                    continue;
                }

                moons[i]
                    .borrow_mut()
                    .apply_gravity_axis(axis, &moons[j].borrow());
            }
        }

        for (i, mut moon) in moons.iter().map(|m| m.borrow_mut()).enumerate() {
            moon.apply_velocity_axis(axis);
            value[i] = [moon.position, moon.velocity];
        }

        if !states.insert(value) {
            return states.len();
        }
    }
}

struct Moon {
    position: [isize; AXIS_COUNT],
    velocity: [isize; AXIS_COUNT],
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
        for i in 0..AXIS_COUNT {
            self.apply_gravity_axis(i, other);
        }
    }

    fn apply_gravity_axis(&mut self, axis: usize, other: &Moon) {
        self.velocity[axis] += match self.position[axis].cmp(&other.position[axis]) {
            cmp::Ordering::Less => 1,
            cmp::Ordering::Equal => 0,
            cmp::Ordering::Greater => -1,
        };
    }

    fn apply_velocity(&mut self) {
        for i in 0..AXIS_COUNT {
            self.apply_velocity_axis(i);
        }
    }

    fn apply_velocity_axis(&mut self, axis: usize) {
        self.position[axis] += self.velocity[axis];
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
            velocity: [0; AXIS_COUNT],
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
    use super::{part1, part1_steps, part2};

    #[test]
    fn part1_examples() {
        assert_eq!(Ok(179), part1_steps(include_str!("test1.txt"), 10));
        assert_eq!(Ok(1940), part1_steps(include_str!("test2.txt"), 100));
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(10845), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(Ok(2772), part2(include_str!("test1.txt")));
        assert_eq!(Ok(4686774924), part2(include_str!("test2.txt")));
    }

    #[test]
    #[ignore]
    fn part2_solution() {
        assert_eq!(Ok(551272644867044), part2(include_str!("input.txt")));
    }
}
