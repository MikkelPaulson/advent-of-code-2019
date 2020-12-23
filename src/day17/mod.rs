use crate::intcode::Intcode;
use std::io::prelude::*;
use std::str;

pub fn part1(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let mut intcode = Intcode::parse(input);
    intcode.run();

    let map = intcode.output_string();

    println!("{}", map);
    println!("{:?}", get_intersections(&map));

    Ok(get_intersections(&map)
        .iter()
        .map(|(row, col)| row * col)
        .sum())
}

pub fn part2(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let mut intcode = Intcode::parse(input);
    intcode.set(0, 2);

    // Main movement routine
    intcode.input_str(&"A,A,B,C,B,C,B,C,B,A\n");

    // Function A
    intcode.input_str(&"R,10,L,12,R,6\n");

    // Function B
    intcode.input_str(&"R,6,R,10,R,12,R,6\n");

    // Function C
    intcode.input_str(&"R,10,L,12,L,12\n");

    // "Continuous video feed"
    intcode.input_str(&"n\n");
    intcode.run();

    let result = intcode.output.pop().map(|i| i as usize).ok_or("No output.");
    println!("{}", intcode.output_string());
    result
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
