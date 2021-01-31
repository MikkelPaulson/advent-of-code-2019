use std::collections::{HashMap, HashSet};

use super::map::{Coord, CoordDiff};
use super::maze::{Maze, Tile};

pub fn part1(input: &str) -> Result<usize, String> {
    let (maze, key_doors) = parse(input)?;

    println!(
        "{}",
        maze.display_with_overlay(|coord| if coord == &Coord::ORIGIN {
            Some('@')
        } else {
            None
        })
    );

    let (path, length) = explore(Coord::ORIGIN, &maze, key_doors, None, 0)
        .ok_or_else(|| "No paths found.".to_string())?;

    println!(
        "Shortest path is {} at {} moves.",
        path.chars().rev().collect::<String>(),
        length
    );

    Ok(length)
}

type KeyDoor = HashMap<Coord, (char, Option<Coord>)>;

fn explore(
    start_point: Coord,
    maze: &Maze,
    key_doors: KeyDoor,
    high_score: Option<usize>,
    mut distance: usize,
) -> Option<(String, usize)> {
    if let Some(i) = high_score {
        println!("Shortest is {}, I have {}.", i, distance);
    }

    let (mut explored, mut edges) = (HashSet::new(), HashSet::new());
    let mut best_path = None;
    let high_score = high_score.unwrap_or(std::usize::MAX);

    let keys = key_doors.keys().copied().collect();
    let door_tiles = key_doors
        .values()
        .filter_map(|(_, coord)| coord.map(|coord| (coord, Tile::Wall)))
        .collect();

    explored.insert(start_point);
    edges.insert(start_point);

    while edges.len() > 0 && distance < high_score {
        // Are any keys found in this iteration?
        for key_coord in edges.intersection(&keys) {
            // I just picked up the last key! Short-circuit the rest of the logic.
            if keys.len() == 1 {
                return Some((key_doors.get(key_coord).unwrap().0.to_string(), distance));
            }

            let mut key_doors_clone = key_doors.clone();
            let (label, _) = key_doors_clone.remove(key_coord).unwrap();

            println!(
                "{}",
                maze.display_with_overlay(|coord| {
                    if coord == &start_point {
                        Some('@')
                    } else if let Some((symbol, _)) = key_doors.get(coord) {
                        Some(if coord == key_coord {
                            symbol.to_ascii_uppercase()
                        } else {
                            *symbol
                        })
                    } else if explored.contains(coord) {
                        Some('+')
                    } else {
                        None
                    }
                })
            );

            // Explore a new route from this location.
            if let Some(mut other_path) = explore(
                *key_coord,
                maze,
                key_doors_clone,
                best_path.clone().map(|(_, i)| i),
                distance,
            ) {
                other_path.0.push(label);
                best_path = Some(other_path);
            }
        }

        distance += 1;
        maze.explore_step_with_overlay(&mut explored, &mut edges, &door_tiles);
    }

    if let Some(path) = best_path {
        if path.1 <= high_score {
            Some(path)
        } else {
            None
        }
    } else {
        None
    }
}

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
                    Tile::Floor
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
                (
                    key_coord - offset,
                    (c, doors.remove(&c).map(|door| door - offset)),
                )
            })
            .collect(),
    ))
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn part1_examples() {
        assert_eq!(Ok(8), part1(include_str!("test1.txt")));
        assert_eq!(Ok(86), part1(include_str!("test2.txt")));
        assert_eq!(Ok(132), part1(include_str!("test3.txt")));
        assert_eq!(Ok(136), part1(include_str!("test4.txt")));
        assert_eq!(Ok(81), part1(include_str!("test5.txt")));
    }

    #[test]
    #[ignore]
    fn part1_solution() {
        assert_eq!(Ok(0), part1(include_str!("input.txt")));
    }
}
