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

    Ok(loop {
        let result = intcode.run();

        let direction = intcode.output.pop().ok_or("No direction output.")?;
        let color = intcode.output.pop().ok_or("No color output.")?;

        ship.painted_panels.points.insert(ship.robot.position);

        match color {
            0 => {
                ship.white_panels.points.remove(&ship.robot.position);
            }
            1 => {
                ship.white_panels.points.insert(ship.robot.position);
            }
            _ => return Err("Invalid color."),
        }

        match direction {
            0 => {
                ship.robot.facing.turn_left();
                ship.robot.advance();
            }
            1 => {
                ship.robot.facing.turn_right();
                ship.robot.advance();
            }
            _ => return Err("Invalid direction."),
        }

        match result {
            Response::Terminated => break ship.painted_panels.points.len(),
            Response::InputRequired => {
                intcode
                    .input
                    .push(if ship.white_panels.points.contains(&ship.robot.position) {
                        1
                    } else {
                        0
                    })
            }
        }
    })
}

#[derive(Default)]
struct Ship {
    painted_panels: Map,
    white_panels: Map,
    robot: Robot,
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
