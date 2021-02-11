use std::collections::HashMap;

use super::map::{Coord, Direction};
use super::maze::{Maze, Tile};

pub fn part1(input: &str) -> Result<usize, String> {
    let (maze, start_coord, end_coord) = parse(input)?;

    println!(
        "{}",
        maze.display_with_overlay(|c| if c == &start_coord {
            Some('@')
        } else if c == &end_coord {
            Some('O')
        } else {
            None
        })
    );

    maze.get_path_len(start_coord, end_coord)
        .map(|i| i as usize)
        .ok_or_else(|| format!("No path found from {} to {}.", start_coord, end_coord))
}

fn parse(input: &str) -> Result<(Maze, Coord, Coord), String> {
    let input_lines = input
        .trim_end_matches('\n')
        .split('\n')
        .collect::<Vec<&str>>();

    let mut maze = Maze::default();
    let mut portals: HashMap<String, Vec<(Coord, Direction)>> = HashMap::new();

    for (y, line) in input_lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = [x, y].into();

            if c == '.' {
                maze.entry(coord).or_insert(Tile::Floor);
            } else if c.is_ascii_uppercase() {
                for &direction in Direction::ALL.iter() {
                    let diff = direction.into();
                    // Expect a . in two spaces if we're facing in the correct direction.
                    let portal_coord = coord + diff + diff;
                    if portal_coord.x >= 0
                        && portal_coord.y >= 0
                        && input_lines
                            .get(portal_coord.y as usize)
                            .unwrap_or(&"")
                            .chars()
                            .nth(portal_coord.x as usize)
                            .map(|c| c == '.')
                            .unwrap_or(false)
                    {
                        // Expect another letter in one space if we're at the correct offset.
                        let other_coord = coord + diff;
                        if let Some(other_c) = input_lines
                            .get(other_coord.y as usize)
                            .unwrap_or(&"")
                            .chars()
                            .nth(other_coord.x as usize)
                            .filter(char::is_ascii_uppercase)
                        {
                            portals
                                .entry(if let Direction::North | Direction::West = direction {
                                    format!("{}{}", other_c, c)
                                } else {
                                    format!("{}{}", c, other_c)
                                })
                                .or_default()
                                .push((portal_coord, direction.reverse()));
                        }
                        break;
                    }
                }
            }
        }
    }

    let start = portals
        .remove("AA")
        .map(|mut coords| coords.pop())
        .flatten()
        .ok_or_else(|| "No start point found.".to_string())?;
    let end = portals
        .remove("ZZ")
        .map(|mut coords| coords.pop())
        .flatten()
        .ok_or_else(|| "No end point found.".to_string())?;

    for (label, mut portal) in portals.drain() {
        let (point1, point2) = (
            portal.pop().unwrap(),
            portal
                .pop()
                .ok_or_else(|| format!("Orphaned portal {}.", label))?,
        );

        maze.insert(
            point1.0,
            Tile::Portal {
                coord: point2.0,
                direction: point1.1,
            },
        );
        maze.insert(
            point2.0,
            Tile::Portal {
                coord: point1.0,
                direction: point2.1,
            },
        );
    }

    Ok((maze, start.0, end.0))
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn part1_examples() {
        assert_eq!(Ok(23), part1(include_str!("test1.txt")));
        assert_eq!(Ok(58), part1(include_str!("test2.txt")));
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(482), part1(include_str!("input.txt")));
    }
}
