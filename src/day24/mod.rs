use super::map::{Coord, CoordDiff};

pub fn part1(input: &str) -> Result<u64, String> {
    let mut map = parse(input);

    println!("Initial state:");
    print_map(map);

    for i in 1..5 {
        map = cycle(map);
        println!("\nAfter {} minutes:", i);
        print_map(map);
    }

    Err("Not implemented".to_string())
}

fn cycle(map: u32) -> u32 {
    let mut result = 0;

    for x in 0..5 {
        for y in 0..5 {
            let coord = Coord { x, y };
            let adjacent_bugs = CoordDiff::DIRECTIONS
                .iter()
                .filter(|&&cd| bug_at(map, coord + cd))
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
    let bit = coord.y * 5 + coord.x;
    if bit < 0 || bit >= 25 {
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
