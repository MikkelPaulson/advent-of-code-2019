use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod intcode;
mod map;
mod math;
mod maze;

fn main() -> Result<(), String> {
    let puzzle: Puzzle = env::args()
        .nth(1)
        .ok_or_else(|| "Missing expected day.part.".to_string())
        .and_then(|arg| arg.parse())?;

    puzzle.run().map(|output| println!("{}", output))
}

struct Puzzle {
    day: u8,
    part: u8,
}

impl Puzzle {
    pub fn try_new(day: u8, part: u8) -> Result<Self, String> {
        if (1..=25).contains(&day) && (1..=2).contains(&part) {
            Ok(Self { day, part })
        } else {
            Err(format!(
                "Invalid day.part syntax, expected [1-25].[1-2],  got {}.{}",
                day, part
            ))
        }
    }

    pub fn run(&self) -> Result<u64, String> {
        let input = self.get_input()?;

        match (self.day, self.part) {
            (1, 1) => day1::part1(&input),
            (1, 2) => day1::part2(&input),
            (2, 1) => day2::part1(&input),
            (2, 2) => day2::part2(&input),
            (3, 1) => day3::part1(&input),
            (3, 2) => day3::part2(&input),
            (4, 1) => day4::part1(&input),
            (4, 2) => day4::part2(&input),
            (5, 1) => day5::part1(&input),
            (5, 2) => day5::part2(&input),
            (6, 1) => day6::part1(&input),
            (6, 2) => day6::part2(&input),
            (7, 1) => day7::part1(&input),
            (7, 2) => day7::part2(&input),
            (8, 1) => day8::part1(&input),
            (8, 2) => day8::part2(&input),
            (9, 1) => day9::part1(&input),
            (9, 2) => day9::part2(&input),
            (10, 1) => day10::part1(&input),
            (10, 2) => day10::part2(&input),
            (11, 1) => day11::part1(&input),
            (11, 2) => day11::part2(&input),
            (12, 1) => day12::part1(&input),
            (12, 2) => day12::part2(&input),
            (13, 1) => day13::part1(&input),
            (13, 2) => day13::part2(&input),
            (14, 1) => day14::part1(&input),
            (14, 2) => day14::part2(&input),
            (15, 1) => day15::part1(&input),
            (15, 2) => day15::part2(&input),
            (16, 1) => day16::part1(&input),
            (16, 2) => day16::part2(&input),
            (17, 1) => day17::part1(&input),
            (17, 2) => day17::part2(&input),
            (18, 1) => day18::part1(&input),
            (18, 2) => day18::part2(&input),
            (19, 1) => day19::part1(&input),
            (19, 2) => day19::part2(&input),
            (20, 1) => day20::part1(&input),
            (20, 2) => day20::part2(&input),
            (21, 1) => day21::part1(&input),
            (21, 2) => day21::part2(&input),
            (22, 1) => day22::part1(&input),
            (23, 1) => day23::part1(&input),
            (23, 2) => day23::part2(&input),
            (25, 1) => day25::part1(&input),
            (day, part) => Err(format!(
                "Day {} part {} has not yet been implemented.",
                day, part
            )),
        }
    }

    fn get_input(&self) -> Result<String, String> {
        let mut buffer = String::new();

        if atty::is(atty::Stream::Stdin) {
            File::open(format!("src/day{}/input.txt", self.day))
                .map_err(|e| format!("{}", e))?
                .read_to_string(&mut buffer)
                .map_err(|e| format!("{}", e))?;
        } else {
            let mut stdin = io::stdin();
            stdin
                .read_to_string(&mut buffer)
                .map_err(|e| format!("{}", e))?;
        }

        Ok(buffer)
    }
}

impl str::FromStr for Puzzle {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut split = raw.split('.');
        let day = split
            .next()
            .ok_or("Missing day.")?
            .parse()
            .map_err(|e| format!("{:?}", e))?;
        let part = split
            .next()
            .ok_or("Missing part.")?
            .parse()
            .map_err(|e| format!("{:?}", e))?;

        split
            .next()
            .map(|s| -> Result<(), String> { Err(format!("Unexpected input part: {}", s)) })
            .transpose()?;

        Self::try_new(day, part)
    }
}
