use std::cmp;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

use crate::intcode::Intcode;
use crate::map::Coord;

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

#[derive(Default)]
struct Game {
    tiles: HashMap<Coord, Tile>,
    score: isize,
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
            self.tiles
                .insert([input[0], input[1]].into(), Tile::try_from(&input[2])?);
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
                [cmp::max(x_max, coord.x), cmp::max(y_max, coord.y)]
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

        write!(f, "{}", output.trim_end_matches('\n'))
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
            Tile::Paddle => '_',
            Tile::Ball => 'o',
        }
    }
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(298), part1(include_str!("input.txt")));
    }
}
