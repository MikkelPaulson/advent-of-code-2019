use std::collections::HashSet;

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

fn bit_for(coord: Coord) -> u32 {
    if !(0..5).contains(&coord.y) || !(0..5).contains(&coord.x) {
        0
    } else {
        1 << (coord.y * 5 + coord.x)
    }
}

fn bug_at(map: u32, coord: Coord) -> bool {
    map & bit_for(coord) != 0
}

fn print_map(map: u32) {
    println!("{}", map);

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

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn part1_examples() {
        assert_eq!(Ok(2129920), part1(include_str!("test1.txt")));
    }

    #[test]
    fn part1_solution() {
        assert_eq!(Ok(18842609), part1(include_str!("input.txt")));
    }
}
