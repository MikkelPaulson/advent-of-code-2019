use std::collections::{HashMap, HashSet};
use std::mem;

use super::map::{Coord, CoordDiff};
use super::maze::{Maze, Tile};

pub fn part1(input: &str) -> Result<usize, String> {
    let (maze, key_doors) = parse(input)?;

    let (distance, sequence) = explore(Coord::ORIGIN, &maze, &key_doors)?;

    let sequence_pretty = {
        let mut sequence_pretty = String::with_capacity(sequence.len() * 3);
        if let Some(first) = sequence.get(0) {
            sequence_pretty.push(*first);
            for c in &sequence[1..] {
                sequence_pretty.push_str(", ");
                sequence_pretty.push(*c);
            }
        }
        sequence_pretty
    };

    println!("Shortest path is {} steps: {}", distance, sequence_pretty);

    Ok(distance)
}

type KeyDoor = HashMap<Coord, (char, Option<Coord>)>;

#[derive(Debug, Default, Clone)]
struct MazeState {
    keys: Vec<char>,
    key_string: String,
    explored: HashSet<Coord>,
    edges: HashSet<Coord>,
    overlay: HashMap<Coord, Tile>,
}

fn explore(
    start_coord: Coord,
    maze: &Maze,
    key_doors: &KeyDoor,
) -> Result<(usize, Vec<char>), String> {
    let mut maze_states = {
        let mut maze_states = Vec::new();

        let mut starting_state = MazeState::default();
        starting_state.explored.insert(start_coord);
        starting_state.edges.insert(start_coord);

        maze_states.push(starting_state);
        maze_states
    };
    let mut maze_states_next = Vec::new();

    let key_coords: HashSet<Coord> = key_doors.keys().copied().collect();
    let mut explored: HashMap<String, HashSet<Coord>> = HashMap::new();

    let mut steps = 0;

    while !maze_states.is_empty() {
        steps += 1;

        println!("Step {}, tracking {} states.", steps, maze_states.len());
        if let Some(maze_state) = if steps % 2 == 0 {
            maze_states.first()
        } else {
            maze_states.last()
        } {
            println!(
                "{:?}\n{}",
                maze_state,
                maze.display_with_overlay(|coord| if maze_state.edges.contains(coord) {
                    Some('@')
                } else if maze_state.explored.contains(coord) {
                    Some('+')
                } else if maze_state.overlay.contains_key(coord) {
                    Some('.')
                } else if let Some((c, _)) = key_doors.get(coord) {
                    if maze_state.keys.iter().any(|fc| c == fc) {
                        None
                    } else {
                        Some(*c)
                    }
                } else {
                    None
                })
            );
        }

        while let Some(mut maze_state) = maze_states.pop() {
            if let Some(coords) = explored.get_mut(&maze_state.key_string) {
                if !coords.is_disjoint(&maze_state.edges) {
                    maze_state.edges = maze_state.edges.difference(&coords).copied().collect();
                    maze_state.edges.iter().for_each(|coord| {
                        coords.insert(*coord);
                    });
                }
            } else {
                explored.insert(maze_state.key_string.clone(), maze_state.edges.clone());
            }

            maze.explore_step_with_overlay(
                &mut maze_state.explored,
                &mut maze_state.edges,
                &maze_state.overlay,
            );

            // Are we picking up any keys on this pass?
            for key_coord in maze_state
                .edges
                .intersection(&key_coords)
                .copied()
                .collect::<Vec<Coord>>()
            {
                if let Some((c, _)) = key_doors.get(&key_coord) {
                    // Already have this key
                    if maze_state.key_string.contains(&[*c][..]) {
                        continue;
                    }
                }

                let mut new_state = MazeState::default();

                new_state.keys = maze_state.keys.clone();
                new_state.overlay = maze_state.overlay.clone();

                if let Some((c, door_coord_opt)) = key_doors.get(&key_coord) {
                    new_state.keys.push(*c);

                    if let Some(door_coord) = door_coord_opt {
                        new_state.overlay.insert(*door_coord, Tile::Floor);
                    }
                }

                new_state.key_string = {
                    let mut keys_sorted = new_state.keys.clone();
                    keys_sorted.sort();
                    keys_sorted.iter().collect()
                };

                // Just picked up the last key!
                if new_state.keys.len() == key_doors.len() {
                    return Ok((steps, new_state.keys));
                }

                new_state.explored.insert(key_coord);
                new_state.edges.insert(key_coord);

                maze_state.edges.remove(&key_coord);
                maze_states_next.push(new_state);
            }

            // Only continue exploring if we haven't exhausted the possibilities.
            if !maze_state.edges.is_empty() {
                maze_states_next.push(maze_state);
            }
        }

        mem::swap(&mut maze_states, &mut maze_states_next);
    }

    Err("No path to end!".to_string())
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

        if let Some(tile) = match raw_tile {
            '#' => None,
            '.' => Some(Tile::Floor),
            '@' => {
                offset = coord - Coord::ORIGIN;
                Some(Tile::Floor)
            }
            c if c.is_uppercase() => {
                doors.insert(c.to_ascii_lowercase(), coord);
                Some(Tile::Door(c))
            }
            c if c.is_lowercase() => {
                keys.insert(c, coord);
                Some(Tile::Floor)
            }
            c => {
                return Err(format!("Invalid character: {:?}", c));
            }
        } {
            raw_maze.insert(coord, tile);
        }

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
        assert_eq!(Ok(81), part1(include_str!("test5.txt")));
    }

    #[test]
    #[ignore]
    fn part1_examples_slow() {
        assert_eq!(Ok(136), part1(include_str!("test4.txt")));
    }

    #[test]
    #[ignore]
    fn part1_solution() {
        assert_eq!(Ok(4676), part1(include_str!("input.txt")));
    }
}
