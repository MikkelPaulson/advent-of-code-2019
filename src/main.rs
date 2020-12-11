use std::env;
use std::io;
use std::str;

mod day1;

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

    pub fn run(&self, input: Box<dyn io::Read>) -> Result<String, &'static str> {
        match (self.day, self.part) {
            (1, 1) => day1::part1(input),
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
