use std::cmp;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

use crate::intcode::Intcode;
use crate::map::Coord;

pub fn part1(input: &str) -> Result<usize, String> {
    let mut intcode: Intcode = input.parse()?;

    intcode.run();
    let display = Display::try_from(&intcode.output.split_off(0)[..])?;
    println!("{}", display);

    Ok(display
        .data
        .values()
        .filter(|tile| tile == &&Tile::Block)
        .count())
}

struct Display {
    data: HashMap<Coord, Tile>,
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_coord: Coord = self
            .data
            .keys()
            .fold([0, 0], |[x_max, y_max], coord| {
                [cmp::max(x_max, coord.x), cmp::max(y_max, coord.y)]
            })
            .into();

        let mut output =
            String::with_capacity(((max_coord.y + 3) * (max_coord.x + 4)) as usize - 1);

        output.push('+');
        (0..=max_coord.x).for_each(|_| output.push('-'));
        output.push_str("+\n");

        for y in 0..=max_coord.y {
            output.push('|');
            for x in 0..=max_coord.x {
                output.push(self.data.get(&[x, y].into()).unwrap_or(&Tile::Empty).into());
            }
            output.push_str("|\n");
        }

        output.push('+');
        (0..=max_coord.x).for_each(|_| output.push('-'));
        output.push('+');

        write!(f, "{}", output)
    }
}

impl TryFrom<&[isize]> for Display {
    type Error = String;

    fn try_from(input: &[isize]) -> Result<Self, Self::Error> {
        Ok(Self {
            data: input
                .chunks_exact(3)
                .map(|data| match Tile::try_from(&data[2]) {
                    Ok(t) => Ok(([data[0], data[1]].into(), t)),
                    Err(e) => Err(e),
                })
                .collect::<Result<_, _>>()?,
        })
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
