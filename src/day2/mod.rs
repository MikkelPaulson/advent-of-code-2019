use crate::intcode::Intcode;
use std::io::prelude::*;
use std::str;

pub fn part1(input: Box<dyn Read>) -> Result<String, &'static str> {
    let mut intcode = Intcode::parse(input);

    intcode.set(1, 12);
    intcode.set(2, 2);

    intcode.run();

    Ok(intcode.get(0).to_string())
}
