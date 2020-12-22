use crate::intcode::Intcode;
use std::io::prelude::*;
use std::str;

pub fn part1(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let mut intcode = Intcode::parse(input);

    intcode.input.push(1);
    intcode.run();

    println!("Output: {:?}", intcode.output);

    intcode.output.pop().ok_or("No output").map(|n| n as usize)
}

pub fn part2(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let mut intcode = Intcode::parse(input);

    intcode.input.push(5);
    intcode.run();

    println!("Output: {:?}", intcode.output);

    intcode.output.pop().ok_or("No output").map(|n| n as usize)
}
