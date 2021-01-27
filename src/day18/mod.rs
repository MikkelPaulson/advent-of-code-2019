use std::collections::{HashMap, HashSet};

use super::map::{Coord, CoordDiff};
use super::maze::{Maze, Tile};

pub fn part1(input: &str) -> Result<usize, String> {
    let (maze, key_doors) = parse(input)?;

    println!(
        "{}",
        maze.display_with_overlay(|coord| if coord == &Coord::ORIGIN {
            Some('*')
        } else {
            None
        })
    );

    Err("Incomplete".to_string())
}

type KeyDoor = HashMap<Coord, Coord>;

fn parse(input: &str) -> Result<(Maze, KeyDoor), String> {
    let mut coord = Coord::ORIGIN;
    let mut raw_maze = HashMap::new();
    let mut offset = CoordDiff::ZERO;
    let mut keys = HashMap::with_capacity(26);
    let mut doors = HashMap::with_capacity(26);

    for raw_tile in input.chars() {
        if raw_tile == '\n' {
            coord.y += 1;
            coord.x = 0;
            continue;
        }

        raw_maze.insert(
            coord,
            match raw_tile {
                '#' => Tile::Wall,
                '.' => Tile::Floor,
                '@' => {
                    offset = coord - Coord::ORIGIN;
                    Tile::Floor
                }
                c if c.is_uppercase() => {
                    doors.insert(c.to_ascii_lowercase(), coord);
                    Tile::Wall
                }
                c if c.is_lowercase() => {
                    keys.insert(c, coord);
                    Tile::Floor
                }
                c => {
                    return Err(format!("Invalid character: {:?}", c));
                }
            },
        );

        coord.x += 1;
    }

    let mut maze = Maze::default();
    maze.extend(raw_maze.drain().map(|(coord, tile)| (coord - offset, tile)));

    Ok((
        maze,
        keys.drain()
            .map(|(c, key_coord)| {
                if let Some(door_coord) = doors.remove(&c) {
                    Ok((key_coord, door_coord))
                } else {
                    Err(format!("No door for key {}.", c))
                }
            })
            .collect::<Result<_, _>>()?,
    ))
}
