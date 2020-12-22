use crate::intcode::Intcode;
use std::io::prelude::*;
use std::str;

pub fn part1(input: Box<dyn Read>) -> Result<usize, &'static str> {
    let mut intcode = Intcode::parse(input);

    intcode.run();

    Ok(intcode.get(0) as usize)
}
