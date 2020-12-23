use crate::intcode::Intcode;
use std::io::prelude::*;
use std::str;

pub fn part1(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let mut intcode = Intcode::parse(input);
    intcode.run();
    let map = String::from_utf8(intcode.output.iter().map(|c| *c as u8).collect()).unwrap();

    println!("{}", map);
    println!("{:?}", get_intersections(&map));

    Ok(get_intersections(&map)
        .iter()
        .map(|(row, col)| row * col)
        .sum())
}

fn get_intersections(map: &str) -> Vec<(usize, usize)> {
    let map_lines = map.trim_end().split('\n').collect::<Vec<&str>>();
    let mut intersections = Vec::new();

    for row in 1..map_lines.len() - 1 {
        for col in 1..map_lines[0].len() - 1 {
            if &map_lines[row][col..col + 1] != "."
                && &map_lines[row - 1][col..col + 1] != "."
                && &map_lines[row][col - 1..col] != "."
                && &map_lines[row + 1][col..col + 1] != "."
                && &map_lines[row][col..col + 1] != "."
            {
                intersections.push((row, col));
            }
        }
    }

    intersections
}
