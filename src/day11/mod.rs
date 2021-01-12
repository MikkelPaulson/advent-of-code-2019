use crate::intcode::{Intcode, Response};
use crate::map::{Coord, Map};

use std::default;
use std::fmt;
use std::io::prelude::*;
use std::str;

pub fn part1(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let mut intcode = Intcode::parse(input);
    let mut ship = Ship::default();

    // Start on a black panel.
    intcode.input.push(0);
    ship.run(intcode)?;

    Ok(ship.painted_panels.points.len())
}

pub fn part2(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let mut intcode = Intcode::parse(input);
    let mut ship = Ship::default();

    // Start on a white panel.
    intcode.input.push(1);
    ship.run(intcode)?;

    println!("{}", ship.white_panels);

    Ok(0)
}

#[derive(Default)]
struct Ship {
    painted_panels: Map,
    white_panels: Map,
    robot: Robot,
}

impl Ship {
    pub fn run(&mut self, mut intcode: Intcode) -> Result<(), &'static str> {
        loop {
            let result = intcode.run();

            let direction = intcode.output.pop().ok_or("No direction output.")?;
            let color = intcode.output.pop().ok_or("No color output.")?;

            self.painted_panels.points.insert(self.robot.position);

            match color {
                0 => {
                    self.white_panels.points.remove(&self.robot.position);
                }
                1 => {
                    self.white_panels.points.insert(self.robot.position);
                }
                _ => return Err("Invalid color."),
            }

            match direction {
                0 => {
                    self.robot.facing.turn_left();
                    self.robot.advance();
                }
                1 => {
                    self.robot.facing.turn_right();
                    self.robot.advance();
                }
                _ => return Err("Invalid direction."),
            }

            match result {
                Response::Terminated => return Ok(()),
                Response::InputRequired => {
                    intcode
                        .input
                        .push(if self.white_panels.points.contains(&self.robot.position) {
                            1
                        } else {
                            0
                        })
                }
            }
        }
    }
}

struct Robot {
    position: Coord,
    facing: Direction,
}

impl Robot {
    pub fn advance(&mut self) {
        match self.facing {
            Direction::Up => self.position.y -= 1,
            Direction::Left => self.position.x -= 1,
            Direction::Down => self.position.y += 1,
            Direction::Right => self.position.x += 1,
        }
    }
}

impl default::Default for Robot {
    fn default() -> Self {
        Self {
            position: Coord::default(),
            facing: Direction::Up,
        }
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.facing {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&mut self) {
        *self = match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        };
    }

    fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}
