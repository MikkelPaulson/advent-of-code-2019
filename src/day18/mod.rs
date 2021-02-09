use std::cmp;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::mem;
use std::rc::Rc;

use super::map::{Coord, CoordDiff};
use super::maze::{Maze, Tile};

pub fn part1(input: &str) -> Result<usize, String> {
    let (maze, key_doors) = parse(input)?;

    let distance = explore(Coord::ORIGIN, &maze, &key_doors)?;

    Ok(distance)
}

pub fn part2(input: &str) -> Result<usize, String> {
    // drain_filter() isn't stable yet, so have to do this the messy way.
    let (maze, key_doors) = {
        let (mut maze, key_doors) = parse(input)?;
        maze.retain(|coord, _| {
            coord != &Coord::ORIGIN
                && (coord.x != 0 || coord.y.abs() != 1)
                && (coord.y != 0 || coord.x.abs() != 1)
        });
        (maze, key_doors)
    };

    println!(
        "{}",
        maze.display_with_overlay(|coord| key_doors.get(coord).map(|(c, _)| *c))
    );

    let mut routes = BinaryHeap::new();
    routes.push(get_route(&maze, &key_doors));

    let key_coords: HashMap<char, Coord> = key_doors.iter().map(|(k, (c, _))| (*c, *k)).collect();
    let mut path_cache: HashMap<[Coord; 2], u32> = HashMap::new();
    let mut route_cache: HashSet<(u32, u32, [Coord; 4])> = HashSet::new();
    let open_all_doors: HashMap<Coord, Tile> = key_doors
        .values()
        .filter_map(|(_, opt)| opt.map(|coord| (coord, Tile::Floor)))
        .collect();

    while let Some(route) = routes.pop() {
        println!(
            "\n\nMin length of {} routes is {}: {:?}\n",
            routes.len() + 1,
            route.distance,
            route
        );

        // If the shortest route on the heap has collected all keys, we're done!
        let mut key_option_count = 0;

        for (i, quadrant) in route.quadrants.iter().enumerate() {
            let mut key_options: HashSet<char> = quadrant
                .key_paths
                .iter()
                .filter_map(|s| {
                    s.trim_start_matches(|c: char| route.open_doors.contains(&c))
                        .chars()
                        .next()
                        .filter(|c| c.is_ascii_lowercase())
                })
                .collect();
            key_option_count += key_options.len();

            for key in key_options.drain() {
                let mut route = route.clone();

                route.open_doors.insert(key);
                route.open_doors.insert(key.to_ascii_uppercase());

                route.quadrants[i].location = *key_coords
                    .get(&key)
                    .ok_or_else(|| format!("Key {} not found", key))?;

                let distance = path_cache
                    .entry([quadrant.location, route.quadrants[i].location])
                    .or_insert_with(|| {
                        maze.get_path_len_with_overlay(
                            quadrant.location,
                            route.quadrants[i].location,
                            &open_all_doors,
                        )
                        .unwrap()
                    })
                    .to_owned();
                route.distance += distance;

                println!(
                    "Quadrant {}: moving {} spaces from {:?} to {:?} to pick up key {}.",
                    i, distance, quadrant.location, route.quadrants[i].location, key
                );

                if route_cache.insert(route.get_cache_key()) {
                    routes.push(route);
                } else {
                    println!("Route already visited, skipping.");
                }
            }
        }

        if key_option_count == 0 {
            return if route.quadrants.iter().any(|q| {
                q.key_paths.iter().any(|p| {
                    !p.trim_start_matches(|c: char| route.open_doors.contains(&c))
                        .is_empty()
                })
            }) {
                Err(format!("Ran out of available keys: {:?}", route))
            } else {
                Ok(route.distance as usize)
            };
        }
    }

    Err("No route found.".to_string())
}

fn get_route(maze: &Maze, key_doors: &KeyDoor) -> Route {
    let mut paths = Vec::new();
    let mut route = Route::default();

    let mut maze_states = Vec::new();
    let mut maze_states_next = Vec::new();

    for (i, cursor) in [[-1 as isize, -1], [-1, 1], [1, -1], [1, 1]]
        .iter()
        .enumerate()
    {
        let quadrant = &mut route.quadrants[i];
        quadrant.location = cursor.to_owned().into();

        let mut maze_state = MazeState::default();
        maze_state.explored.insert(cursor.clone().into());
        maze_state.edges.insert(cursor.clone().into());
        maze_states.push(maze_state);

        while !maze_states.is_empty() {
            for maze_state in maze_states.drain(..) {
                if let Some(coord) = maze_state.edges.iter().last() {
                    let mut new_coords: Vec<Coord> = CoordDiff::DIRECTIONS
                        .iter()
                        .map(|direction| *coord + *direction)
                        .filter(|coord| {
                            !maze_state.explored.contains(&coord) && maze.contains_key(&coord)
                        })
                        .collect();

                    if new_coords.is_empty() && !maze_state.path.is_empty() {
                        paths.push(maze_state.path.clone());
                    }

                    for new_coord in new_coords.drain(..) {
                        let mut maze_state = maze_state.clone();

                        if let Some((c, _)) = key_doors.get(&new_coord) {
                            maze_state.path.push(*c);
                        } else if let Some(Tile::Door(c)) = maze.get(&new_coord) {
                            maze_state.path.push(c.to_ascii_uppercase());
                        }

                        mem::swap(&mut maze_state.explored, &mut maze_state.edges);
                        maze_state.edges.clear();
                        maze_state.edges.insert(new_coord);

                        maze_states_next.push(maze_state);
                    }
                }
            }
            mem::swap(&mut maze_states, &mut maze_states_next);
        }

        paths.sort();
        let mut key_paths: Vec<String> = Vec::new();
        while let Some(path) = paths.pop() {
            match key_paths.last() {
                Some(key_path) => {
                    if !key_path.starts_with(&path) {
                        key_paths.push(path);
                    }
                }
                None => key_paths.push(path),
            }
        }

        quadrant.key_paths = Rc::new(key_paths);

        println!("{:?}", quadrant);
    }

    route
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Route {
    open_doors: HashSet<char>,
    quadrants: [Quadrant; 4],
    distance: u32,
}

impl Route {
    pub fn get_cache_key(&self) -> (u32, u32, [Coord; 4]) {
        (
            self.distance,
            self.open_doors
                .iter()
                .filter_map(|c| {
                    if c.is_ascii_lowercase() {
                        Some(1 << (*c as u8 - 'a' as u8) as u32)
                    } else {
                        None
                    }
                })
                .sum(),
            [
                self.quadrants[0].location,
                self.quadrants[1].location,
                self.quadrants[2].location,
                self.quadrants[3].location,
            ],
        )
    }
}

impl cmp::Ord for Route {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl cmp::PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Quadrant {
    key_paths: Rc<Vec<String>>,
    location: Coord,
}

type KeyDoor = HashMap<Coord, (char, Option<Coord>)>;

#[derive(Debug, Default, Clone)]
struct MazeState {
    key_bitfield: u32,
    explored: HashSet<Coord>,
    edges: HashSet<Coord>,
    overlay: HashMap<Coord, Tile>,
    path: String,
}

fn explore(start_coord: Coord, maze: &Maze, key_doors: &KeyDoor) -> Result<usize, String> {
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
    let mut explored: HashMap<u32, HashSet<Coord>> = HashMap::new();

    let mut steps = 0;

    while !maze_states.is_empty() {
        steps += 1;

        if let Some(maze_state) = maze_states.first() {
            println!(
                "Step {}, tracking {} states.\n{:?}\n{}",
                steps,
                maze_states.len(),
                maze_state,
                maze.display_with_overlay(|coord| if maze_state.edges.contains(coord) {
                    Some('@')
                } else if maze_state.explored.contains(coord) {
                    Some('+')
                } else if maze_state.overlay.contains_key(coord) {
                    Some('.')
                } else if let Some((c, _)) = key_doors.get(coord) {
                    if maze_state.key_bitfield & get_key_bit(*c) > 0 {
                        None
                    } else {
                        Some(*c)
                    }
                } else {
                    None
                })
            );
        }

        for mut maze_state in maze_states.drain(..) {
            if let Some(coords) = explored.get_mut(&maze_state.key_bitfield) {
                if !coords.is_disjoint(&maze_state.edges) {
                    maze_state.edges = maze_state.edges.difference(&coords).copied().collect();
                }
                maze_state.edges.iter().for_each(|coord| {
                    coords.insert(*coord);
                });
            } else {
                explored.insert(maze_state.key_bitfield, maze_state.edges.clone());
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
                    if maze_state.key_bitfield & get_key_bit(*c) != 0 {
                        continue;
                    }
                }

                let mut new_state = MazeState::default();

                new_state.overlay = maze_state.overlay.clone();
                new_state.key_bitfield = maze_state.key_bitfield;

                if let Some((c, door_coord_opt)) = key_doors.get(&key_coord) {
                    new_state.key_bitfield |= get_key_bit(*c);

                    if let Some(door_coord) = door_coord_opt {
                        new_state.overlay.insert(*door_coord, Tile::Floor);
                    }
                }

                // Just picked up the last key!
                if new_state.key_bitfield.count_ones() as usize == key_doors.len() {
                    return Ok(steps);
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

const fn get_key_bit(key: char) -> u32 {
    if let 'a'..='z' = key {
        1 << (key as u32 - 'a' as u32)
    } else {
        0
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
    use super::{part1, part2};

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
        assert_eq!(Ok(4676), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(Ok(8), part2(include_str!("test6.txt")));
        assert_eq!(Ok(24), part2(include_str!("test7.txt")));
        assert_eq!(Ok(32), part2(include_str!("test8.txt")));
        assert_eq!(Ok(72), part2(include_str!("test9.txt")));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(2066), part2(include_str!("input.txt")));
    }
}
