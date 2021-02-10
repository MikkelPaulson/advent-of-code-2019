use std::cmp;
use std::collections::{hash_map, BinaryHeap, HashMap, HashSet};
use std::mem;
use std::rc::Rc;

use super::map::{Coord, CoordDiff};
use super::maze::{Maze, Tile};

pub fn part1(input: &str) -> Result<usize, String> {
    let (maze, key_doors) = parse(input)?;

    explore(&maze, &key_doors, &[Coord::ORIGIN])
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

    explore(
        &maze,
        &key_doors,
        &[[-1 as isize, -1], [-1, 1], [1, -1], [1, 1]]
            .iter()
            .map(|c| c.clone().into())
            .collect::<Vec<Coord>>(),
    )
}

fn explore(maze: &Maze, key_doors: &KeyDoor, cursors: &[Coord]) -> Result<usize, String> {
    println!(
        "{}",
        maze.display_with_overlay(|coord| key_doors.get(coord).map(|(c, _)| *c))
    );

    let mut routes = BinaryHeap::new();
    routes.push(get_route(maze, key_doors, cursors));

    let key_coords: HashMap<char, Coord> = key_doors.iter().map(|(k, (c, _))| (*c, *k)).collect();
    let mut path_cache: HashMap<[Coord; 2], u32> = HashMap::new();
    let mut route_cache: HashMap<(u32, Vec<Coord>), u32> = HashMap::new();
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

        for (i, section) in route.sections.iter().enumerate() {
            let mut key_options: HashSet<char> = section
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

                route.sections[i].location = *key_coords
                    .get(&key)
                    .ok_or_else(|| format!("Key {} not found", key))?;

                let distance = path_cache
                    .entry([section.location, route.sections[i].location])
                    .or_insert_with(|| {
                        maze.get_path_len_with_overlay(
                            section.location,
                            route.sections[i].location,
                            &open_all_doors,
                        )
                        .unwrap()
                    })
                    .to_owned();
                route.distance += distance;

                println!(
                    "section {}: moving {} spaces from {:?} to {:?} to pick up key {}.",
                    i, distance, section.location, route.sections[i].location, key
                );

                match route_cache.entry(route.get_cache_key()) {
                    hash_map::Entry::Occupied(o) if o.get() > &route.distance => {
                        routes.push(route);
                    }
                    hash_map::Entry::Occupied(_) => {
                        println!("Route already visited, skipping.");
                    }
                    e => {
                        e.or_insert(route.distance);
                        routes.push(route);
                    }
                }
            }
        }

        if key_option_count == 0 {
            return if route.sections.iter().any(|q| {
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

fn get_route(maze: &Maze, key_doors: &KeyDoor, cursors: &[Coord]) -> Route {
    let mut paths = Vec::new();
    let mut route = Route::default();

    let mut maze_states = Vec::new();
    let mut maze_states_next = Vec::new();

    for cursor in cursors {
        let mut section = Section::default();
        section.location = cursor.to_owned().into();

        let mut maze_state = MazeState::default();
        maze_state.cursor = cursor.clone();
        maze_states.push(maze_state);

        let mut explored = HashSet::new();

        while !maze_states.is_empty() {
            for maze_state in maze_states.drain(..) {
                println!("{:?}", maze_state);

                let mut coords: Vec<Coord> = CoordDiff::DIRECTIONS
                    .iter()
                    .map(|direction| maze_state.cursor + *direction)
                    .filter(|coord| !explored.contains(coord) && maze.contains_key(coord))
                    .collect();

                if coords.is_empty() && !maze_state.path.is_empty() {
                    paths.push(maze_state.path.clone());
                }

                for coord in coords.drain(..) {
                    let mut maze_state = maze_state.clone();

                    if let Some((c, _)) = key_doors.get(&coord) {
                        maze_state.path.push(*c);
                    } else if let Some(Tile::Door(c)) = maze.get(&coord) {
                        maze_state.path.push(c.to_ascii_uppercase());
                    }

                    explored.insert(coord);
                    maze_state.cursor = coord;

                    maze_states_next.push(maze_state);
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

        section.key_paths = Rc::new(key_paths);

        println!("{:?}", section);

        route.sections.push(section);
    }

    route
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Route {
    open_doors: HashSet<char>,
    sections: Vec<Section>,
    distance: u32,
}

impl Route {
    pub fn get_cache_key(&self) -> (u32, Vec<Coord>) {
        (
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
            self.sections
                .iter()
                .map(|section| section.location)
                .collect(),
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
struct Section {
    key_paths: Rc<Vec<String>>,
    location: Coord,
}

type KeyDoor = HashMap<Coord, (char, Option<Coord>)>;

#[derive(Debug, Default, Clone)]
struct MazeState {
    cursor: Coord,
    path: String,
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
