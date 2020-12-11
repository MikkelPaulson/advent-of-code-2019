use std::io;
use std::io::prelude::*;

pub fn part1(input: Box<dyn Read>) -> Result<String, &'static str> {
    let mut sum = 0;
    for mass in parse(input) {
        sum += mass.div_euclid(3) - 2;
    }
    Ok(sum.to_string())
}

fn parse(input: Box<dyn Read>) -> Vec<usize> {
    io::BufReader::new(input)
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect()
}
