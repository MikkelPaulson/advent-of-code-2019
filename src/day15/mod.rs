use crate::intcode::Intcode;
use crate::map::{Coord, Direction};
use crate::maze::{Maze, Tile};

use std::collections::{HashMap, HashSet};
use std::mem;
use std::str;

pub fn part1(input: &str) -> Result<usize, String> {
    let (maze, oxygen_coord) = explore(input.parse()?)?;

    println!(
        "{}",
        maze.display_with_overlay(|coord| if coord == &oxygen_coord {
            Some('O')
        } else if coord == &Coord::ORIGIN {
            Some('*')
        } else {
            None
        })
    );

    maze.get_path_len(oxygen_coord, Coord::ORIGIN)
        .ok_or_else(|| format!("No path found from {} to {}", oxygen_coord, Coord::ORIGIN))
        .map(|i| i as usize)
}

pub fn part2(input: &str) -> Result<usize, String> {
    let (maze, oxygen_coord) = explore(input.parse()?)?;
    let (mut explored, mut edges) = (HashSet::new(), HashSet::new());

    explored.insert(oxygen_coord);
    edges.insert(oxygen_coord);

    let mut minutes = 0;

    while !edges.is_empty() {
        /*
        println!(
            "Minute {}: \n{}",
            minutes,
            maze.display_with_overlay(|coord| {
                if edges.contains(coord) {
                    Some('O')
                } else if explored.contains(coord) {
                    Some('o')
                } else {
                    None
                }
            })
        );
        */

        maze.explore_step(&mut explored, &mut edges);
        minutes += 1;

        //std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!(
        "Minute {}: \n{}",
        minutes,
        maze.display_with_overlay(|coord| {
            if explored.contains(coord) {
                Some('O')
            } else {
                None
            }
        })
    );

    Ok(minutes - 1)
}

fn explore(intcode: Intcode) -> Result<(Maze, Coord), String> {
    let mut maze = Maze::default();
    let mut droids: HashMap<Coord, Intcode> = HashMap::new();
    let mut new_droids: HashMap<Coord, Intcode> = HashMap::new();
    let mut oxygen_coord = None;
    let mut legal_moves = Vec::with_capacity(4);

    droids.insert(Coord::ORIGIN, intcode);

    while !droids.is_empty() {
        /*
        println!(
            "{}",
            maze.display_with_overlay(|coord| {
                if droids.contains_key(coord) {
                    Some('D')
                } else if oxygen_coord.as_ref() == Some(coord) {
                    Some('O')
                } else {
                    None
                }
            })
        );
        */

        for (coord, mut droid) in droids.drain() {
            for &direction in Direction::ALL {
                let location = coord + direction.into();

                if !maze.contains_key(&location) {
                    match move_droid(&mut droid, direction) {
                        Some(0) => {
                            maze.insert(location, Tile::Wall);
                        }
                        Some(1) => {
                            maze.insert(location, Tile::Floor);
                            move_droid(&mut droid, direction.reverse());
                            legal_moves.push(direction);
                        }
                        Some(2) => {
                            maze.insert(location, Tile::Floor);
                            move_droid(&mut droid, direction.reverse());
                            legal_moves.push(direction);

                            oxygen_coord = Some(location);
                        }
                        Some(i) => return Err(format!("Invalid response from program: {}", i)),
                        None => return Err("No response from program.".to_string()),
                    }
                }
            }

            while legal_moves.len() > 1 {
                let direction = legal_moves.pop().unwrap();
                let mut droid_clone = droid.clone();
                move_droid(&mut droid_clone, direction);
                new_droids.insert(coord + direction.into(), droid_clone);
            }

            if let Some(direction) = legal_moves.pop() {
                move_droid(&mut droid, direction);
                new_droids.insert(coord + direction.into(), droid);
            }
        }

        mem::swap(&mut droids, &mut new_droids);

        //std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok((
        maze,
        oxygen_coord.ok_or_else(|| "No oxygen generator found, have fun breathing!".to_string())?,
    ))
}

fn move_droid(droid: &mut Intcode, direction: Direction) -> Option<isize> {
    droid.input.push(match direction {
        Direction::North => 1,
        Direction::East => 4,
        Direction::South => 2,
        Direction::West => 3,
    });

    droid.run();

    droid.output.pop()
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(248), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(382), part2(include_str!("input.txt")));
    }
}
