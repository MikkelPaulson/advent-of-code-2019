use std::collections::{HashMap, HashSet};
use std::mem;
use std::ops;

use crate::map::{Coord, CoordMap, Direction};

#[derive(Clone, Debug, Default)]
pub struct Maze(HashMap<Coord, Tile>);

impl Maze {
    pub fn get_path_len(&self, origin: Coord, destination: Coord) -> Option<u32> {
        self.get_path_len_with_overlay(origin, destination, &HashMap::new())
    }

    pub fn get_path_len_with_overlay(
        &self,
        origin: Coord,
        destination: Coord,
        tiles: &HashMap<Coord, Tile>,
    ) -> Option<u32> {
        let (mut explored, mut edges) = (HashSet::new(), HashSet::new());

        explored.insert(origin);
        edges.insert(origin);

        let mut distance = 0;

        loop {
            if edges.contains(&destination) {
                break Some(distance);
            }

            if edges.is_empty() {
                break None;
            }

            /*
            println!(
                "{}",
                self.display_with_overlay(|coord| {
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

            self.explore_step_with_overlay(&mut explored, &mut edges, tiles);
            distance += 1;
        }
    }

    pub fn explore_step(&self, explored: &mut HashSet<Coord>, edges: &mut HashSet<Coord>) {
        self.explore_step_with_overlay(explored, edges, &HashMap::new())
    }

    pub fn explore_step_with_overlay(
        &self,
        explored: &mut HashSet<Coord>,
        edges: &mut HashSet<Coord>,
        tiles: &HashMap<Coord, Tile>,
    ) {
        let mut new_edges = HashSet::with_capacity(edges.len());

        for edge in edges.drain() {
            for &direction in Direction::ALL {
                let target = match tiles.get(&edge).or(self.get(&edge)) {
                    Some(&Tile::Portal {
                        coord,
                        direction: portal_direction,
                    }) if portal_direction == direction => coord,
                    Some(&Tile::Floor) | Some(&Tile::Portal { .. }) => edge + direction.into(),
                    _ => continue,
                };

                if let Some(&Tile::Floor) | Some(&Tile::Portal { .. }) =
                    tiles.get(&target).or(self.get(&target))
                {
                    if !explored.contains(&target) {
                        new_edges.insert(target);
                        explored.insert(target);
                    }
                }
            }
        }

        mem::swap(edges, &mut new_edges);
    }

    pub fn display_with_overlay<F: Fn(&Coord) -> Option<char>>(&self, overlay: F) -> String {
        let mut result = String::new();

        if let Some([min, max]) = self.get_min_max() {
            result.reserve(((max.x - min.x + 2) * (max.y - min.y + 1)) as usize);

            for coord in CoordMap::new(min, max) {
                result.push(if let Some(c) = overlay(&coord) {
                    c
                } else {
                    match self.get(&coord) {
                        None => ' ',
                        Some(Tile::Wall) => '#',
                        Some(Tile::Floor) => '.',
                        Some(Tile::Door(c)) => c.to_ascii_uppercase(),
                        Some(Tile::Portal {
                            direction: Direction::North,
                            ..
                        }) => '^',
                        Some(Tile::Portal {
                            direction: Direction::South,
                            ..
                        }) => 'v',
                        Some(Tile::Portal {
                            direction: Direction::East,
                            ..
                        }) => '>',
                        Some(Tile::Portal {
                            direction: Direction::West,
                            ..
                        }) => '<',
                    }
                });

                if coord.x == max.x {
                    result.push('\n');
                }
            }
        }

        result.shrink_to_fit();
        result
    }

    fn get_min_max(&self) -> Option<[Coord; 2]> {
        match [
            self.keys().map(|c| Some(c)).fold(None, |opt_acc, c| {
                opt_acc
                    .map(|acc: Coord| acc.min(&c.unwrap()))
                    .or(c.copied())
            }),
            self.keys().map(|c| Some(c)).fold(None, |opt_acc, c| {
                opt_acc
                    .map(|acc: Coord| acc.max(&c.unwrap()))
                    .or(c.copied())
            }),
        ] {
            [Some(a), Some(b)] => Some([a, b]),
            _ => None,
        }
    }
}

impl ops::Deref for Maze {
    type Target = HashMap<Coord, Tile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Maze {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Tile {
    Wall,
    Floor,
    Door(char),
    Portal { coord: Coord, direction: Direction },
}
