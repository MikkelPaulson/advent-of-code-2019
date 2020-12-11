use std::io;
use std::io::prelude::*;

pub fn part1(input: Box<dyn Read>) -> Result<String, &'static str> {
    let mut sum = 0;
    for mass in parse(input) {
        sum += calc_fuel_simple(mass).unwrap();
    }
    Ok(sum.to_string())
}

pub fn part2(input: Box<dyn Read>) -> Result<String, &'static str> {
    let mut sum = 0;
    for mass in parse(input) {
        sum += calc_fuel(mass);
    }
    Ok(sum.to_string())
}

fn calc_fuel(mass: usize) -> usize {
    calc_fuel_simple(mass).map_or(0, |fuel_mass| fuel_mass + calc_fuel(fuel_mass))
}

fn calc_fuel_simple(mass: usize) -> Option<usize> {
    mass.div_euclid(3).checked_sub(2)
}

fn parse(input: Box<dyn Read>) -> Vec<usize> {
    io::BufReader::new(input)
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect()
}
