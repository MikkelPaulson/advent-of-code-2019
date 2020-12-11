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

pub fn part2(input: Box<dyn Read>) -> Result<String, &'static str> {
    let clean_intcode = Intcode::parse(input);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode = clean_intcode.clone();

            intcode.set(1, noun);
            intcode.set(2, verb);

            intcode.run();

            if intcode.get(0) == 19690720 {
                return Ok((100 * noun + verb).to_string());
            }
        }
    }
    Err("No matching result was found.")
}
