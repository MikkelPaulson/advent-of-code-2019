use std::env;
use std::io;
use std::str;

mod day1;
mod day17;
mod day2;
mod day3;
mod day4;
mod day5;
mod day9;

mod intcode;

fn main() -> Result<(), &'static str> {
    let puzzle: Puzzle = env::args()
        .nth(1)
        .ok_or("Missing expected day.part")
        .and_then(|arg| arg.parse())?;

    puzzle
        .run(Box::new(io::stdin()))
        .map(|output| println!("{}", output))
}

struct Puzzle {
    day: u8,
    part: u8,
}

impl Puzzle {
    pub fn try_new(day: u8, part: u8) -> Result<Self, &'static str> {
        if (1..=25).contains(&day) && (1..=2).contains(&part) {
            Ok(Self { day, part })
        } else {
            Err("Invalid day.part syntax, expected [1-25].[1-2]")
        }
    }

    pub fn run(&self, input: Box<dyn io::Read>) -> Result<usize, &'static str> {
        match (self.day, self.part) {
            (1, 1) => day1::part1(input),
            (1, 2) => day1::part2(input),
            (2, 1) => day2::part1(input),
            (2, 2) => day2::part2(input),
            (3, 1) => day3::part1(input),
            (3, 2) => day3::part2(input),
            (4, 1) => day4::part1(input),
            (4, 2) => day4::part2(input),
            (5, 1) => day5::part1(input),
            (5, 2) => day5::part2(input),
            (9, 1) => day9::part1(input),
            (9, 2) => day9::part2(input),
            (17, 1) => day17::part1(input),
            (17, 2) => day17::part2(input),
            _ => Err("That day/part does not yet exist"),
        }
    }
}

impl str::FromStr for Puzzle {
    type Err = &'static str;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut split = raw.split('.');
        let day = split
            .next()
            .and_then(|day| day.parse().ok())
            .ok_or("Invalid day")?;
        let part = split
            .next()
            .and_then(|day| day.parse().ok())
            .ok_or("Invalid part")?;
        split
            .next()
            .map(|_| -> Result<(), &'static str> { Err("Invalid input") })
            .transpose()?;

        Self::try_new(day, part)
    }
}
