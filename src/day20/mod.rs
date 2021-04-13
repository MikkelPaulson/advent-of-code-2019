use std::collections::{BTreeSet, HashMap, HashSet};
use std::mem;

use super::map::{Coord, Direction};
use super::maze::{Maze, Tile};

pub fn part1(input: &str) -> Result<u64, String> {
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
        .map(|i| i as u64)
        .ok_or_else(|| format!("No path found from {} to {}.", start_coord, end_coord))
}

pub fn part2(input: &str) -> Result<u64, String> {
    let (maze, start_coord, end_coord) = parse(input)?;

    let (inner_portals, outer_portals) = {
        let mut portals: HashSet<Coord> = HashSet::new();
        let mut edges: HashMap<Direction, BTreeSet<i64>> = HashMap::new();

        maze.iter().for_each(|(&coord, &tile)| {
            if let Tile::Portal { direction, .. } = tile {
                portals.insert(coord);
                edges.entry(direction).or_default().insert(
                    if let Direction::North | Direction::South = direction {
                        coord.y
                    } else {
                        coord.x
                    },
                );
            }
        });

        let (mut inner_portals, mut outer_portals) = (HashSet::new(), HashSet::new());
        let (mut inner_x, mut inner_y) = (HashSet::new(), HashSet::new());

        for (direction, coords) in edges.drain() {
            let inner_coord = if let Direction::North | Direction::West = direction {
                coords.iter().last()
            } else {
                coords.iter().next()
            }
            .unwrap();

            if let Direction::West | Direction::East = direction {
                inner_x.insert(*inner_coord);
            } else {
                inner_y.insert(*inner_coord);
            }
        }

        portals.drain().for_each(|coord| {
            if inner_x.contains(&coord.x) || inner_y.contains(&coord.y) {
                inner_portals.insert(coord);
            } else {
                outer_portals.insert(coord);
            }
        });

        (inner_portals, outer_portals)
    };

    let mut mazes = {
        let mut mazes: Vec<(HashSet<Coord>, HashSet<Coord>)> = Vec::new();
        let (mut explored, mut edges) = (HashSet::new(), HashSet::new());

        explored.insert(start_coord);
        edges.insert(start_coord);

        mazes.push((explored, edges));
        mazes
    };

    let overlay: HashMap<Coord, Tile> = inner_portals
        .iter()
        .chain(outer_portals.iter())
        .map(|&coord| (coord, Tile::Floor))
        .collect();
    let overlay_outer: HashMap<Coord, Tile> = inner_portals
        .iter()
        .map(|&coord| (coord, Tile::Floor))
        .chain(outer_portals.iter().map(|&coord| (coord, Tile::Wall)))
        .collect();

    let (mut portal_edges, mut portal_edges_next): (Vec<(usize, Coord)>, Vec<(usize, Coord)>) =
        (Vec::new(), Vec::new());
    let mut step = 0;

    loop {
        if mazes
            .first()
            .map_or(false, |(_, edges)| edges.contains(&end_coord))
        {
            return Ok(step);
        }

        if mazes.last().map_or(true, |(_, edges)| !edges.is_empty()) {
            mazes.push((HashSet::new(), HashSet::new()));
        }

        let mut no_edges = true;

        for i in 0..mazes.len() - 1 {
            let (explored, edges) = &mut mazes[i];

            if no_edges && !edges.is_empty() {
                no_edges = false;
            }

            maze.explore_step_with_overlay(
                explored,
                edges,
                if i == 0 { &overlay_outer } else { &overlay },
            );

            inner_portals.intersection(edges).for_each(|&coord| {
                println!("Inner portal detected at {}, level {}", coord, i);
                portal_edges_next.push((i + 1, coord));
            });

            outer_portals.intersection(edges).for_each(|&coord| {
                println!("Outer portal detected at {}, level {}", coord, i);
                portal_edges_next.push((i - 1, coord));
            });
        }

        if no_edges {
            return Err(format!(
                "No path found from {} to {} after {} steps.",
                start_coord, end_coord, step
            ));
        }

        portal_edges.drain(..).for_each(|(i, origin_coord)| {
            if let Some(Tile::Portal { coord, .. }) = maze.get(&origin_coord) {
                println!("Jumping to level {} at {} (step {})", i, coord, step);
                mazes[i].0.insert(*coord);
                mazes[i].1.insert(*coord);
            }
        });

        mem::swap(&mut portal_edges, &mut portal_edges_next);

        step += 1;
    }
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
            let coord = [x as i64, y as i64].into();

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
    use super::{part1, part2};

    #[test]
    fn part1_examples() {
        assert_eq!(Ok(23), part1(include_str!("test1.txt")));
        assert_eq!(Ok(58), part1(include_str!("test2.txt")));
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(482), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(Ok(26), part2(include_str!("test1.txt")));
        assert_eq!(Ok(396), part2(include_str!("test3.txt")));
    }

    #[test]
    #[ignore]
    fn part2_solution() {
        assert_eq!(Ok(0), part2(include_str!("input.txt")));
    }
}
