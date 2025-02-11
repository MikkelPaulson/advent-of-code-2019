use std::collections::{HashSet, VecDeque};

use super::map::{Coord, CoordDiff};

pub fn part1(input: &str) -> Result<u64, String> {
    let mut map = parse(input);
    let mut states = HashSet::new();

    loop {
        if !states.insert(map) {
            print_map(map);
            return Ok(map as u64);
        }
        map = cycle(map);
    }
}

pub fn part2(input: &str) -> Result<u64, String> {
    part2_cycles(input, 200)
}

fn part2_cycles(input: &str, minutes: usize) -> Result<u64, String> {
    let mut map = VecDeque::from([0, parse(input), 0]);

    for _ in 0..minutes {
        map = cycle_with_overlay(map);
    }

    map.iter().enumerate().for_each(|(i, &layer)| {
        println!("\nDepth {i}:");
        print_map(layer)
    });

    Ok(count_bugs(&map))
}

fn cycle(map: u32) -> u32 {
    let mut result = 0;

    for x in 0..5 {
        for y in 0..5 {
            let coord = Coord { x, y };
            let adjacent_bugs = CoordDiff::DIRECTIONS
                .iter()
                .filter(|&&direction| bug_at(map, coord + direction))
                .count();

            result |= match (bug_at(map, coord), adjacent_bugs) {
                (_, 1) | (false, 2) => bit_for(coord),
                _ => 0,
            };
        }
    }

    result
}

fn cycle_with_overlay(map: VecDeque<u32>) -> VecDeque<u32> {
    assert_eq!(Some(&0), map.front());
    assert_eq!(Some(&0), map.back());

    let mut next_map: VecDeque<u32> = std::iter::repeat(0).take(map.len()).collect();

    for (i, layer) in next_map.iter_mut().enumerate() {
        for x in 0..5 {
            for y in 0..5 {
                /*
                if x == 3 && y == 2 && i == 1 {
                    let adjacent_bugs = directions_from([x, y], i, map.len())
                        .inspect(|v| eprintln!("Direction: {v:?}"))
                        .filter(|&(coord, layer)| map[layer] & bit_for_array(coord) != 0)
                        .inspect(|v| eprintln!("Bug found: {v:?}"))
                        .count();
                    let bit = bit_for_array([x, y]);

                    *layer |= match (map[i] & bit, adjacent_bugs) {
                        (_, 1) | (0, 2) => bit,
                        _ => 0,
                    };
                }
                */

                let adjacent_bugs = directions_from([x, y], i, map.len())
                    .filter(|&(coord, layer)| map[layer] & bit_for_array(coord) != 0)
                    .count();
                let bit = bit_for_array([x, y]);

                *layer |= match (map[i] & bit, adjacent_bugs) {
                    (_, 1) | (0, 2) => bit,
                    _ => 0,
                };
            }
        }
    }

    if *next_map.front().unwrap() > 0 {
        next_map.push_front(0);
    }

    if *next_map.back().unwrap() > 0 {
        next_map.push_back(0);
    }

    next_map
}

fn bit_for(coord: Coord) -> u32 {
    if !(0..5).contains(&coord.y) || !(0..5).contains(&coord.x) {
        0
    } else {
        1 << (coord.y * 5 + coord.x)
    }
}

fn bit_for_array(coord: [u8; 2]) -> u32 {
    let [x, y] = coord;
    1 << (y * 5 + x)
}

fn directions_from(
    coord: [u8; 2],
    layer: usize,
    len: usize,
) -> impl Iterator<Item = ([u8; 2], usize)> {
    let [x, y] = coord;

    let up_iter = match y {
        0 if layer == 0 => None,
        0 => Some(([2, 1], layer - 1)),
        2 | 3 if x == 2 => None,
        _ => Some(([x, y - 1], layer)),
    }
    .into_iter()
    .chain(
        if coord == [2, 3] && layer + 1 < len {
            Some((0..5).map(move |x| ([x, 4], layer + 1)))
        } else {
            None
        }
        .into_iter()
        .flatten(),
    );

    let down_iter = match y {
        1 | 2 if x == 2 => None,
        4 if layer == 0 => None,
        4 => Some(([2, 3], layer - 1)),
        _ => Some(([x, y + 1], layer)),
    }
    .into_iter()
    .chain(
        if coord == [2, 1] && layer + 1 < len {
            Some((0..5).map(move |x| ([x, 0], layer + 1)))
        } else {
            None
        }
        .into_iter()
        .flatten(),
    );

    let left_iter = match x {
        0 if layer == 0 => None,
        0 => Some(([1, 2], layer - 1)),
        2 | 3 if y == 2 => None,
        _ => Some(([x - 1, y], layer)),
    }
    .into_iter()
    .chain(
        if coord == [3, 2] && layer + 1 < len {
            Some((0..5).map(move |y| ([4, y], layer + 1)))
        } else {
            None
        }
        .into_iter()
        .flatten(),
    );

    let right_iter = match x {
        1 | 2 if y == 2 => None,
        4 if layer == 0 => None,
        4 => Some(([3, 2], layer - 1)),
        _ => Some(([x + 1, y], layer)),
    }
    .into_iter()
    .chain(
        if coord == [1, 2] && layer + 1 < len {
            Some((0..5).map(move |y| ([0, y], layer + 1)))
        } else {
            None
        }
        .into_iter()
        .flatten(),
    );

    std::iter::empty()
        .chain(up_iter)
        .chain(down_iter)
        .chain(left_iter)
        .chain(right_iter)
}

fn bug_at(map: u32, coord: Coord) -> bool {
    map & bit_for(coord) != 0
}

fn print_map(map: u32) {
    for i in 0..25 {
        if map & 1 << i == 0 {
            print!(".");
        } else {
            print!("#");
        }

        if i % 5 == 4 {
            print!("\n");
        }
    }
}

fn parse(input: &str) -> u32 {
    input
        .chars()
        .filter(|c| ['.', '#'].contains(c))
        .enumerate()
        .fold(0, |acc, (i, c)| if c == '#' { acc + (1 << i) } else { acc })
}

fn count_bugs(map: &VecDeque<u32>) -> u64 {
    map.into_iter().map(|&i| i.count_ones()).sum::<u32>() as u64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(Ok(2129920), part1(include_str!("test1.txt")));
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(18842609), part1(include_str!("input.txt")));
    }

    #[test]
    fn part2_example() {
        assert_eq!(Ok(99), part2_cycles(include_str!("test1.txt"), 10));
    }

    #[test]
    fn part2_solution() {
        assert_eq!(Ok(2059), part2(include_str!("input.txt")));
    }

    #[test]
    ///      |     |         |     |
    ///   1  |  2  |    3    |  4  |  5
    ///      |     |         |     |
    /// -----+-----+---------+-----+-----
    ///      |     |         |     |
    ///   6  |  7  |    8    |  9  |  10
    ///      |     |         |     |
    /// -----+-----+---------+-----+-----
    ///      |     |A|B|C|D|E|     |
    ///      |     |-+-+-+-+-|     |
    ///      |     |F|G|H|I|J|     |
    ///      |     |-+-+-+-+-|     |
    ///  11  | 12  |K|L|?|N|O|  14 |  15
    ///      |     |-+-+-+-+-|     |
    ///      |     |P|Q|R|S|T|     |
    ///      |     |-+-+-+-+-|     |
    ///      |     |U|V|W|X|Y|     |
    /// -----+-----+---------+-----+-----
    ///      |     |         |     |
    ///  16  | 17  |    18   |  19 |  20
    ///      |     |         |     |
    /// -----+-----+---------+-----+-----
    ///      |     |         |     |
    ///  21  | 22  |    23   |  24 |  25
    ///      |     |         |     |
    fn directions_from_test() {
        assert_eq!(
            0,
            directions_from([2, 2], 1, 3).collect::<Vec<_>>().len(),
            "Tile ? has no adjacent tiles.",
        );

        assert_eq!(
            vec![([3, 2], 0), ([3, 4], 0), ([2, 3], 0), ([4, 3], 0)],
            directions_from([3, 3], 0, 2).collect::<Vec<_>>(),
            "Tile 19 has four adjacent tiles: 14, 18, 20, and 24.",
        );

        assert_eq!(
            vec![([1, 0], 1), ([1, 2], 1), ([0, 1], 1), ([2, 1], 1)],
            directions_from([1, 1], 1, 2).collect::<Vec<_>>(),
            "Tile G has four adjacent tiles: B, F, H, and L.",
        );

        assert_eq!(
            vec![([2, 1], 0), ([3, 1], 1), ([2, 0], 1), ([4, 0], 1)],
            directions_from([3, 0], 1, 2).collect::<Vec<_>>(),
            "Tile D has four adjacent tiles: 8, C, E, and I.",
        );

        assert_eq!(
            vec![([2, 1], 0), ([4, 1], 1), ([3, 0], 1), ([3, 2], 0)],
            directions_from([4, 0], 1, 2).collect::<Vec<_>>(),
            "Tile E has four adjacent tiles: 8, D, 14, and J.",
        );

        assert_eq!(
            vec![
                ([3, 1], 0),
                ([3, 3], 0),
                ([4, 0], 1),
                ([4, 1], 1),
                ([4, 2], 1),
                ([4, 3], 1),
                ([4, 4], 1),
                ([4, 2], 0),
            ],
            directions_from([3, 2], 0, 2).collect::<Vec<_>>(),
            "Tile 14 has *eight* adjacent tiles: 9, E, J, O, T, Y, 15, and 19.",
        );

        assert_eq!(
            vec![([3, 1], 1), ([3, 3], 1), ([4, 2], 1)],
            directions_from([3, 2], 1, 2).collect::<Vec<_>>(),
            "Tile N has *three* adjacent tiles: I, O, S, and nothing within the subgrid because \
            the recursion depth is exceeded.",
        );
    }
}
