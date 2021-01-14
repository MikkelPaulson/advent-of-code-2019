use std::cmp::{max, Ordering};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::thread;
use std::time::Duration;

use crate::intcode::{Intcode, Response};
use crate::map::{Coord, CoordDiff};

pub fn part1(input: &str) -> Result<usize, String> {
    let mut intcode: Intcode = input.parse()?;
    intcode.run();

    let game = Game::try_from(&intcode.output.split_off(0)[..])?;
    println!("{}", game);

    Ok(game
        .tiles
        .values()
        .filter(|tile| tile == &&Tile::Block)
        .count())
}

pub fn part2(input: &str) -> Result<usize, String> {
    part2_speed(input, Duration::from_millis(5))
}

fn part2_speed(input: &str, speed: Duration) -> Result<usize, String> {
    let mut intcode: Intcode = input.parse()?;
    intcode.set(0, 2);
    intcode.run();

    intcode.input.push(0);

    let mut game = Game::try_from(&intcode.output.split_off(0)[..])?;
    println!("{}", game);

    while let Response::InputRequired = intcode.run() {
        game.update(&intcode.output.split_off(0)[..])?;
        println!("{}", game);

        intcode.input.push(
            if let (Some(paddle), Some(ball)) = (game.paddle, game.ball) {
                match (ball + game.ball_direction.unwrap_or(CoordDiff::ZERO))
                    .x
                    .cmp(&paddle.x)
                {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                }
            } else {
                0
            },
        );

        if speed > Duration::from_nanos(0) {
            thread::sleep(speed);
        }
    }

    game.update(&intcode.output.split_off(0)[..])?;
    println!("{}", game);

    Ok(game.score as usize)
}

#[derive(Default)]
struct Game {
    tiles: HashMap<Coord, Tile>,
    score: isize,
    paddle: Option<Coord>,
    ball: Option<Coord>,
    ball_direction: Option<CoordDiff>,
}

impl Game {
    fn update(&mut self, input: &[isize]) -> Result<(), String> {
        for chunk in input.chunks_exact(3) {
            self.update_part([chunk[0], chunk[1], chunk[2]])?;
        }
        Ok(())
    }

    fn update_part(&mut self, input: [isize; 3]) -> Result<(), String> {
        if input[0..=1] == [-1, 0] {
            self.score = input[2];
        } else {
            let (coord, tile) = ([input[0], input[1]].into(), Tile::try_from(&input[2])?);

            match tile {
                Tile::Paddle => self.paddle = Some(coord),
                Tile::Ball => {
                    if let Some(old_coord) = self.ball {
                        let ball_direction = coord - old_coord;
                        self.ball_direction = Some(
                            if (coord + ball_direction).y == self.paddle.map(|c| c.y).unwrap_or(0) {
                                CoordDiff { x: 0, y: 0 }
                            } else {
                                ball_direction
                            },
                        );
                    }
                    self.ball = Some(coord);
                }
                _ => {}
            }

            self.tiles.insert(coord, tile);
        }
        Ok(())
    }
}

impl TryFrom<&[isize]> for Game {
    type Error = String;

    fn try_from(input: &[isize]) -> Result<Self, Self::Error> {
        let mut game = Game::default();
        game.update(input)?;
        Ok(game)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_coord: Coord = self
            .tiles
            .keys()
            .fold([0, 0], |[x_max, y_max], coord| {
                [max(x_max, coord.x), max(y_max, coord.y)]
            })
            .into();

        let mut output =
            String::with_capacity(((max_coord.y + 1) * (max_coord.x + 2)) as usize - 1);

        for y in 0..=max_coord.y {
            for x in 0..=max_coord.x {
                output.push(
                    self.tiles
                        .get(&[x, y].into())
                        .unwrap_or(&Tile::Empty)
                        .into(),
                );
            }
            output.push('\n');
        }

        write!(
            f,
            "{}\nScore: {}   Paddle: {:?}\nBall: {:?}   Ball direction: {:?}",
            output.trim_end_matches('\n'),
            self.score,
            self.paddle,
            self.ball,
            self.ball_direction
        )
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<&isize> for Tile {
    type Error = String;

    fn try_from(input: &isize) -> Result<Self, Self::Error> {
        match input {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Wall),
            2 => Ok(Self::Block),
            3 => Ok(Self::Paddle),
            4 => Ok(Self::Ball),
            _ => Err(format!("Invalid tile: {:?}", input)),
        }
    }
}

impl From<&Tile> for char {
    fn from(input: &Tile) -> Self {
        match input {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Block => '*',
            Tile::Paddle => '=',
            Tile::Ball => 'o',
        }
    }
}

#[cfg(test)]
mod test {
    use super::{part1, part2_speed, Duration};

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(298), part1(include_str!("input.txt")));
    }

    #[test]
    #[ignore]
    fn part2_solution() {
        assert_eq!(
            Ok(13956),
            part2_speed(include_str!("input.txt"), Duration::from_nanos(0))
        );
    }
}
